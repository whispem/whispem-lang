/*
 * wvm.c -- standalone Whispem Virtual Machine (C)
 *
 * Reads a .whbc bytecode file produced by the Whispem compiler and
 * executes it.  No Rust, no cargo -- just C.
 *
 *   gcc -O2 -o wvm vm/wvm.c -lm
 *   ./wvm examples/hello.whbc
 *   ./wvm compiler/wsc.whbc compiler/wsc.wsp   # bootstrap
 *
 * Matches the Rust VM semantics exactly: same opcodes, same builtins,
 * same .whbc binary format (v3).
 */

#include <ctype.h>
#include <math.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ═══════════════════════════════════════════════════════════════════════════
 *  Forward declarations & limits
 * ═══════════════════════════════════════════════════════════════════════════ */

#define MAX_STACK   4096
#define MAX_FRAMES  256
#define MAX_CHUNKS  1024

/* ═══════════════════════════════════════════════════════════════════════════
 *  Opcodes   (must match src/opcode.rs)
 * ═══════════════════════════════════════════════════════════════════════════ */

enum {
    OP_PUSH_CONST       = 0x00,
    OP_PUSH_TRUE        = 0x01,
    OP_PUSH_FALSE       = 0x02,
    OP_PUSH_NONE        = 0x03,

    OP_LOAD             = 0x10,
    OP_STORE            = 0x11,
    OP_LOAD_GLOBAL      = 0x12,

    OP_ADD              = 0x20,
    OP_SUB              = 0x21,
    OP_MUL              = 0x22,
    OP_DIV              = 0x23,
    OP_MOD              = 0x24,
    OP_NEG              = 0x25,

    OP_EQ               = 0x30,
    OP_NEQ              = 0x31,
    OP_LT               = 0x32,
    OP_LTE              = 0x33,
    OP_GT               = 0x34,
    OP_GTE              = 0x35,
    OP_NOT              = 0x36,

    OP_JUMP             = 0x40,
    OP_JUMP_IF_FALSE    = 0x41,
    OP_JUMP_IF_TRUE     = 0x42,
    OP_PEEK_JUMP_FALSE  = 0x43,
    OP_PEEK_JUMP_TRUE   = 0x44,

    OP_CALL             = 0x50,
    OP_RETURN           = 0x51,
    OP_RETURN_NONE      = 0x52,

    OP_MAKE_ARRAY       = 0x60,
    OP_MAKE_DICT        = 0x61,
    OP_GET_INDEX        = 0x62,
    OP_SET_INDEX        = 0x63,

    OP_PRINT            = 0x70,
    OP_POP              = 0x71,

    OP_HALT             = 0xFF,
};

/* ═══════════════════════════════════════════════════════════════════════════
 *  Runtime error
 * ═══════════════════════════════════════════════════════════════════════════ */

static void die(const char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    fprintf(stderr, "wvm: ");
    vfprintf(stderr, fmt, ap);
    fprintf(stderr, "\n");
    va_end(ap);
    exit(1);
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  Ref-counted heap objects: strings, arrays, dicts
 * ═══════════════════════════════════════════════════════════════════════════ */

/* --- WStr (ref-counted string) ----------------------------------------- */

typedef struct WStr {
    int    rc;
    size_t len;     /* byte length */
    char   data[];  /* flexible array member, null-terminated */
} WStr;

static WStr *wstr_new(const char *s, size_t len) {
    WStr *w = malloc(sizeof(WStr) + len + 1);
    if (!w) die("out of memory");
    w->rc  = 1;
    w->len = len;
    memcpy(w->data, s, len);
    w->data[len] = '\0';
    return w;
}

static WStr *wstr_from_cstr(const char *s) {
    return wstr_new(s, strlen(s));
}

static void wstr_inc(WStr *w) { if (w) w->rc++; }
static void wstr_dec(WStr *w) { if (w && --w->rc <= 0) free(w); }

static WStr *wstr_cat(const WStr *a, const WStr *b) {
    size_t len = a->len + b->len;
    WStr *w = malloc(sizeof(WStr) + len + 1);
    if (!w) die("out of memory");
    w->rc  = 1;
    w->len = len;
    memcpy(w->data, a->data, a->len);
    memcpy(w->data + a->len, b->data, b->len);
    w->data[len] = '\0';
    return w;
}

/* --- Forward declarations for Value ------------------------------------ */

typedef struct Value Value;
typedef struct WArray WArray;
typedef struct WDict WDict;

/* --- Value ------------------------------------------------------------- */

enum ValueTag { V_NUM, V_BOOL, V_STR, V_ARRAY, V_DICT, V_NONE };

struct Value {
    enum ValueTag tag;
    union {
        double   num;
        bool     boolean;
        WStr    *str;
        WArray  *array;
        WDict   *dict;
    } as;
};

static Value val_num(double n)    { return (Value){.tag = V_NUM,   .as.num = n}; }
static Value val_bool(bool b)     { return (Value){.tag = V_BOOL,  .as.boolean = b}; }
static Value val_none(void)       { return (Value){.tag = V_NONE}; }
static Value val_str(WStr *s)     { wstr_inc(s); return (Value){.tag = V_STR, .as.str = s}; }
static Value val_str_own(WStr *s) { return (Value){.tag = V_STR, .as.str = s}; } /* takes ownership */

/* Forward */
static void  val_drop(Value *v);
static Value val_clone(const Value *v);
static void  warray_dec(WArray *a);
static void  wdict_dec(WDict *d);

static void val_drop(Value *v) {
    switch (v->tag) {
    case V_STR:   wstr_dec(v->as.str);     break;
    case V_ARRAY: warray_dec(v->as.array);  break;
    case V_DICT:  wdict_dec(v->as.dict);    break;
    default: break;
    }
    v->tag = V_NONE;
}

/* --- WArray (ref-counted array) ---------------------------------------- */

struct WArray {
    int     rc;
    size_t  len;
    size_t  cap;
    Value  *items;
};

static WArray *warray_new(size_t cap) {
    WArray *a = malloc(sizeof(WArray));
    if (!a) die("out of memory");
    a->rc    = 1;
    a->len   = 0;
    a->cap   = cap < 4 ? 4 : cap;
    a->items = malloc(sizeof(Value) * a->cap);
    if (!a->items) die("out of memory");
    return a;
}

static void warray_inc(WArray *a) { if (a) a->rc++; }

static void warray_dec(WArray *a) {
    if (!a) return;
    if (--a->rc <= 0) {
        for (size_t i = 0; i < a->len; i++) val_drop(&a->items[i]);
        free(a->items);
        free(a);
    }
}

static void warray_push(WArray *a, Value v) {
    if (a->len >= a->cap) {
        a->cap *= 2;
        a->items = realloc(a->items, sizeof(Value) * a->cap);
        if (!a->items) die("out of memory");
    }
    a->items[a->len++] = v;
}

/* Deep-clone an array (new rc=1, all items cloned). */
static WArray *warray_clone(const WArray *src) {
    WArray *a = warray_new(src->len);
    a->len = src->len;
    for (size_t i = 0; i < src->len; i++)
        a->items[i] = val_clone(&src->items[i]);
    return a;
}

/* Copy-on-write: return a mutable array with rc==1. */
static WArray *warray_cow(WArray *a) {
    if (a->rc == 1) return a;
    WArray *copy = warray_clone(a);
    warray_dec(a);
    return copy;
}

/* --- WDict (ref-counted dict as parallel arrays) ----------------------- */

struct WDict {
    int     rc;
    size_t  len;
    size_t  cap;
    WStr  **keys;
    Value  *vals;
};

static WDict *wdict_new(size_t cap) {
    WDict *d = malloc(sizeof(WDict));
    if (!d) die("out of memory");
    d->rc   = 1;
    d->len  = 0;
    d->cap  = cap < 4 ? 4 : cap;
    d->keys = malloc(sizeof(WStr*) * d->cap);
    d->vals = malloc(sizeof(Value) * d->cap);
    if (!d->keys || !d->vals) die("out of memory");
    return d;
}

static void wdict_inc(WDict *d) { if (d) d->rc++; }

static void wdict_dec(WDict *d) {
    if (!d) return;
    if (--d->rc <= 0) {
        for (size_t i = 0; i < d->len; i++) {
            wstr_dec(d->keys[i]);
            val_drop(&d->vals[i]);
        }
        free(d->keys);
        free(d->vals);
        free(d);
    }
}

/* Find key; returns index or (size_t)-1. */
static size_t wdict_find(const WDict *d, const char *key, size_t keylen) {
    for (size_t i = 0; i < d->len; i++)
        if (d->keys[i]->len == keylen && memcmp(d->keys[i]->data, key, keylen) == 0)
            return i;
    return (size_t)-1;
}

/* Insert or update.  Caller must have COW'd if needed. */
static void wdict_set(WDict *d, WStr *key, Value val) {
    size_t idx = wdict_find(d, key->data, key->len);
    if (idx != (size_t)-1) {
        wstr_dec(key);
        val_drop(&d->vals[idx]);
        d->vals[idx] = val;
        return;
    }
    if (d->len >= d->cap) {
        d->cap *= 2;
        d->keys = realloc(d->keys, sizeof(WStr*) * d->cap);
        d->vals = realloc(d->vals, sizeof(Value) * d->cap);
        if (!d->keys || !d->vals) die("out of memory");
    }
    d->keys[d->len] = key;
    d->vals[d->len] = val;
    d->len++;
}

/* Get value by key; returns NULL if not found. */
static const Value *wdict_get(const WDict *d, const char *key, size_t keylen) {
    size_t idx = wdict_find(d, key, keylen);
    if (idx == (size_t)-1) return NULL;
    return &d->vals[idx];
}

/* Deep-clone a dict. */
static WDict *wdict_clone(const WDict *src) {
    WDict *d = wdict_new(src->len);
    d->len = src->len;
    for (size_t i = 0; i < src->len; i++) {
        d->keys[i] = src->keys[i]; wstr_inc(d->keys[i]);
        d->vals[i] = val_clone(&src->vals[i]);
    }
    return d;
}

/* Copy-on-write. */
static WDict *wdict_cow(WDict *d) {
    if (d->rc == 1) return d;
    WDict *copy = wdict_clone(d);
    wdict_dec(d);
    return copy;
}

/* --- val_clone / val_str_owned helpers --------------------------------- */

static Value val_clone(const Value *v) {
    Value r = *v;
    switch (v->tag) {
    case V_STR:   wstr_inc(r.as.str);     break;
    case V_ARRAY: warray_inc(r.as.array);  break;
    case V_DICT:  wdict_inc(r.as.dict);    break;
    default: break;
    }
    return r;
}

static Value val_array_own(WArray *a) { return (Value){.tag = V_ARRAY, .as.array = a}; }
static Value val_dict_own(WDict *d)  { return (Value){.tag = V_DICT, .as.dict = d}; }

/* ═══════════════════════════════════════════════════════════════════════════
 *  is_truthy  (matches Rust Value::is_truthy)
 * ═══════════════════════════════════════════════════════════════════════════ */

static bool is_truthy(const Value *v) {
    switch (v->tag) {
    case V_BOOL:  return v->as.boolean;
    case V_NUM:   return v->as.num != 0.0;
    case V_STR:   return v->as.str->len > 0;
    case V_ARRAY: return v->as.array->len > 0;
    case V_DICT:  return v->as.dict->len > 0;
    case V_NONE:  return false;
    }
    return false;
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  Value formatting  (matches Rust Value::format)
 * ═══════════════════════════════════════════════════════════════════════════ */

/* Format a number to a string buffer (caller owns).  Matches Rust:
 *   if fract==0 and abs < 1e15 => print as integer
 *   else => print as float
 */
static char *fmt_number(double n) {
    char buf[64];
    if (n == (double)(int64_t)n && fabs(n) < 1e15) {
        snprintf(buf, sizeof(buf), "%lld", (long long)(int64_t)n);
    } else {
        /* Mimic Rust Display: shortest decimal that round-trips to the same f64.
         * Try increasing precision until parsing back gives the same bits. */
        for (int prec = 1; prec <= 17; prec++) {
            snprintf(buf, sizeof(buf), "%.*g", prec, n);
            char *end;
            double back = strtod(buf, &end);
            if (back == n) break;
        }
    }
    return strdup(buf);
}

/* Forward declaration. */
static char *val_format(const Value *v);

/* Comparison function for sorting dict keys and array-of-strings. */
static int cmp_str_ptrs(const void *a, const void *b) {
    const char *sa = *(const char **)a;
    const char *sb = *(const char **)b;
    return strcmp(sa, sb);
}

static char *val_format(const Value *v) {
    switch (v->tag) {
    case V_NUM:   return fmt_number(v->as.num);
    case V_BOOL:  return strdup(v->as.boolean ? "true" : "false");
    case V_STR:   return strdup(v->as.str->data);
    case V_NONE:  return strdup("");
    case V_ARRAY: {
        /* Build "[e1, e2, ...]" */
        size_t total = 3; /* "[]" + NUL */
        size_t n = v->as.array->len;
        char **parts = malloc(sizeof(char*) * n);
        for (size_t i = 0; i < n; i++) {
            parts[i] = val_format(&v->as.array->items[i]);
            total += strlen(parts[i]) + 2; /* ", " */
        }
        char *out = malloc(total);
        strcpy(out, "[");
        for (size_t i = 0; i < n; i++) {
            if (i > 0) strcat(out, ", ");
            strcat(out, parts[i]);
            free(parts[i]);
        }
        strcat(out, "]");
        free(parts);
        return out;
    }
    case V_DICT: {
        /* Build '{"k": v, ...}' with keys sorted. */
        size_t n = v->as.dict->len;
        /* Build formatted pairs: "\"key\": value" */
        char **entries = malloc(sizeof(char*) * n);
        for (size_t i = 0; i < n; i++) {
            char *fv = val_format(&v->as.dict->vals[i]);
            size_t elen = 2 + v->as.dict->keys[i]->len + 2 + strlen(fv) + 1;
            entries[i] = malloc(elen + 4);
            sprintf(entries[i], "\"%s\": %s", v->as.dict->keys[i]->data, fv);
            free(fv);
        }
        /* Sort entries lexicographically (matches Rust sort on formatted pairs). */
        qsort(entries, n, sizeof(char*), cmp_str_ptrs);
        size_t total = 3;
        for (size_t i = 0; i < n; i++) total += strlen(entries[i]) + 2;
        char *out = malloc(total);
        strcpy(out, "{");
        for (size_t i = 0; i < n; i++) {
            if (i > 0) strcat(out, ", ");
            strcat(out, entries[i]);
            free(entries[i]);
        }
        strcat(out, "}");
        free(entries);
        return out;
    }
    }
    return strdup("");
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  type_name helper
 * ═══════════════════════════════════════════════════════════════════════════ */

static const char *type_name(const Value *v) {
    switch (v->tag) {
    case V_NUM:   return "number";
    case V_BOOL:  return "bool";
    case V_STR:   return "string";
    case V_ARRAY: return "array";
    case V_DICT:  return "dict";
    case V_NONE:  return "none";
    }
    return "unknown";
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  Dict key conversion  (matches Rust to_dict_key)
 * ═══════════════════════════════════════════════════════════════════════════ */

static WStr *to_dict_key(const Value *v) {
    switch (v->tag) {
    case V_STR:
        wstr_inc(v->as.str);
        return v->as.str;
    case V_NUM: {
        char *s = fmt_number(v->as.num);
        WStr *w = wstr_from_cstr(s);
        free(s);
        return w;
    }
    default:
        die("dict key must be string or number, got %s", type_name(v));
        return NULL;
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  Bytecode chunks
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef struct {
    char   *name;
    int     param_count;
    Value  *constants;
    int     const_count;
    uint8_t *code;
    uint32_t code_len;
    uint32_t *lines;
    uint32_t lines_len;
} Chunk;

/* ═══════════════════════════════════════════════════════════════════════════
 *  .whbc deserialiser  (matches src/chunk.rs)
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef struct {
    const uint8_t *data;
    size_t         len;
    size_t         pos;
} Reader;

static void need(Reader *r, size_t n) {
    if (r->pos + n > r->len)
        die("unexpected end of bytecode at offset %zu (need %zu bytes)", r->pos, n);
}

static uint8_t read_u8(Reader *r) {
    need(r, 1);
    return r->data[r->pos++];
}

static uint16_t read_u16(Reader *r) {
    need(r, 2);
    uint16_t v = ((uint16_t)r->data[r->pos] << 8) | r->data[r->pos + 1];
    r->pos += 2;
    return v;
}

static uint32_t read_u32(Reader *r) {
    need(r, 4);
    uint32_t v = ((uint32_t)r->data[r->pos]   << 24)
               | ((uint32_t)r->data[r->pos+1] << 16)
               | ((uint32_t)r->data[r->pos+2] << 8)
               |  (uint32_t)r->data[r->pos+3];
    r->pos += 4;
    return v;
}

static double read_f64(Reader *r) {
    need(r, 8);
    uint64_t bits = 0;
    for (int i = 0; i < 8; i++)
        bits = (bits << 8) | r->data[r->pos + i];
    r->pos += 8;
    double d;
    memcpy(&d, &bits, 8);
    return d;
}

static Value read_const(Reader *r) {
    uint8_t tag = read_u8(r);
    switch (tag) {
    case 0:  return val_num(read_f64(r));
    case 1:  return val_bool(read_u8(r) != 0);
    case 2: {
        uint16_t slen = read_u16(r);
        need(r, slen);
        WStr *s = wstr_new((const char *)r->data + r->pos, slen);
        r->pos += slen;
        return val_str_own(s);
    }
    case 3:  return val_none();
    default: die("unknown constant tag %u", tag); return val_none();
    }
}

static Chunk read_chunk(Reader *r) {
    Chunk c = {0};
    /* name */
    uint16_t nlen = read_u16(r);
    need(r, nlen);
    c.name = malloc(nlen + 1);
    memcpy(c.name, r->data + r->pos, nlen);
    c.name[nlen] = '\0';
    r->pos += nlen;
    /* param_count */
    c.param_count = read_u8(r);
    /* constants */
    c.const_count = read_u8(r);
    c.constants = malloc(sizeof(Value) * (c.const_count ? c.const_count : 1));
    for (int i = 0; i < c.const_count; i++)
        c.constants[i] = read_const(r);
    /* code */
    c.code_len = read_u32(r);
    need(r, c.code_len);
    c.code = malloc(c.code_len);
    memcpy(c.code, r->data + r->pos, c.code_len);
    r->pos += c.code_len;
    /* lines */
    c.lines_len = read_u32(r);
    need(r, c.lines_len * 4);
    c.lines = malloc(sizeof(uint32_t) * c.lines_len);
    for (uint32_t i = 0; i < c.lines_len; i++)
        c.lines[i] = read_u32(r);
    return c;
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  Program (all chunks)
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef struct {
    Chunk  *chunks;
    int     chunk_count;
} Program;

static Program load_program(const uint8_t *data, size_t len) {
    Reader r = { .data = data, .len = len, .pos = 0 };

    /* Magic */
    if (len < 5) die("file too short");
    if (memcmp(r.data, "WHBC", 4) != 0) die("bad magic bytes (not a .whbc file)");
    r.pos = 4;

    /* Version */
    uint8_t ver = read_u8(&r);
    if (ver != 3) die("version mismatch: expected 3, got %u", ver);

    /* fn_count */
    uint16_t fn_count = read_u16(&r);
    if (fn_count == 0) die("bytecode contains no chunks");

    Program p;
    p.chunk_count = fn_count;
    p.chunks = malloc(sizeof(Chunk) * fn_count);
    for (int i = 0; i < fn_count; i++)
        p.chunks[i] = read_chunk(&r);

    return p;
}

/* Find a chunk by name; returns index or -1. */
static int find_chunk(const Program *p, const char *name) {
    for (int i = 0; i < p->chunk_count; i++)
        if (strcmp(p->chunks[i].name, name) == 0) return i;
    return -1;
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  Call frame
 * ═══════════════════════════════════════════════════════════════════════════ */

/* Local variable: simple dynamic array of (name, value) pairs. */

typedef struct {
    WStr  *name;
    Value  val;
} Local;

typedef struct {
    int      chunk_idx;
    uint32_t ip;
    Local   *locals;
    int      local_count;
    int      local_cap;
} CallFrame;

static void frame_init(CallFrame *f, int chunk_idx) {
    f->chunk_idx   = chunk_idx;
    f->ip          = 0;
    f->local_count = 0;
    f->local_cap   = 16;
    f->locals      = malloc(sizeof(Local) * f->local_cap);
}

static void frame_free(CallFrame *f) {
    for (int i = 0; i < f->local_count; i++) {
        wstr_dec(f->locals[i].name);
        val_drop(&f->locals[i].val);
    }
    free(f->locals);
}

/* Look up a local variable. Returns NULL if not found. */
static Value *frame_get(CallFrame *f, const char *name, size_t nlen) {
    for (int i = 0; i < f->local_count; i++)
        if (f->locals[i].name->len == nlen &&
            memcmp(f->locals[i].name->data, name, nlen) == 0)
            return &f->locals[i].val;
    return NULL;
}

/* Set a local variable (insert or update). */
static void frame_set(CallFrame *f, WStr *name, Value val) {
    Value *existing = frame_get(f, name->data, name->len);
    if (existing) {
        val_drop(existing);
        *existing = val;
        wstr_dec(name);
        return;
    }
    if (f->local_count >= f->local_cap) {
        f->local_cap *= 2;
        f->locals = realloc(f->locals, sizeof(Local) * f->local_cap);
    }
    f->locals[f->local_count].name = name;
    f->locals[f->local_count].val  = val;
    f->local_count++;
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  Global variables (same structure as locals)
 * ═══════════════════════════════════════════════════════════════════════════ */

static Local  *g_globals     = NULL;
static int     g_global_count = 0;
static int     g_global_cap   = 0;

static void globals_init(void) {
    g_global_cap   = 64;
    g_global_count = 0;
    g_globals      = malloc(sizeof(Local) * g_global_cap);
}

static Value *global_get(const char *name, size_t nlen) {
    for (int i = 0; i < g_global_count; i++)
        if (g_globals[i].name->len == nlen &&
            memcmp(g_globals[i].name->data, name, nlen) == 0)
            return &g_globals[i].val;
    return NULL;
}

static void global_set(WStr *name, Value val) {
    Value *existing = global_get(name->data, name->len);
    if (existing) {
        val_drop(existing);
        *existing = val;
        wstr_dec(name);
        return;
    }
    if (g_global_count >= g_global_cap) {
        g_global_cap *= 2;
        g_globals = realloc(g_globals, sizeof(Local) * g_global_cap);
    }
    g_globals[g_global_count].name = name;
    g_globals[g_global_count].val  = val;
    g_global_count++;
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  UTF-8 helpers (for char_at, substr, length on strings, ord)
 * ═══════════════════════════════════════════════════════════════════════════ */

/* Count the number of Unicode codepoints in a UTF-8 string. */
static size_t utf8_char_count(const char *s, size_t len) {
    size_t count = 0;
    size_t i = 0;
    while (i < len) {
        unsigned char c = (unsigned char)s[i];
        if      (c < 0x80) i += 1;
        else if (c < 0xE0) i += 2;
        else if (c < 0xF0) i += 3;
        else               i += 4;
        count++;
    }
    return count;
}

/* Get the byte offset and byte length of the n-th codepoint. */
static bool utf8_nth(const char *s, size_t slen, size_t n,
                     size_t *out_offset, size_t *out_cplen)
{
    size_t i = 0;
    size_t count = 0;
    while (i < slen) {
        if (count == n) {
            *out_offset = i;
            unsigned char c = (unsigned char)s[i];
            if      (c < 0x80) *out_cplen = 1;
            else if (c < 0xE0) *out_cplen = 2;
            else if (c < 0xF0) *out_cplen = 3;
            else               *out_cplen = 4;
            return true;
        }
        unsigned char c = (unsigned char)s[i];
        if      (c < 0x80) i += 1;
        else if (c < 0xE0) i += 2;
        else if (c < 0xF0) i += 3;
        else               i += 4;
        count++;
    }
    return false;
}

/* Get Unicode codepoint at byte offset. */
static uint32_t utf8_decode(const char *s, size_t *advance) {
    unsigned char c = (unsigned char)s[0];
    if (c < 0x80)       { *advance = 1; return c; }
    else if (c < 0xE0)  { *advance = 2; return ((c & 0x1F) << 6)  | (s[1] & 0x3F); }
    else if (c < 0xF0)  { *advance = 3; return ((c & 0x0F) << 12) | ((s[1] & 0x3F) << 6) | (s[2] & 0x3F); }
    else                 { *advance = 4; return ((c & 0x07) << 18) | ((s[1] & 0x3F) << 12) | ((s[2] & 0x3F) << 6) | (s[3] & 0x3F); }
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  VM state and execution
 * ═══════════════════════════════════════════════════════════════════════════ */

static Program    g_prog;
static Value      g_stack[MAX_STACK];
static int        g_sp = 0;              /* stack pointer */
static CallFrame  g_frames[MAX_FRAMES];
static int        g_fp = 0;              /* frame pointer */
static int        g_argc = 0;
static char     **g_argv = NULL;

/* Stack operations */
static void push(Value v) {
    if (g_sp >= MAX_STACK) die("stack overflow");
    g_stack[g_sp++] = v;
}

static Value pop(void) {
    if (g_sp <= 0) die("stack underflow");
    return g_stack[--g_sp];
}

static Value *peek(void) {
    if (g_sp <= 0) die("stack underflow (peek)");
    return &g_stack[g_sp - 1];
}

/* Frame accessors */
static CallFrame *frame(void) { return &g_frames[g_fp - 1]; }
static Chunk     *chunk(void) { return &g_prog.chunks[frame()->chunk_idx]; }

static uint8_t frame_read_byte(void) {
    CallFrame *f = frame();
    Chunk *c = &g_prog.chunks[f->chunk_idx];
    return c->code[f->ip++];
}

static uint16_t frame_read_u16(void) {
    CallFrame *f = frame();
    Chunk *c = &g_prog.chunks[f->chunk_idx];
    uint16_t hi = c->code[f->ip];
    uint16_t lo = c->code[f->ip + 1];
    f->ip += 2;
    return (hi << 8) | lo;
}

static const Value *const_val(uint8_t idx) {
    return &chunk()->constants[idx];
}

static const char *const_str(uint8_t idx) {
    const Value *v = const_val(idx);
    if (v->tag != V_STR) die("expected string constant at index %d", idx);
    return v->as.str->data;
}

static size_t const_str_len(uint8_t idx) {
    const Value *v = const_val(idx);
    if (v->tag != V_STR) die("expected string constant at index %d", idx);
    return v->as.str->len;
}

static uint32_t current_line(void) {
    CallFrame *f = frame();
    uint32_t ip = f->ip > 0 ? f->ip - 1 : 0;
    Chunk *c = &g_prog.chunks[f->chunk_idx];
    if (ip < c->lines_len) return c->lines[ip];
    return 0;
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  Variable lookup  (matches Rust lookup_local / store)
 * ═══════════════════════════════════════════════════════════════════════════ */

static Value lookup_var(const char *name, size_t nlen) {
    /* Check current frame locals first. */
    Value *v = frame_get(frame(), name, nlen);
    if (v) return val_clone(v);
    /* Then globals. */
    v = global_get(name, nlen);
    if (v) return val_clone(v);
    die("line %u: undefined variable '%.*s'", current_line(), (int)nlen, name);
    return val_none();
}

static void store_var(const char *name, size_t nlen, Value val) {
    if (g_fp > 1) {
        /* Inside a function: store as local. */
        WStr *key = wstr_new(name, nlen);
        frame_set(frame(), key, val);
    } else {
        /* Top-level: store as global. */
        WStr *key = wstr_new(name, nlen);
        global_set(key, val);
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  Built-in functions  (matches Rust call_builtin)
 * ═══════════════════════════════════════════════════════════════════════════ */

static bool call_builtin(const char *name, int argc, Value *args, Value *result) {

    /* ── length ──────────────────────────────────────────────────────── */
    if (strcmp(name, "length") == 0) {
        if (argc != 1) die("length() takes 1 argument, got %d", argc);
        switch (args[0].tag) {
        case V_ARRAY: *result = val_num((double)args[0].as.array->len); break;
        case V_STR:   *result = val_num((double)utf8_char_count(args[0].as.str->data, args[0].as.str->len)); break;
        case V_DICT:  *result = val_num((double)args[0].as.dict->len); break;
        default: die("length() expects array, string, or dict, got %s", type_name(&args[0]));
        }
        return true;
    }

    /* ── push ────────────────────────────────────────────────────────── */
    if (strcmp(name, "push") == 0) {
        if (argc != 2) die("push() takes 2 arguments, got %d", argc);
        if (args[0].tag != V_ARRAY) die("push() expects array, got %s", type_name(&args[0]));
        /* Clone the array ref (bump rc), then COW on our copy. */
        WArray *a = args[0].as.array;
        warray_inc(a);
        a = warray_cow(a);
        warray_push(a, val_clone(&args[1]));
        *result = val_array_own(a);
        return true;
    }

    /* ── pop ─────────────────────────────────────────────────────────── */
    if (strcmp(name, "pop") == 0) {
        if (argc != 1) die("pop() takes 1 argument, got %d", argc);
        if (args[0].tag != V_ARRAY) die("pop() expects array, got %s", type_name(&args[0]));
        WArray *a = args[0].as.array;
        if (a->len == 0) die("pop() on empty array");
        /* Clone the last element as the result (Rust pop returns the element). */
        *result = val_clone(&a->items[a->len - 1]);
        return true;
    }

    /* ── reverse ─────────────────────────────────────────────────────── */
    if (strcmp(name, "reverse") == 0) {
        if (argc != 1) die("reverse() takes 1 argument, got %d", argc);
        if (args[0].tag != V_ARRAY) die("reverse() expects array, got %s", type_name(&args[0]));
        WArray *a = args[0].as.array;
        warray_inc(a);
        a = warray_cow(a);
        if (a->len > 1) {
            for (size_t i = 0, j = a->len - 1; i < j; i++, j--) {
                Value tmp = a->items[i];
                a->items[i] = a->items[j];
                a->items[j] = tmp;
            }
        }
        *result = val_array_own(a);
        return true;
    }

    /* ── slice ───────────────────────────────────────────────────────── */
    if (strcmp(name, "slice") == 0) {
        if (argc != 3) die("slice() takes 3 arguments, got %d", argc);
        if (args[0].tag != V_ARRAY) die("slice() expects array, got %s", type_name(&args[0]));
        size_t start = (size_t)args[1].as.num;
        size_t end   = (size_t)args[2].as.num;
        WArray *src  = args[0].as.array;
        if (start > end) die("invalid slice: start %zu > end %zu", start, end);
        if (end > src->len) die("slice out of bounds: end %zu > length %zu", end, src->len);
        WArray *a = warray_new(end - start);
        for (size_t i = start; i < end; i++)
            warray_push(a, val_clone(&src->items[i]));
        *result = val_array_own(a);
        return true;
    }

    /* ── range ───────────────────────────────────────────────────────── */
    if (strcmp(name, "range") == 0) {
        if (argc != 2) die("range() takes 2 arguments, got %d", argc);
        int64_t start = (int64_t)args[0].as.num;
        int64_t end   = (int64_t)args[1].as.num;
        size_t n = (end > start) ? (size_t)(end - start) : 0;
        WArray *a = warray_new(n);
        for (int64_t i = start; i < end; i++)
            warray_push(a, val_num((double)i));
        *result = val_array_own(a);
        return true;
    }

    /* ── keys ────────────────────────────────────────────────────────── */
    if (strcmp(name, "keys") == 0) {
        if (argc != 1) die("keys() takes 1 argument, got %d", argc);
        if (args[0].tag != V_DICT) die("keys() expects dict, got %s", type_name(&args[0]));
        WDict *d = args[0].as.dict;
        /* Collect key strings, sort them, return as array. */
        WArray *a = warray_new(d->len);
        /* Build a sortable array of indices. */
        size_t *indices = malloc(sizeof(size_t) * d->len);
        for (size_t i = 0; i < d->len; i++) indices[i] = i;
        /* Sort indices by key string. */
        /* Simple insertion sort (dicts are small). */
        for (size_t i = 1; i < d->len; i++) {
            size_t tmp = indices[i];
            size_t j = i;
            while (j > 0 && strcmp(d->keys[indices[j-1]]->data, d->keys[tmp]->data) > 0) {
                indices[j] = indices[j-1];
                j--;
            }
            indices[j] = tmp;
        }
        for (size_t i = 0; i < d->len; i++)
            warray_push(a, val_str(d->keys[indices[i]]));
        free(indices);
        *result = val_array_own(a);
        return true;
    }

    /* ── values ──────────────────────────────────────────────────────── */
    if (strcmp(name, "values") == 0) {
        if (argc != 1) die("values() takes 1 argument, got %d", argc);
        if (args[0].tag != V_DICT) die("values() expects dict, got %s", type_name(&args[0]));
        WDict *d = args[0].as.dict;
        /* Sort by key, return values in that order. */
        size_t *indices = malloc(sizeof(size_t) * d->len);
        for (size_t i = 0; i < d->len; i++) indices[i] = i;
        for (size_t i = 1; i < d->len; i++) {
            size_t tmp = indices[i];
            size_t j = i;
            while (j > 0 && strcmp(d->keys[indices[j-1]]->data, d->keys[tmp]->data) > 0) {
                indices[j] = indices[j-1];
                j--;
            }
            indices[j] = tmp;
        }
        WArray *a = warray_new(d->len);
        for (size_t i = 0; i < d->len; i++)
            warray_push(a, val_clone(&d->vals[indices[i]]));
        free(indices);
        *result = val_array_own(a);
        return true;
    }

    /* ── has_key ─────────────────────────────────────────────────────── */
    if (strcmp(name, "has_key") == 0) {
        if (argc != 2) die("has_key() takes 2 arguments, got %d", argc);
        if (args[0].tag != V_DICT) die("has_key() expects dict, got %s", type_name(&args[0]));
        WStr *k = to_dict_key(&args[1]);
        bool found = wdict_find(args[0].as.dict, k->data, k->len) != (size_t)-1;
        wstr_dec(k);
        *result = val_bool(found);
        return true;
    }

    /* ── char_at ─────────────────────────────────────────────────────── */
    if (strcmp(name, "char_at") == 0) {
        if (argc != 2) die("char_at() takes 2 arguments, got %d", argc);
        if (args[0].tag != V_STR || args[1].tag != V_NUM)
            die("char_at() expects (string, number)");
        size_t idx = (size_t)args[1].as.num;
        size_t off, cplen;
        if (!utf8_nth(args[0].as.str->data, args[0].as.str->len, idx, &off, &cplen))
            die("char_at: index %zu out of bounds", idx);
        *result = val_str_own(wstr_new(args[0].as.str->data + off, cplen));
        return true;
    }

    /* ── substr ──────────────────────────────────────────────────────── */
    if (strcmp(name, "substr") == 0) {
        if (argc != 3) die("substr() takes 3 arguments, got %d", argc);
        if (args[0].tag != V_STR) die("substr() expects string, got %s", type_name(&args[0]));
        size_t start = (size_t)args[1].as.num;
        size_t len   = (size_t)args[2].as.num;
        const char *s = args[0].as.str->data;
        size_t slen   = args[0].as.str->len;
        size_t char_count = utf8_char_count(s, slen);
        /* Clamp like Rust: st.min(chars.len())..end.min(chars.len()) */
        if (start > char_count) start = char_count;
        size_t end = start + len;
        if (end > char_count) end = char_count;
        /* Find byte offsets. */
        size_t byte_start = 0, byte_end = 0;
        size_t ci = 0, bi = 0;
        while (bi < slen && ci < end) {
            if (ci == start) byte_start = bi;
            unsigned char c = (unsigned char)s[bi];
            if      (c < 0x80) bi += 1;
            else if (c < 0xE0) bi += 2;
            else if (c < 0xF0) bi += 3;
            else               bi += 4;
            ci++;
        }
        if (ci == start) byte_start = bi; /* start == char_count */
        byte_end = bi;
        *result = val_str_own(wstr_new(s + byte_start, byte_end - byte_start));
        return true;
    }

    /* ── ord ─────────────────────────────────────────────────────────── */
    if (strcmp(name, "ord") == 0) {
        if (argc != 1) die("ord() takes 1 argument, got %d", argc);
        if (args[0].tag != V_STR) die("ord() expects string, got %s", type_name(&args[0]));
        if (args[0].as.str->len == 0) die("ord() on empty string");
        size_t adv;
        uint32_t cp = utf8_decode(args[0].as.str->data, &adv);
        *result = val_num((double)cp);
        return true;
    }

    /* ── num_to_str ──────────────────────────────────────────────────── */
    if (strcmp(name, "num_to_str") == 0) {
        if (argc != 1) die("num_to_str() takes 1 argument, got %d", argc);
        if (args[0].tag != V_NUM) die("num_to_str() expects number, got %s", type_name(&args[0]));
        char *s = fmt_number(args[0].as.num);
        *result = val_str_own(wstr_from_cstr(s));
        free(s);
        return true;
    }

    /* ── str_to_num ──────────────────────────────────────────────────── */
    if (strcmp(name, "str_to_num") == 0) {
        if (argc != 1) die("str_to_num() takes 1 argument, got %d", argc);
        if (args[0].tag != V_STR) die("str_to_num() expects string, got %s", type_name(&args[0]));
        /* Trim leading/trailing whitespace, then parse. */
        const char *s = args[0].as.str->data;
        while (*s && isspace((unsigned char)*s)) s++;
        char *end;
        double n = strtod(s, &end);
        /* Skip trailing whitespace */
        while (*end && isspace((unsigned char)*end)) end++;
        if (*end != '\0') die("str_to_num: cannot parse \"%s\"", args[0].as.str->data);
        *result = val_num(n);
        return true;
    }

    /* ── input ───────────────────────────────────────────────────────── */
    if (strcmp(name, "input") == 0) {
        if (argc > 1) die("input() takes 0 or 1 arguments, got %d", argc);
        if (argc == 1) {
            if (args[0].tag != V_STR) die("input() expects string prompt, got %s", type_name(&args[0]));
            if (args[0].as.str->len > 0) {
                printf("%s", args[0].as.str->data);
                fflush(stdout);
            }
        }
        char buf[4096];
        if (!fgets(buf, sizeof(buf), stdin)) buf[0] = '\0';
        /* Trim trailing \n and \r */
        size_t l = strlen(buf);
        while (l > 0 && (buf[l-1] == '\n' || buf[l-1] == '\r')) l--;
        buf[l] = '\0';
        *result = val_str_own(wstr_from_cstr(buf));
        return true;
    }

    /* ── read_file ───────────────────────────────────────────────────── */
    if (strcmp(name, "read_file") == 0) {
        if (argc != 1) die("read_file() takes 1 argument, got %d", argc);
        if (args[0].tag != V_STR) die("read_file() expects string, got %s", type_name(&args[0]));
        FILE *fp = fopen(args[0].as.str->data, "rb");
        if (!fp) die("cannot open file '%s' for reading", args[0].as.str->data);
        fseek(fp, 0, SEEK_END);
        long sz = ftell(fp);
        fseek(fp, 0, SEEK_SET);
        char *contents = malloc(sz + 1);
        if ((long)fread(contents, 1, sz, fp) != sz)
            die("failed to read file '%s'", args[0].as.str->data);
        contents[sz] = '\0';
        fclose(fp);
        *result = val_str_own(wstr_new(contents, sz));
        free(contents);
        return true;
    }

    /* ── write_file ──────────────────────────────────────────────────── */
    if (strcmp(name, "write_file") == 0) {
        if (argc != 2) die("write_file() takes 2 arguments, got %d", argc);
        if (args[0].tag != V_STR) die("write_file() expects string path, got %s", type_name(&args[0]));
        char *content = val_format(&args[1]);
        FILE *fp = fopen(args[0].as.str->data, "wb");
        if (!fp) die("cannot open file '%s' for writing", args[0].as.str->data);
        fwrite(content, 1, strlen(content), fp);
        fclose(fp);
        free(content);
        *result = val_none();
        return true;
    }

    /* ── args ────────────────────────────────────────────────────────── */
    if (strcmp(name, "args") == 0) {
        if (argc != 0) die("args() takes 0 arguments, got %d", argc);
        WArray *a = warray_new(g_argc);
        for (int i = 0; i < g_argc; i++)
            warray_push(a, val_str_own(wstr_from_cstr(g_argv[i])));
        *result = val_array_own(a);
        return true;
    }

    /* ── num_to_hex ──────────────────────────────────────────────────── */
    if (strcmp(name, "num_to_hex") == 0) {
        if (argc != 1) die("num_to_hex() takes 1 argument, got %d", argc);
        if (args[0].tag != V_NUM) die("num_to_hex() expects number, got %s", type_name(&args[0]));
        uint64_t bits;
        double n = args[0].as.num;
        memcpy(&bits, &n, 8);
        char buf[17];
        snprintf(buf, sizeof(buf), "%016llx", (unsigned long long)bits);
        *result = val_str_own(wstr_from_cstr(buf));
        return true;
    }

    /* ── write_hex ───────────────────────────────────────────────────── */
    if (strcmp(name, "write_hex") == 0) {
        if (argc != 2) die("write_hex() takes 2 arguments, got %d", argc);
        if (args[0].tag != V_STR) die("write_hex() expects string path, got %s", type_name(&args[0]));
        if (args[1].tag != V_STR) die("write_hex() expects string hex, got %s", type_name(&args[1]));
        const char *hex = args[1].as.str->data;
        size_t hexlen = args[1].as.str->len;
        size_t nbytes = hexlen / 2;
        uint8_t *bytes = malloc(nbytes);
        for (size_t i = 0; i < nbytes; i++) {
            char tmp[3] = { hex[i*2], hex[i*2+1], '\0' };
            bytes[i] = (uint8_t)strtoul(tmp, NULL, 16);
        }
        FILE *fp = fopen(args[0].as.str->data, "wb");
        if (!fp) die("cannot open file '%s' for writing", args[0].as.str->data);
        fwrite(bytes, 1, nbytes, fp);
        fclose(fp);
        free(bytes);
        *result = val_none();
        return true;
    }

    return false;  /* not a builtin */
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  Value equality  (matches Rust eq)
 * ═══════════════════════════════════════════════════════════════════════════ */

static bool val_eq(const Value *a, const Value *b) {
    if (a->tag != b->tag) return false;
    switch (a->tag) {
    case V_NUM:   return a->as.num == b->as.num;
    case V_BOOL:  return a->as.boolean == b->as.boolean;
    case V_STR:   return a->as.str->len == b->as.str->len &&
                         memcmp(a->as.str->data, b->as.str->data, a->as.str->len) == 0;
    case V_NONE:  return true;
    default:      return false;
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  Main execution loop
 * ═══════════════════════════════════════════════════════════════════════════ */

static void execute(void) {
    for (;;) {
        uint8_t op = frame_read_byte();

        switch (op) {

        /* ── Constants ─────────────────────────────────────────────── */
        case OP_PUSH_CONST: {
            uint8_t idx = frame_read_byte();
            push(val_clone(const_val(idx)));
            break;
        }
        case OP_PUSH_TRUE:  push(val_bool(true));  break;
        case OP_PUSH_FALSE: push(val_bool(false)); break;
        case OP_PUSH_NONE:  push(val_none());      break;

        /* ── Variables ─────────────────────────────────────────────── */
        case OP_LOAD: {
            uint8_t idx = frame_read_byte();
            const char *name = const_str(idx);
            size_t nlen = const_str_len(idx);
            push(lookup_var(name, nlen));
            break;
        }
        case OP_LOAD_GLOBAL: {
            uint8_t idx = frame_read_byte();
            const char *name = const_str(idx);
            size_t nlen = const_str_len(idx);
            Value *v = global_get(name, nlen);
            if (!v) die("line %u: undefined variable '%s'", current_line(), name);
            push(val_clone(v));
            break;
        }
        case OP_STORE: {
            uint8_t idx = frame_read_byte();
            const char *name = const_str(idx);
            size_t nlen = const_str_len(idx);
            Value val = pop();
            store_var(name, nlen, val);
            break;
        }

        /* ── Arithmetic ────────────────────────────────────────────── */
        case OP_ADD: {
            Value b = pop();
            Value a = pop();
            if (a.tag == V_NUM && b.tag == V_NUM) {
                push(val_num(a.as.num + b.as.num));
            } else if (a.tag == V_STR && b.tag == V_STR) {
                WStr *r = wstr_cat(a.as.str, b.as.str);
                val_drop(&a); val_drop(&b);
                push(val_str_own(r));
            } else if (a.tag == V_STR) {
                char *fb = val_format(&b);
                WStr *bs = wstr_from_cstr(fb);
                free(fb);
                WStr *r = wstr_cat(a.as.str, bs);
                wstr_dec(bs);
                val_drop(&a); val_drop(&b);
                push(val_str_own(r));
            } else if (b.tag == V_STR) {
                char *fa = val_format(&a);
                WStr *as = wstr_from_cstr(fa);
                free(fa);
                WStr *r = wstr_cat(as, b.as.str);
                wstr_dec(as);
                val_drop(&a); val_drop(&b);
                push(val_str_own(r));
            } else {
                die("line %u: cannot add %s and %s", current_line(), type_name(&a), type_name(&b));
            }
            break;
        }
        case OP_SUB: {
            Value b = pop(), a = pop();
            if (a.tag != V_NUM || b.tag != V_NUM)
                die("line %u: cannot subtract %s and %s", current_line(), type_name(&a), type_name(&b));
            push(val_num(a.as.num - b.as.num));
            break;
        }
        case OP_MUL: {
            Value b = pop(), a = pop();
            if (a.tag != V_NUM || b.tag != V_NUM)
                die("line %u: cannot multiply %s and %s", current_line(), type_name(&a), type_name(&b));
            push(val_num(a.as.num * b.as.num));
            break;
        }
        case OP_DIV: {
            Value b = pop(), a = pop();
            if (a.tag != V_NUM || b.tag != V_NUM)
                die("line %u: cannot divide %s and %s", current_line(), type_name(&a), type_name(&b));
            if (b.as.num == 0.0) die("line %u: division by zero", current_line());
            push(val_num(a.as.num / b.as.num));
            break;
        }
        case OP_MOD: {
            Value b = pop(), a = pop();
            if (a.tag != V_NUM || b.tag != V_NUM)
                die("line %u: cannot modulo %s and %s", current_line(), type_name(&a), type_name(&b));
            if (b.as.num == 0.0) die("line %u: division by zero", current_line());
            push(val_num(fmod(a.as.num, b.as.num)));
            break;
        }
        case OP_NEG: {
            Value a = pop();
            if (a.tag != V_NUM)
                die("line %u: cannot negate %s", current_line(), type_name(&a));
            push(val_num(-a.as.num));
            break;
        }

        /* ── Comparison ────────────────────────────────────────────── */
        case OP_EQ: {
            Value b = pop(), a = pop();
            bool r = val_eq(&a, &b);
            val_drop(&a); val_drop(&b);
            push(val_bool(r));
            break;
        }
        case OP_NEQ: {
            Value b = pop(), a = pop();
            bool r = !val_eq(&a, &b);
            val_drop(&a); val_drop(&b);
            push(val_bool(r));
            break;
        }
        case OP_LT: {
            Value b = pop(), a = pop();
            if (a.tag == V_NUM && b.tag == V_NUM)
                push(val_bool(a.as.num < b.as.num));
            else if (a.tag == V_STR && b.tag == V_STR) {
                bool r = strcmp(a.as.str->data, b.as.str->data) < 0;
                val_drop(&a); val_drop(&b);
                push(val_bool(r));
            } else die("line %u: cannot compare %s and %s", current_line(), type_name(&a), type_name(&b));
            break;
        }
        case OP_LTE: {
            Value b = pop(), a = pop();
            if (a.tag == V_NUM && b.tag == V_NUM)
                push(val_bool(a.as.num <= b.as.num));
            else if (a.tag == V_STR && b.tag == V_STR) {
                bool r = strcmp(a.as.str->data, b.as.str->data) <= 0;
                val_drop(&a); val_drop(&b);
                push(val_bool(r));
            } else die("line %u: cannot compare %s and %s", current_line(), type_name(&a), type_name(&b));
            break;
        }
        case OP_GT: {
            Value b = pop(), a = pop();
            if (a.tag == V_NUM && b.tag == V_NUM)
                push(val_bool(a.as.num > b.as.num));
            else if (a.tag == V_STR && b.tag == V_STR) {
                bool r = strcmp(a.as.str->data, b.as.str->data) > 0;
                val_drop(&a); val_drop(&b);
                push(val_bool(r));
            } else die("line %u: cannot compare %s and %s", current_line(), type_name(&a), type_name(&b));
            break;
        }
        case OP_GTE: {
            Value b = pop(), a = pop();
            if (a.tag == V_NUM && b.tag == V_NUM)
                push(val_bool(a.as.num >= b.as.num));
            else if (a.tag == V_STR && b.tag == V_STR) {
                bool r = strcmp(a.as.str->data, b.as.str->data) >= 0;
                val_drop(&a); val_drop(&b);
                push(val_bool(r));
            } else die("line %u: cannot compare %s and %s", current_line(), type_name(&a), type_name(&b));
            break;
        }
        case OP_NOT: {
            Value a = pop();
            bool t = is_truthy(&a);
            val_drop(&a);
            push(val_bool(!t));
            break;
        }

        /* ── Control flow ──────────────────────────────────────────── */
        case OP_JUMP: {
            uint16_t target = frame_read_u16();
            frame()->ip = target;
            break;
        }
        case OP_JUMP_IF_FALSE: {
            uint16_t target = frame_read_u16();
            Value cond = pop();
            if (!is_truthy(&cond)) frame()->ip = target;
            val_drop(&cond);
            break;
        }
        case OP_JUMP_IF_TRUE: {
            uint16_t target = frame_read_u16();
            Value cond = pop();
            if (is_truthy(&cond)) frame()->ip = target;
            val_drop(&cond);
            break;
        }
        case OP_PEEK_JUMP_FALSE: {
            uint16_t target = frame_read_u16();
            if (!is_truthy(peek())) frame()->ip = target;
            break;
        }
        case OP_PEEK_JUMP_TRUE: {
            uint16_t target = frame_read_u16();
            if (is_truthy(peek())) frame()->ip = target;
            break;
        }

        /* ── Functions ─────────────────────────────────────────────── */
        case OP_CALL: {
            uint8_t name_idx = frame_read_byte();
            uint8_t arg_count = frame_read_byte();

            const char *name = const_str(name_idx);

            /* Pop arguments (they come off in reverse order). */
            Value args_buf[256];
            for (int i = arg_count - 1; i >= 0; i--)
                args_buf[i] = pop();

            /* Try builtin first. */
            Value builtin_result;
            if (call_builtin(name, arg_count, args_buf, &builtin_result)) {
                /* Drop args. */
                for (int i = 0; i < arg_count; i++) val_drop(&args_buf[i]);
                push(builtin_result);
                break;
            }

            /* User function. */
            int ci = find_chunk(&g_prog, name);
            if (ci < 0) die("line %u: undefined function '%s'", current_line(), name);

            Chunk *fn = &g_prog.chunks[ci];
            if ((int)arg_count != fn->param_count)
                die("line %u: %s() expects %d arguments, got %d",
                    current_line(), name, fn->param_count, arg_count);

            /* Push args back onto stack (they'll be Store'd by the callee). */
            for (int i = 0; i < arg_count; i++)
                push(args_buf[i]);

            if (g_fp >= MAX_FRAMES) die("call stack overflow");
            frame_init(&g_frames[g_fp], ci);
            g_fp++;
            break;
        }
        case OP_RETURN: {
            Value val = pop();
            g_fp--;
            frame_free(&g_frames[g_fp]);
            push(val);
            break;
        }
        case OP_RETURN_NONE: {
            g_fp--;
            frame_free(&g_frames[g_fp]);
            push(val_none());
            break;
        }

        /* ── Collections ───────────────────────────────────────────── */
        case OP_MAKE_ARRAY: {
            uint8_t n = frame_read_byte();
            WArray *a = warray_new(n);
            /* Elements are on stack in reverse order. */
            Value tmp[256];
            for (int i = n - 1; i >= 0; i--)
                tmp[i] = pop();
            for (int i = 0; i < n; i++)
                warray_push(a, tmp[i]);  /* items already owned */
            push(val_array_own(a));
            break;
        }
        case OP_MAKE_DICT: {
            uint8_t n = frame_read_byte();
            WDict *d = wdict_new(n);
            /* Pairs on stack: (key, val) * n in reverse order. */
            typedef struct { Value key; Value val; } KV;
            KV pairs[128];
            for (int i = n - 1; i >= 0; i--) {
                pairs[i].val = pop();
                pairs[i].key = pop();
            }
            for (int i = 0; i < n; i++) {
                WStr *k = to_dict_key(&pairs[i].key);
                val_drop(&pairs[i].key);
                wdict_set(d, k, pairs[i].val);
            }
            push(val_dict_own(d));
            break;
        }
        case OP_GET_INDEX: {
            Value idx = pop();
            Value obj = pop();
            if (obj.tag == V_ARRAY) {
                if (idx.tag != V_NUM) die("line %u: array index must be number", current_line());
                size_t i = (size_t)idx.as.num;
                WArray *a = obj.as.array;
                if (i >= a->len) die("line %u: index %zu out of bounds (length %zu)",
                                     current_line(), i, a->len);
                push(val_clone(&a->items[i]));
                val_drop(&obj);
            } else if (obj.tag == V_DICT) {
                WStr *k = to_dict_key(&idx);
                const Value *v = wdict_get(obj.as.dict, k->data, k->len);
                if (!v) die("line %u: undefined dict key \"%s\"", current_line(), k->data);
                push(val_clone(v));
                wstr_dec(k);
                val_drop(&obj);
            } else {
                die("line %u: cannot index %s", current_line(), type_name(&obj));
            }
            val_drop(&idx);
            break;
        }
        case OP_SET_INDEX: {
            Value new_val = pop();
            Value idx     = pop();
            Value obj     = pop();
            if (obj.tag == V_ARRAY) {
                if (idx.tag != V_NUM) die("line %u: array index must be number", current_line());
                size_t i = (size_t)idx.as.num;
                WArray *a = obj.as.array;
                if (i >= a->len) die("line %u: index %zu out of bounds (length %zu)",
                                     current_line(), i, a->len);
                a = warray_cow(a);
                val_drop(&a->items[i]);
                a->items[i] = new_val;
                obj.as.array = a;
                push(obj);
            } else if (obj.tag == V_DICT) {
                WStr *k = to_dict_key(&idx);
                WDict *d = obj.as.dict;
                d = wdict_cow(d);
                wdict_set(d, k, new_val);
                obj.as.dict = d;
                push(obj);
            } else {
                die("line %u: cannot set index on %s", current_line(), type_name(&obj));
            }
            val_drop(&idx);
            break;
        }

        /* ── I/O ───────────────────────────────────────────────────── */
        case OP_PRINT: {
            Value val = pop();
            char *s = val_format(&val);
            printf("%s\n", s);
            free(s);
            val_drop(&val);
            break;
        }
        case OP_POP: {
            Value v = pop();
            val_drop(&v);
            break;
        }

        /* ── Halt ──────────────────────────────────────────────────── */
        case OP_HALT: {
            g_fp--;
            frame_free(&g_frames[g_fp]);
            return;
        }

        default:
            die("unknown opcode 0x%02x at line %u", op, current_line());
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  Disassembler  (matches Rust --dump output)
 * ═══════════════════════════════════════════════════════════════════════════ */

static const char *opcode_name(uint8_t op) {
    switch (op) {
    case OP_PUSH_CONST:      return "PUSH_CONST";
    case OP_PUSH_TRUE:       return "PUSH_TRUE";
    case OP_PUSH_FALSE:      return "PUSH_FALSE";
    case OP_PUSH_NONE:       return "PUSH_NONE";
    case OP_LOAD:            return "LOAD";
    case OP_STORE:           return "STORE";
    case OP_LOAD_GLOBAL:     return "LOAD_GLOBAL";
    case OP_ADD:             return "ADD";
    case OP_SUB:             return "SUB";
    case OP_MUL:             return "MUL";
    case OP_DIV:             return "DIV";
    case OP_MOD:             return "MOD";
    case OP_NEG:             return "NEG";
    case OP_EQ:              return "EQ";
    case OP_NEQ:             return "NEQ";
    case OP_LT:              return "LT";
    case OP_LTE:             return "LTE";
    case OP_GT:              return "GT";
    case OP_GTE:             return "GTE";
    case OP_NOT:             return "NOT";
    case OP_JUMP:            return "JUMP";
    case OP_JUMP_IF_FALSE:   return "JUMP_IF_FALSE";
    case OP_JUMP_IF_TRUE:    return "JUMP_IF_TRUE";
    case OP_PEEK_JUMP_FALSE: return "PEEK_JUMP_IF_FALSE";
    case OP_PEEK_JUMP_TRUE:  return "PEEK_JUMP_IF_TRUE";
    case OP_CALL:            return "CALL";
    case OP_RETURN:          return "RETURN";
    case OP_RETURN_NONE:     return "RETURN_NONE";
    case OP_MAKE_ARRAY:      return "MAKE_ARRAY";
    case OP_MAKE_DICT:       return "MAKE_DICT";
    case OP_GET_INDEX:       return "GET_INDEX";
    case OP_SET_INDEX:       return "SET_INDEX";
    case OP_PRINT:           return "PRINT";
    case OP_POP:             return "POP";
    case OP_HALT:            return "HALT";
    default:                 return NULL;
    }
}

/* Operand size: 1 for u8-operand ops, 2 for u16/call ops, 0 otherwise. */
static int operand_size(uint8_t op) {
    switch (op) {
    case OP_PUSH_CONST: case OP_LOAD: case OP_LOAD_GLOBAL:
    case OP_STORE: case OP_MAKE_ARRAY: case OP_MAKE_DICT:
        return 1;
    case OP_JUMP: case OP_JUMP_IF_FALSE: case OP_JUMP_IF_TRUE:
    case OP_PEEK_JUMP_FALSE: case OP_PEEK_JUMP_TRUE: case OP_CALL:
        return 2;
    default:
        return 0;
    }
}

static char *const_annotation(const Chunk *c, int idx) {
    if (idx >= c->const_count) return strdup("(out of range)");
    const Value *v = &c->constants[idx];
    char buf[256];
    switch (v->tag) {
    case V_STR:
        snprintf(buf, sizeof(buf), "'%s'", v->as.str->data);
        break;
    case V_NUM: {
        double n = v->as.num;
        if (n == (double)(int64_t)n && fabs(n) < 1e15)
            snprintf(buf, sizeof(buf), "'%lld'", (long long)(int64_t)n);
        else
            snprintf(buf, sizeof(buf), "'%g'", n);
        break;
    }
    case V_BOOL:
        snprintf(buf, sizeof(buf), "'%s'", v->as.boolean ? "true" : "false");
        break;
    case V_NONE:  return strdup("'none'");
    case V_ARRAY: return strdup("[array]");
    case V_DICT:  return strdup("{dict}");
    }
    return strdup(buf);
}

static void disassemble_chunk(const Chunk *c) {
    printf("== %s ==\n", c->name);
    uint32_t offset = 0;
    while (offset < c->code_len) {
        /* Offset */
        printf("%04u  ", offset);

        /* Line number */
        uint32_t line = (offset < c->lines_len) ? c->lines[offset] : 0;
        if (offset > 0 && offset < c->lines_len &&
            c->lines[offset - 1] == line) {
            printf("   |  ");
        } else {
            printf("%4u  ", line);
        }

        uint8_t op = c->code[offset];
        const char *name = opcode_name(op);
        if (!name) {
            printf("UNKNOWN(0x%02x)\n", op);
            offset++;
            continue;
        }

        int opsz = operand_size(op);
        if (opsz == 0) {
            printf("%s\n", name);
            offset += 1;
        } else if (opsz == 1) {
            uint8_t idx = c->code[offset + 1];
            char *ann = const_annotation(c, idx);
            printf("%-20s %3u    %s\n", name, idx, ann);
            free(ann);
            offset += 2;
        } else {
            /* 2-byte operand */
            switch (op) {
            case OP_JUMP: case OP_JUMP_IF_FALSE: case OP_JUMP_IF_TRUE:
            case OP_PEEK_JUMP_FALSE: case OP_PEEK_JUMP_TRUE: {
                uint16_t target = ((uint16_t)c->code[offset+1] << 8)
                                | c->code[offset+2];
                printf("%-20s        -> %04u\n", name, target);
                break;
            }
            case OP_CALL: {
                uint8_t name_idx = c->code[offset + 1];
                uint8_t argc     = c->code[offset + 2];
                char *ann = const_annotation(c, name_idx);
                printf("%-20s %3u    %s (%u args)\n", name, name_idx, ann, argc);
                free(ann);
                break;
            }
            default:
                printf("%-20s 0x%02x 0x%02x\n", name,
                       c->code[offset+1], c->code[offset+2]);
                break;
            }
            offset += 3;
        }
    }
}

static void disassemble_all(void) {
    /* Chunk 0 = <main>, then the rest sorted by name. */
    disassemble_chunk(&g_prog.chunks[0]);
    for (int i = 1; i < g_prog.chunk_count; i++) {
        printf("\n");
        disassemble_chunk(&g_prog.chunks[i]);
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  REPL  (interactive mode)
 * ═══════════════════════════════════════════════════════════════════════════ */

/*
 * The REPL accumulates lines of Whispem source, writes them to a temp file,
 * compiles via wsc.whbc (by invoking wvm as a subprocess), then loads and
 * executes the resulting .whbc in-process.
 *
 * Because the C VM uses process-global state, we fork() for each execution
 * so the parent's state stays clean.  On systems without fork (Windows) we
 * fall back to system().
 */

static const char *find_compiler(const char *self_path) {
    /* Try paths relative to the binary: compiler/wsc.whbc */
    static char buf[4096];

    /* Try based on argv[0] directory */
    const char *last_slash = strrchr(self_path, '/');
    if (last_slash) {
        size_t dir_len = (size_t)(last_slash - self_path);
        snprintf(buf, sizeof buf, "%.*s/compiler/wsc.whbc", (int)dir_len, self_path);
        FILE *f = fopen(buf, "rb");
        if (f) { fclose(f); return buf; }
    }

    /* Try ./compiler/wsc.whbc */
    {
        FILE *f = fopen("compiler/wsc.whbc", "rb");
        if (f) { fclose(f); return "compiler/wsc.whbc"; }
    }

    return NULL;
}

#ifndef _WIN32
#include <unistd.h>
#include <sys/wait.h>
#endif

static void run_repl(const char *self_path) {
    const char *wsc = find_compiler(self_path);
    if (!wsc) {
        fprintf(stderr, "Cannot find compiler/wsc.whbc.\n"
                        "Run the REPL from the Whispem project root.\n");
        exit(1);
    }

    printf("Whispem v3.0.0 — REPL (C VM)\n");
    printf("Type 'exit' or press Ctrl-D to quit.\n\n");

    /* Accumulated source */
    size_t src_cap = 4096;
    size_t src_len = 0;
    char  *source  = malloc(src_cap);
    source[0] = '\0';

    /* Temp files */
    char tmp_wsp[256], tmp_whbc[256];
    snprintf(tmp_wsp,  sizeof tmp_wsp,  "/tmp/wvm_repl_%d.wsp",  (int)getpid());
    snprintf(tmp_whbc, sizeof tmp_whbc, "/tmp/wvm_repl_%d.whbc", (int)getpid());

    for (;;) {
        printf(">>> ");
        fflush(stdout);

        char line[4096];
        if (!fgets(line, sizeof line, stdin)) {
            printf("\n");
            break;
        }

        /* Trim trailing newline for checking */
        char *trimmed = line;
        while (*trimmed == ' ' || *trimmed == '\t') trimmed++;
        char *end = trimmed + strlen(trimmed) - 1;
        while (end >= trimmed && (*end == '\n' || *end == '\r' || *end == ' ')) end--;
        *(end + 1) = '\0';

        if (strcmp(trimmed, "exit") == 0 || strcmp(trimmed, "quit") == 0)
            break;
        if (*trimmed == '\0')
            continue;

        /* Multi-line: if line ends with '{', read until '}' */
        char block[16384];
        snprintf(block, sizeof block, "%s\n", line);  /* include original newline */
        if (end >= trimmed && *end == '{') {
            for (;;) {
                printf("... ");
                fflush(stdout);
                char cont[4096];
                if (!fgets(cont, sizeof cont, stdin)) break;
                strncat(block, cont, sizeof block - strlen(block) - 1);
                char *ct = cont;
                while (*ct == ' ' || *ct == '\t') ct++;
                char *ce = ct + strlen(ct) - 1;
                while (ce >= ct && (*ce == '\n' || *ce == '\r' || *ce == ' ')) ce--;
                *(ce + 1) = '\0';
                if (strcmp(ct, "}") == 0) break;
            }
        }

        /* Append to accumulated source */
        size_t blen = strlen(block);
        while (src_len + blen + 1 > src_cap) {
            src_cap *= 2;
            source = realloc(source, src_cap);
        }
        memcpy(source + src_len, block, blen);
        src_len += blen;
        source[src_len] = '\0';

        /* Write accumulated source to temp file */
        FILE *wsp = fopen(tmp_wsp, "w");
        if (!wsp) { fprintf(stderr, "Cannot write temp file\n"); continue; }
        fputs(source, wsp);
        fclose(wsp);

        /* Compile: invoke wvm with the compiler */
        char cmd[8192];
        snprintf(cmd, sizeof cmd, "\"%s\" \"%s\" \"%s\" > /dev/null 2>&1",
                 self_path, wsc, tmp_wsp);
        int rc = system(cmd);
        if (rc != 0) {
            fprintf(stderr, "Compile error.\n");
            /* Undo the last line so the user can try again */
            src_len -= blen;
            source[src_len] = '\0';
            continue;
        }

        /* Execute the compiled .whbc in a child process */
#ifndef _WIN32
        pid_t pid = fork();
        if (pid == 0) {
            /* Child: load and run the .whbc */
            FILE *fp = fopen(tmp_whbc, "rb");
            if (!fp) _exit(1);
            fseek(fp, 0, SEEK_END);
            long sz = ftell(fp);
            fseek(fp, 0, SEEK_SET);
            uint8_t *data = malloc(sz);
            if ((long)fread(data, 1, sz, fp) != sz) _exit(1);
            fclose(fp);
            g_prog = load_program(data, sz);
            free(data);
            g_argc = 0;
            g_argv = NULL;
            globals_init();
            frame_init(&g_frames[0], 0);
            g_fp = 1;
            g_sp = 0;
            execute();
            _exit(0);
        } else if (pid > 0) {
            int status;
            waitpid(pid, &status, 0);
        } else {
            perror("fork");
        }
#else
        /* Fallback: system() */
        snprintf(cmd, sizeof cmd, "\"%s\" \"%s\"", self_path, tmp_whbc);
        system(cmd);
#endif
    }

    /* Cleanup */
    unlink(tmp_wsp);
    unlink(tmp_whbc);
    free(source);
    printf("Bye!\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 *  main
 * ═══════════════════════════════════════════════════════════════════════════ */

int main(int argc, char **argv) {
    if (argc < 2) {
        run_repl(argv[0]);
        return 0;
    }

    int argi = 1;
    bool dump_mode = false;

    if (strcmp(argv[argi], "--dump") == 0) {
        dump_mode = true;
        argi++;
        if (argi >= argc) {
            fprintf(stderr, "Usage: wvm --dump <file.whbc>\n");
            return 1;
        }
    }

    /* Read the .whbc file. */
    const char *path = argv[argi];
    FILE *fp = fopen(path, "rb");
    if (!fp) die("cannot open '%s'", path);
    fseek(fp, 0, SEEK_END);
    long sz = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    uint8_t *data = malloc(sz);
    if ((long)fread(data, 1, sz, fp) != sz) die("failed to read '%s'", path);
    fclose(fp);

    /* Parse bytecode. */
    g_prog = load_program(data, sz);
    free(data);

    if (dump_mode) {
        disassemble_all();
        return 0;
    }

    /* Script args = everything after the .whbc file. */
    g_argc = argc - argi - 1;
    g_argv = argv + argi + 1;

    /* Initialise globals and run. */
    globals_init();
    frame_init(&g_frames[0], 0);  /* chunk 0 = <main> */
    g_fp = 1;
    execute();

    return 0;
}
