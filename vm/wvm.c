/*
 * wvm.c — Whispem Virtual Machine v5.0.0
 *
 * Executes .whbc bytecode files (format version 4).
 *
 *   gcc -O2 -o wvm vm/wvm.c -lm
 *   ./wvm examples/hello.whbc
 *   ./wvm compiler/wsc.whbc source.wsp
 */

#include <ctype.h>
#include <math.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STACK        4096
#define MAX_FRAMES       256
#define MAX_OPEN_UPVALS  64

enum {
    OP_PUSH_CONST       = 0x00,
    OP_PUSH_TRUE        = 0x01,
    OP_PUSH_FALSE       = 0x02,
    OP_PUSH_NONE        = 0x03,
    OP_LOAD             = 0x10,
    OP_STORE            = 0x11,
    OP_LOAD_GLOBAL      = 0x12,
    OP_LOAD_UPVALUE     = 0x13,
    OP_STORE_UPVALUE    = 0x14,
    OP_CLOSE_UPVALUE    = 0x15,
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
    OP_MAKE_CLOSURE     = 0x53,
    OP_MAKE_ARRAY       = 0x60,
    OP_MAKE_DICT        = 0x61,
    OP_GET_INDEX        = 0x62,
    OP_SET_INDEX        = 0x63,
    OP_PRINT            = 0x70,
    OP_POP              = 0x71,
    OP_HALT             = 0xFF,
};

static void die(const char *fmt, ...) {
    va_list ap; va_start(ap, fmt);
    fprintf(stderr, "wvm: "); vfprintf(stderr, fmt, ap); fprintf(stderr, "\n");
    va_end(ap); exit(1);
}

typedef struct WStr { int rc; size_t len; char data[]; } WStr;

static WStr *wstr_new(const char *s, size_t len) {
    WStr *w = malloc(sizeof(WStr)+len+1);
    if(!w) die("oom");
    w->rc=1; w->len=len;
    memcpy(w->data,s,len); w->data[len]='\0'; return w;
}
static WStr *wstr_from_cstr(const char *s) { return wstr_new(s, strlen(s)); }
static void  wstr_inc(WStr *w)             { if(w) w->rc++; }
static void  wstr_dec(WStr *w)             { if(w && --w->rc<=0) free(w); }
static WStr *wstr_cat(const WStr *a, const WStr *b) {
    size_t len=a->len+b->len;
    WStr *w=malloc(sizeof(WStr)+len+1); if(!w) die("oom");
    w->rc=1; w->len=len;
    memcpy(w->data,a->data,a->len);
    memcpy(w->data+a->len,b->data,b->len);
    w->data[len]='\0'; return w;
}

typedef struct Value   Value;
typedef struct WArray  WArray;
typedef struct WDict   WDict;
typedef struct Closure Closure;
enum ValueTag { V_NUM, V_BOOL, V_STR, V_ARRAY, V_DICT, V_CLOSURE, V_NONE };

struct Value {
    enum ValueTag tag;
    union { double num; bool boolean; WStr *str; WArray *array; WDict *dict; Closure *closure; } as;
};

static void val_drop(Value *v);
static Value val_clone(const Value *v);
static void warray_dec(WArray *a);
static void wdict_dec(WDict *d);
static void closure_dec(Closure *c);

static Value val_num(double n)     { return (Value){.tag=V_NUM,  .as.num=n}; }
static Value val_bool(bool b)      { return (Value){.tag=V_BOOL, .as.boolean=b}; }
static Value val_none(void)        { return (Value){.tag=V_NONE}; }
static Value val_str(WStr *s)      { wstr_inc(s); return (Value){.tag=V_STR, .as.str=s}; }
static Value val_str_own(WStr *s)  { return (Value){.tag=V_STR, .as.str=s}; }

static void val_drop(Value *v) {
    switch(v->tag) {
    case V_STR:     wstr_dec(v->as.str);      break;
    case V_ARRAY:   warray_dec(v->as.array);   break;
    case V_DICT:    wdict_dec(v->as.dict);     break;
    case V_CLOSURE: closure_dec(v->as.closure);break;
    default: break;
    }
    v->tag=V_NONE;
}

struct WArray { int rc; size_t len; size_t cap; Value *items; };

static WArray *warray_new(size_t cap) {
    WArray *a=malloc(sizeof(WArray)); if(!a) die("oom");
    a->rc=1; a->len=0; a->cap=(cap<4?4:cap);
    a->items=malloc(sizeof(Value)*a->cap); if(!a->items) die("oom"); return a;
}
static void warray_inc(WArray *a) { if(a) a->rc++; }
static void warray_dec(WArray *a) {
    if(!a) return;
    if(--a->rc<=0) { for(size_t i=0;i<a->len;i++) val_drop(&a->items[i]); free(a->items); free(a); }
}
static void warray_push(WArray *a, Value v) {
    if(a->len>=a->cap) { a->cap*=2; a->items=realloc(a->items,sizeof(Value)*a->cap); if(!a->items) die("oom"); }
    a->items[a->len++]=v;
}
static WArray *warray_clone(const WArray *src) {
    WArray *a=warray_new(src->len); a->len=src->len;
    for(size_t i=0;i<src->len;i++) a->items[i]=val_clone(&src->items[i]);
    return a;
}
static WArray *warray_cow(WArray *a) {
    if(a->rc==1) return a;
    WArray *copy=warray_clone(a); warray_dec(a); return copy;
}

struct WDict { int rc; size_t len; size_t cap; WStr **keys; Value *vals; };

static WDict *wdict_new(size_t cap) {
    WDict *d=malloc(sizeof(WDict)); if(!d) die("oom");
    d->rc=1; d->len=0; d->cap=(cap<4?4:cap);
    d->keys=malloc(sizeof(WStr*)*d->cap); d->vals=malloc(sizeof(Value)*d->cap);
    if(!d->keys||!d->vals) die("oom");
    return d;
}
static void wdict_inc(WDict *d) { if(d) d->rc++; }
static void wdict_dec(WDict *d) {
    if(!d) return;
    if(--d->rc<=0) { for(size_t i=0;i<d->len;i++){wstr_dec(d->keys[i]);val_drop(&d->vals[i]);} free(d->keys);free(d->vals);free(d); }
}
static size_t wdict_find(const WDict *d, const char *key, size_t kl) {
    for(size_t i=0;i<d->len;i++) if(d->keys[i]->len==kl&&memcmp(d->keys[i]->data,key,kl)==0) return i;
    return (size_t)-1;
}
static void wdict_set(WDict *d, WStr *key, Value val) {
    size_t idx=wdict_find(d,key->data,key->len);
    if(idx!=(size_t)-1){wstr_dec(key);val_drop(&d->vals[idx]);d->vals[idx]=val;return;}
    if(d->len>=d->cap){d->cap*=2;d->keys=realloc(d->keys,sizeof(WStr*)*d->cap);d->vals=realloc(d->vals,sizeof(Value)*d->cap);if(!d->keys||!d->vals)die("oom");}
    d->keys[d->len]=key; d->vals[d->len]=val; d->len++;
}
static const Value *wdict_get(const WDict *d, const char *key, size_t kl) {
    size_t idx=wdict_find(d,key,kl); if(idx==(size_t)-1) return NULL; return &d->vals[idx];
}
static WDict *wdict_clone(const WDict *src) {
    WDict *d=wdict_new(src->len); d->len=src->len;
    for(size_t i=0;i<src->len;i++){d->keys[i]=src->keys[i];wstr_inc(d->keys[i]);d->vals[i]=val_clone(&src->vals[i]);}
    return d;
}
static WDict *wdict_cow(WDict *d) {
    if(d->rc==1) return d;
    WDict *copy=wdict_clone(d); wdict_dec(d); return copy;
}

typedef struct UpvalueCell {
    int   rc;
    Value val;
} UpvalueCell;

static UpvalueCell *uv_cell_new(Value v) {
    UpvalueCell *c=malloc(sizeof(UpvalueCell)); if(!c) die("oom");
    c->rc=1; c->val=v; return c;
}
static void uv_cell_inc(UpvalueCell *c) { if(c) c->rc++; }
static void uv_cell_dec(UpvalueCell *c) {
    if(!c) return;
    if(--c->rc<=0){val_drop(&c->val);free(c);}
}

struct Closure {
    int           rc;
    int           chunk_idx;
    uint8_t       uv_count;
    UpvalueCell **upvalues;  /* owned references */
};

static Closure *closure_new(int ci, uint8_t uvc) {
    Closure *cl=malloc(sizeof(Closure)); if(!cl) die("oom");
    cl->rc=1; cl->chunk_idx=ci; cl->uv_count=uvc;
    cl->upvalues=(uvc>0)?malloc(sizeof(UpvalueCell*)*uvc):NULL;
    return cl;
}
static void closure_inc(Closure *cl) { if(cl) cl->rc++; }
static void closure_dec(Closure *cl) {
    if(!cl) return;
    if(--cl->rc<=0){
        for(int i=0;i<cl->uv_count;i++) uv_cell_dec(cl->upvalues[i]);
        free(cl->upvalues); free(cl);
    }
}
static Value val_closure_own(Closure *cl) { return (Value){.tag=V_CLOSURE,.as.closure=cl}; }

static Value val_clone(const Value *v) {
    Value r=*v;
    switch(v->tag){
    case V_STR:     wstr_inc(r.as.str);       break;
    case V_ARRAY:   warray_inc(r.as.array);    break;
    case V_DICT:    wdict_inc(r.as.dict);      break;
    case V_CLOSURE: closure_inc(r.as.closure); break;
    default: break;
    }
    return r;
}
static Value val_array_own(WArray *a) { return (Value){.tag=V_ARRAY,.as.array=a}; }
static Value val_dict_own(WDict *d)   { return (Value){.tag=V_DICT, .as.dict=d};  }

static bool is_truthy(const Value *v) {
    switch(v->tag){
    case V_BOOL:    return v->as.boolean;
    case V_NUM:     return v->as.num!=0.0;
    case V_STR:     return v->as.str->len>0;
    case V_ARRAY:   return v->as.array->len>0;
    case V_DICT:    return v->as.dict->len>0;
    case V_CLOSURE: return true;
    case V_NONE:    return false;
    }
    return false;
}

static char *fmt_number(double n) {
    char buf[64];
    if(n==(double)(int64_t)n&&fabs(n)<1e15) snprintf(buf,sizeof(buf),"%lld",(long long)(int64_t)n);
    else { for(int p=1;p<=17;p++){snprintf(buf,sizeof(buf),"%.*g",p,n);char *e;double b=strtod(buf,&e);if(b==n)break;} }
    return strdup(buf);
}

static char *val_format(const Value *v);
static int cmp_str_ptrs(const void *a,const void *b){return strcmp(*(const char**)a,*(const char**)b);}

static char *val_format(const Value *v) {
    switch(v->tag){
    case V_NUM:  return fmt_number(v->as.num);
    case V_BOOL: return strdup(v->as.boolean?"true":"false");
    case V_STR:  return strdup(v->as.str->data);
    case V_NONE: return strdup("");
    case V_CLOSURE: {
        char buf[64]; snprintf(buf,sizeof(buf),"<fn>"); return strdup(buf);
    }
    case V_ARRAY: {
        size_t tot=3,n=v->as.array->len;
        char **p=malloc(sizeof(char*)*(n+1));
        for(size_t i=0;i<n;i++){p[i]=val_format(&v->as.array->items[i]);tot+=strlen(p[i])+2;}
        char *out=malloc(tot); strcpy(out,"[");
        for(size_t i=0;i<n;i++){if(i>0)strcat(out,", ");strcat(out,p[i]);free(p[i]);}
        strcat(out,"]"); free(p); return out;
    }
    case V_DICT: {
        size_t n=v->as.dict->len;
        char **e=malloc(sizeof(char*)*(n+1));
        for(size_t i=0;i<n;i++){char *fv=val_format(&v->as.dict->vals[i]);size_t el=2+v->as.dict->keys[i]->len+2+strlen(fv)+4;e[i]=malloc(el);sprintf(e[i],"\"%s\": %s",v->as.dict->keys[i]->data,fv);free(fv);}
        qsort(e,n,sizeof(char*),cmp_str_ptrs);
        size_t tot=3; for(size_t i=0;i<n;i++) tot+=strlen(e[i])+2;
        char *out=malloc(tot); strcpy(out,"{");
        for(size_t i=0;i<n;i++){if(i>0)strcat(out,", ");strcat(out,e[i]);free(e[i]);}
        strcat(out,"}"); free(e); return out;
    }
    }
    return strdup("");
}

static const char *type_name(const Value *v) {
    switch(v->tag){
    case V_NUM:     return "number";
    case V_BOOL:    return "bool";
    case V_STR:     return "string";
    case V_ARRAY:   return "array";
    case V_DICT:    return "dict";
    case V_CLOSURE: return "function";
    case V_NONE:    return "none";
    }
    return "unknown";
}

static WStr *to_dict_key(const Value *v) {
    switch(v->tag){
    case V_STR: wstr_inc(v->as.str); return v->as.str;
    case V_NUM: { char *s=fmt_number(v->as.num); WStr *w=wstr_from_cstr(s); free(s); return w; }
    default: die("dict key must be string or number, got %s",type_name(v)); return NULL;
    }
}

typedef struct {
    char     *name;
    int       param_count;
    int       upvalue_count;
    Value    *constants;
    int       const_count;
    uint8_t  *code;
    uint32_t  code_len;
    uint32_t *lines;
    uint32_t  lines_len;
} Chunk;

typedef struct { const uint8_t *data; size_t len; size_t pos; } Reader;

static void need(Reader *r, size_t n) { if(r->pos+n>r->len) die("unexpected end of bytecode at %zu",r->pos); }
static uint8_t  read_u8(Reader *r)  { need(r,1); return r->data[r->pos++]; }
static uint16_t read_u16(Reader *r) { need(r,2); uint16_t v=((uint16_t)r->data[r->pos]<<8)|r->data[r->pos+1]; r->pos+=2; return v; }
static uint32_t read_u32(Reader *r) { need(r,4); uint32_t v=((uint32_t)r->data[r->pos]<<24)|((uint32_t)r->data[r->pos+1]<<16)|((uint32_t)r->data[r->pos+2]<<8)|(uint32_t)r->data[r->pos+3]; r->pos+=4; return v; }
static double read_f64(Reader *r) { need(r,8); uint64_t bits=0; for(int i=0;i<8;i++) bits=(bits<<8)|r->data[r->pos+i]; r->pos+=8; double d; memcpy(&d,&bits,8); return d; }

static Value read_const(Reader *r) {
    uint8_t tag=read_u8(r);
    switch(tag){
    case 0: return val_num(read_f64(r));
    case 1: return val_bool(read_u8(r)!=0);
    case 2: { uint16_t sl=read_u16(r); need(r,sl); WStr *s=wstr_new((const char*)r->data+r->pos,sl); r->pos+=sl; return val_str_own(s); }
    case 3: return val_none();
    default: die("unknown constant tag %u",tag); return val_none();
    }
}

static Chunk read_chunk(Reader *r) {
    Chunk c={0};
    uint16_t nlen=read_u16(r); need(r,nlen);
    c.name=malloc(nlen+1); memcpy(c.name,r->data+r->pos,nlen); c.name[nlen]='\0'; r->pos+=nlen;
    c.param_count   = read_u8(r);
    c.upvalue_count = read_u8(r);   /* v5: upvalue_count field */
    c.const_count   = read_u8(r);
    c.constants=malloc(sizeof(Value)*(c.const_count?c.const_count:1));
    for(int i=0;i<c.const_count;i++) c.constants[i]=read_const(r);
    c.code_len=read_u32(r); need(r,c.code_len);
    c.code=malloc(c.code_len); memcpy(c.code,r->data+r->pos,c.code_len); r->pos+=c.code_len;
    c.lines_len=read_u32(r); need(r,c.lines_len*4);
    c.lines=malloc(sizeof(uint32_t)*c.lines_len);
    for(uint32_t i=0;i<c.lines_len;i++) c.lines[i]=read_u32(r);
    return c;
}

typedef struct { Chunk *chunks; int chunk_count; } Program;

static Program load_program(const uint8_t *data, size_t len) {
    Reader r={.data=data,.len=len,.pos=0};
    if(len<5) die("file too short");
    if(memcmp(r.data,"WHBC",4)!=0) die("bad magic bytes");
    r.pos=4;
    uint8_t ver=read_u8(&r);
    if(ver!=4) die("version mismatch: expected 4, got %u",ver);
    uint16_t fn_count=read_u16(&r);
    if(fn_count==0) die("no chunks");
    Program p; p.chunk_count=fn_count;
    p.chunks=malloc(sizeof(Chunk)*fn_count);
    for(int i=0;i<fn_count;i++) p.chunks[i]=read_chunk(&r);
    return p;
}

static int find_chunk(const Program *p, const char *name) {
    for(int i=0;i<p->chunk_count;i++) if(strcmp(p->chunks[i].name,name)==0) return i;
    return -1;
}

typedef struct { WStr *name; Value val; } Local;

typedef struct {
    int      chunk_idx;
    uint32_t ip;
    Local   *locals;
    int      local_count;
    int      local_cap;
    /* Upvalue cells owned by this frame (for open upvalues) */
    UpvalueCell *open_uv_cells[MAX_OPEN_UPVALS];
    char         open_uv_names[MAX_OPEN_UPVALS][256];
    int          open_uv_count;
    /* Upvalue cells captured from enclosing scope (for closures) */
    UpvalueCell **upvalues;
    int           uv_count;
} CallFrame;

static void frame_init(CallFrame *f, int ci, UpvalueCell **uvs, int uvc) {
    f->chunk_idx=ci; f->ip=0; f->local_count=0; f->local_cap=16;
    f->locals=malloc(sizeof(Local)*f->local_cap);
    f->open_uv_count=0;
    f->upvalues=uvs; f->uv_count=uvc;
}
static void frame_free(CallFrame *f) {
    for(int i=0;i<f->local_count;i++){wstr_dec(f->locals[i].name);val_drop(&f->locals[i].val);}
    free(f->locals);
    for(int i=0;i<f->open_uv_count;i++) uv_cell_dec(f->open_uv_cells[i]);
    for(int i=0;i<f->uv_count;i++) uv_cell_dec(f->upvalues[i]);
    if(f->uv_count>0) free(f->upvalues);
}

static Value *frame_get(CallFrame *f, const char *name, size_t nlen) {
    for(int i=0;i<f->local_count;i++)
        if(f->locals[i].name->len==nlen&&memcmp(f->locals[i].name->data,name,nlen)==0)
            return &f->locals[i].val;
    return NULL;
}
static void frame_set(CallFrame *f, WStr *name, Value val) {
    /* Also write through any open upvalue cell for this name */
    for(int i=0;i<f->open_uv_count;i++)
        if(strcmp(f->open_uv_names[i],name->data)==0) {
            val_drop(&f->open_uv_cells[i]->val);
            f->open_uv_cells[i]->val=val_clone(&val);
        }
    Value *ex=frame_get(f,name->data,name->len);
    if(ex){val_drop(ex);*ex=val;wstr_dec(name);return;}
    if(f->local_count>=f->local_cap){f->local_cap*=2;f->locals=realloc(f->locals,sizeof(Local)*f->local_cap);}
    f->locals[f->local_count].name=name; f->locals[f->local_count].val=val; f->local_count++;
}

static Local  *g_globals=NULL; static int g_global_count=0; static int g_global_cap=0;
static void globals_init(void){g_global_cap=64;g_global_count=0;g_globals=malloc(sizeof(Local)*g_global_cap);}
static Value *global_get(const char *name,size_t nlen){for(int i=0;i<g_global_count;i++)if(g_globals[i].name->len==nlen&&memcmp(g_globals[i].name->data,name,nlen)==0)return &g_globals[i].val;return NULL;}
static void global_set(WStr *name,Value val){Value *ex=global_get(name->data,name->len);if(ex){val_drop(ex);*ex=val;wstr_dec(name);return;}if(g_global_count>=g_global_cap){g_global_cap*=2;g_globals=realloc(g_globals,sizeof(Local)*g_global_cap);}g_globals[g_global_count].name=name;g_globals[g_global_count].val=val;g_global_count++;}

/* UTF-8 helpers */
static size_t utf8_len(const char *s,size_t len){size_t c=0,i=0;while(i<len){unsigned char b=(unsigned char)s[i];if(b<0x80)i++;else if(b<0xE0)i+=2;else if(b<0xF0)i+=3;else i+=4;c++;}return c;}
static bool utf8_nth(const char *s,size_t sl,size_t n,size_t *off,size_t *cl){size_t i=0,c=0;while(i<sl){if(c==n){*off=i;unsigned char b=(unsigned char)s[i];if(b<0x80)*cl=1;else if(b<0xE0)*cl=2;else if(b<0xF0)*cl=3;else*cl=4;return true;}unsigned char b=(unsigned char)s[i];if(b<0x80)i++;else if(b<0xE0)i+=2;else if(b<0xF0)i+=3;else i+=4;c++;}return false;}
static uint32_t utf8_decode(const char *s,size_t *adv){unsigned char c=(unsigned char)s[0];if(c<0x80){*adv=1;return c;}else if(c<0xE0){*adv=2;return((c&0x1F)<<6)|(s[1]&0x3F);}else if(c<0xF0){*adv=3;return((c&0x0F)<<12)|((s[1]&0x3F)<<6)|(s[2]&0x3F);}else{*adv=4;return((c&0x07)<<18)|((s[1]&0x3F)<<12)|((s[2]&0x3F)<<6)|(s[3]&0x3F);}}

static Program    g_prog;
static Value      g_stack[MAX_STACK];
static int        g_sp=0;
static CallFrame  g_frames[MAX_FRAMES];
static int        g_fp=0;
static int        g_argc=0;
static char     **g_argv=NULL;

static void  push(Value v) { if(g_sp>=MAX_STACK)die("stack overflow"); g_stack[g_sp++]=v; }
static Value pop(void)     { if(g_sp<=0)die("stack underflow"); return g_stack[--g_sp]; }
static Value *peek(void)   { if(g_sp<=0)die("stack underflow (peek)"); return &g_stack[g_sp-1]; }
static CallFrame *frame(void)    { return &g_frames[g_fp-1]; }
static Chunk     *cur_chunk(void){ return &g_prog.chunks[frame()->chunk_idx]; }

static uint8_t  frame_read_byte(void){ CallFrame *f=frame(); return cur_chunk()->code[f->ip++]; }
static uint16_t frame_read_u16(void) { CallFrame *f=frame(); Chunk *c=cur_chunk(); uint16_t hi=c->code[f->ip],lo=c->code[f->ip+1]; f->ip+=2; return(hi<<8)|lo; }
static const Value *const_val(uint8_t idx){ return &cur_chunk()->constants[idx]; }
static const char *const_str(uint8_t idx){ const Value *v=const_val(idx); if(v->tag!=V_STR)die("expected string constant"); return v->as.str->data; }
static size_t const_str_len(uint8_t idx){ const Value *v=const_val(idx); if(v->tag!=V_STR)die("expected string constant"); return v->as.str->len; }
static uint32_t current_line(void){ CallFrame *f=frame(); uint32_t ip=f->ip>0?f->ip-1:0; Chunk *c=cur_chunk(); return ip<c->lines_len?c->lines[ip]:0; }

static Value lookup_var(const char *name, size_t nlen) {
    Value *v=frame_get(frame(),name,nlen); if(v) return val_clone(v);
    v=global_get(name,nlen); if(v) return val_clone(v);
    die("line %u: undefined variable '%.*s'",current_line(),(int)nlen,name);
    return val_none();
}
static void store_var(const char *name, size_t nlen, Value val) {
    if(g_fp>1) { WStr *k=wstr_new(name,nlen); frame_set(frame(),k,val); }
    else { WStr *k=wstr_new(name,nlen); global_set(k,val); }
}

static bool call_builtin(const char *name, int argc, Value *args, Value *result);

static void execute(void) {
    for(;;) {
        uint8_t op=frame_read_byte();
        switch(op) {
        case OP_PUSH_CONST:{ uint8_t i=frame_read_byte(); push(val_clone(const_val(i))); break; }
        case OP_PUSH_TRUE:  push(val_bool(true));  break;
        case OP_PUSH_FALSE: push(val_bool(false)); break;
        case OP_PUSH_NONE:  push(val_none());      break;

        case OP_LOAD:{
            uint8_t i=frame_read_byte();
            push(lookup_var(const_str(i),const_str_len(i)));
            break;
        }
        case OP_LOAD_GLOBAL:{
            uint8_t i=frame_read_byte();
            const char *n=const_str(i); size_t nl=const_str_len(i);
            Value *v=global_get(n,nl);
            if(!v) die("line %u: undefined variable '%s'",current_line(),n);
            push(val_clone(v)); break;
        }
        case OP_STORE:{
            uint8_t i=frame_read_byte();
            Value val=pop();
            store_var(const_str(i),const_str_len(i),val);
            break;
        }

        case OP_LOAD_UPVALUE:{
            uint8_t slot=frame_read_byte();
            CallFrame *f=frame();
            if(slot>=(uint8_t)f->uv_count) die("line %u: LOAD_UPVALUE slot %u out of range",current_line(),slot);
            push(val_clone(&f->upvalues[slot]->val));
            break;
        }
        case OP_STORE_UPVALUE:{
            uint8_t slot=frame_read_byte();
            Value val=pop();
            CallFrame *f=frame();
            if(slot>=(uint8_t)f->uv_count) die("line %u: STORE_UPVALUE slot %u out of range",current_line(),slot);
            UpvalueCell *cell=f->upvalues[slot];
            val_drop(&cell->val);
            cell->val=val;
            const char *uv_name=NULL;
            for(int fi=0;fi<g_fp-1;fi++)
                for(int ui=0;ui<g_frames[fi].open_uv_count;ui++)
                    if(g_frames[fi].open_uv_cells[ui]==cell){
                        uv_name=g_frames[fi].open_uv_names[ui];
                        Value *lv=frame_get(&g_frames[fi],uv_name,strlen(uv_name));
                        if(lv){val_drop(lv);*lv=val_clone(&cell->val);}
                        break;
                    }
            break;
        }
        case OP_CLOSE_UPVALUE:{
            frame_read_byte(); /* slot — no-op: cells are eagerly closed at capture */
            break;
        }

        case OP_MAKE_CLOSURE:{
            uint8_t name_idx=frame_read_byte();
            uint8_t uv_count=frame_read_byte();
            const char *fn_name=const_str(name_idx);

            typedef struct { bool is_local; char name[256]; } UVDesc;
            UVDesc descs[256];
            for(int ui=0;ui<uv_count;ui++){
                descs[ui].is_local=(frame_read_byte()!=0);
                uint8_t nl=frame_read_byte();
                int copy=(nl<255?nl:255);
                for(int ni=0;ni<copy;ni++) descs[ui].name[ni]=(char)frame_read_byte();
                for(int ni=copy;ni<nl;ni++) frame_read_byte(); /* consume overflow */
                descs[ui].name[copy]='\0';
            }

            int ci=find_chunk(&g_prog,fn_name);
            if(ci<0) die("line %u: MAKE_CLOSURE: unknown function '%s'",current_line(),fn_name);

            Closure *cl=closure_new(ci,(uint8_t)uv_count);
            CallFrame *enc=frame();

            for(int ui=0;ui<uv_count;ui++){
                if(descs[ui].is_local){
                    UpvalueCell *cell=NULL;
                    for(int oi=0;oi<enc->open_uv_count;oi++){
                        if(strcmp(enc->open_uv_names[oi],descs[ui].name)==0){
                            cell=enc->open_uv_cells[oi]; break;
                        }
                    }
                    if(!cell){
                        Value *lv=frame_get(enc,descs[ui].name,strlen(descs[ui].name));
                        cell=uv_cell_new(lv?val_clone(lv):val_none());
                        if(enc->open_uv_count<MAX_OPEN_UPVALS){
                            uv_cell_inc(cell); /* frame holds a ref */
                            enc->open_uv_cells[enc->open_uv_count]=cell;
                            strncpy(enc->open_uv_names[enc->open_uv_count],descs[ui].name,255);
                            enc->open_uv_count++;
                        }
                    }
                    uv_cell_inc(cell);
                    cl->upvalues[ui]=cell;
                } else {
                    int pslot=(int)atoi(descs[ui].name);
                    UpvalueCell *cell=NULL;
                    if(pslot<enc->uv_count) cell=enc->upvalues[pslot];
                    if(!cell){ cell=uv_cell_new(val_none()); }
                    uv_cell_inc(cell);
                    cl->upvalues[ui]=cell;
                }
            }
            push(val_closure_own(cl));
            break;
        }

        case OP_ADD:{
            Value b=pop(),a=pop();
            if(a.tag==V_NUM&&b.tag==V_NUM){push(val_num(a.as.num+b.as.num));}
            else if(a.tag==V_STR&&b.tag==V_STR){WStr *r=wstr_cat(a.as.str,b.as.str);val_drop(&a);val_drop(&b);push(val_str_own(r));}
            else if(a.tag==V_STR){char *fb=val_format(&b);WStr *bs=wstr_from_cstr(fb);free(fb);WStr *r=wstr_cat(a.as.str,bs);wstr_dec(bs);val_drop(&a);val_drop(&b);push(val_str_own(r));}
            else if(b.tag==V_STR){char *fa=val_format(&a);WStr *as=wstr_from_cstr(fa);free(fa);WStr *r=wstr_cat(as,b.as.str);wstr_dec(as);val_drop(&a);val_drop(&b);push(val_str_own(r));}
            else die("line %u: cannot add %s and %s",current_line(),type_name(&a),type_name(&b));
            break;
        }
        case OP_SUB:{Value b=pop(),a=pop();if(a.tag!=V_NUM||b.tag!=V_NUM)die("cannot subtract");push(val_num(a.as.num-b.as.num));break;}
        case OP_MUL:{Value b=pop(),a=pop();if(a.tag!=V_NUM||b.tag!=V_NUM)die("cannot multiply");push(val_num(a.as.num*b.as.num));break;}
        case OP_DIV:{Value b=pop(),a=pop();if(a.tag!=V_NUM||b.tag!=V_NUM)die("cannot divide");if(b.as.num==0.0)die("division by zero");push(val_num(a.as.num/b.as.num));break;}
        case OP_MOD:{Value b=pop(),a=pop();if(a.tag!=V_NUM||b.tag!=V_NUM)die("cannot modulo");if(b.as.num==0.0)die("division by zero");push(val_num(fmod(a.as.num,b.as.num)));break;}
        case OP_NEG:{Value a=pop();if(a.tag!=V_NUM)die("cannot negate");push(val_num(-a.as.num));break;}

        case OP_EQ: {Value b=pop(),a=pop();bool r=false;if(a.tag==b.tag){if(a.tag==V_NUM)r=a.as.num==b.as.num;else if(a.tag==V_BOOL)r=a.as.boolean==b.as.boolean;else if(a.tag==V_STR)r=a.as.str->len==b.as.str->len&&memcmp(a.as.str->data,b.as.str->data,a.as.str->len)==0;else if(a.tag==V_NONE)r=true;}val_drop(&a);val_drop(&b);push(val_bool(r));break;}
        case OP_NEQ:{Value b=pop(),a=pop();bool r=true;if(a.tag==b.tag){if(a.tag==V_NUM)r=a.as.num!=b.as.num;else if(a.tag==V_BOOL)r=a.as.boolean!=b.as.boolean;else if(a.tag==V_STR)r=!(a.as.str->len==b.as.str->len&&memcmp(a.as.str->data,b.as.str->data,a.as.str->len)==0);else if(a.tag==V_NONE)r=false;}val_drop(&a);val_drop(&b);push(val_bool(r));break;}
        case OP_LT: {Value b=pop(),a=pop();if(a.tag==V_NUM&&b.tag==V_NUM)push(val_bool(a.as.num<b.as.num));else if(a.tag==V_STR&&b.tag==V_STR){bool r=strcmp(a.as.str->data,b.as.str->data)<0;val_drop(&a);val_drop(&b);push(val_bool(r));}else die("cannot compare");break;}
        case OP_LTE:{Value b=pop(),a=pop();if(a.tag==V_NUM&&b.tag==V_NUM)push(val_bool(a.as.num<=b.as.num));else if(a.tag==V_STR&&b.tag==V_STR){bool r=strcmp(a.as.str->data,b.as.str->data)<=0;val_drop(&a);val_drop(&b);push(val_bool(r));}else die("cannot compare");break;}
        case OP_GT: {Value b=pop(),a=pop();if(a.tag==V_NUM&&b.tag==V_NUM)push(val_bool(a.as.num>b.as.num));else if(a.tag==V_STR&&b.tag==V_STR){bool r=strcmp(a.as.str->data,b.as.str->data)>0;val_drop(&a);val_drop(&b);push(val_bool(r));}else die("cannot compare");break;}
        case OP_GTE:{Value b=pop(),a=pop();if(a.tag==V_NUM&&b.tag==V_NUM)push(val_bool(a.as.num>=b.as.num));else if(a.tag==V_STR&&b.tag==V_STR){bool r=strcmp(a.as.str->data,b.as.str->data)>=0;val_drop(&a);val_drop(&b);push(val_bool(r));}else die("cannot compare");break;}
        case OP_NOT:{Value a=pop();bool t=is_truthy(&a);val_drop(&a);push(val_bool(!t));break;}

        case OP_JUMP:           {uint16_t t=frame_read_u16();frame()->ip=t;break;}
        case OP_JUMP_IF_FALSE:  {uint16_t t=frame_read_u16();Value c=pop();if(!is_truthy(&c))frame()->ip=t;val_drop(&c);break;}
        case OP_JUMP_IF_TRUE:   {uint16_t t=frame_read_u16();Value c=pop();if(is_truthy(&c))frame()->ip=t;val_drop(&c);break;}
        case OP_PEEK_JUMP_FALSE:{uint16_t t=frame_read_u16();if(!is_truthy(peek()))frame()->ip=t;break;}
        case OP_PEEK_JUMP_TRUE: {uint16_t t=frame_read_u16();if(is_truthy(peek()))frame()->ip=t;break;}

        case OP_CALL:{
            uint8_t name_idx=frame_read_byte();
            uint8_t arg_count=frame_read_byte();
            const char *name=const_str(name_idx);

            Value args_buf[256];
            for(int i=arg_count-1;i>=0;i--) args_buf[i]=pop();

            if(strcmp(name,"__callee__")==0){
                Value callee=pop();
                if(callee.tag==V_CLOSURE){
                    Closure *cl=callee.as.closure;
                    Chunk *fn=&g_prog.chunks[cl->chunk_idx];
                    if((int)arg_count!=fn->param_count)
                        die("line %u: %s() expected %d arguments, got %d",current_line(),fn->name,fn->param_count,arg_count);
                    UpvalueCell **uvs=NULL;
                    if(cl->uv_count>0){
                        uvs=malloc(sizeof(UpvalueCell*)*cl->uv_count);
                        for(int i=0;i<cl->uv_count;i++){uv_cell_inc(cl->upvalues[i]);uvs[i]=cl->upvalues[i];}
                    }
                    if(g_fp>=MAX_FRAMES) die("call stack overflow");
                    frame_init(&g_frames[g_fp],cl->chunk_idx,uvs,cl->uv_count); g_fp++;
                    for(int i=0;i<arg_count;i++) push(args_buf[i]);
                    val_drop(&callee);
                } else {
                    die("line %u: cannot call %s",current_line(),type_name(&callee));
                }
                break;
            }

            Value br;
            if(call_builtin(name,arg_count,args_buf,&br)){
                for(int i=0;i<arg_count;i++) val_drop(&args_buf[i]);
                push(br); break;
            }

            Value *cv=frame_get(frame(),name,strlen(name));
            if(!cv) cv=global_get(name,strlen(name));
            if(cv&&cv->tag==V_CLOSURE){
                Closure *cl=cv->as.closure;
                Chunk *fn=&g_prog.chunks[cl->chunk_idx];
                if((int)arg_count!=fn->param_count)
                    die("line %u: %s() expected %d arguments, got %d",current_line(),name,fn->param_count,arg_count);
                UpvalueCell **uvs=NULL;
                if(cl->uv_count>0){
                    uvs=malloc(sizeof(UpvalueCell*)*cl->uv_count);
                    for(int i=0;i<cl->uv_count;i++){uv_cell_inc(cl->upvalues[i]);uvs[i]=cl->upvalues[i];}
                }
                if(g_fp>=MAX_FRAMES) die("call stack overflow");
                frame_init(&g_frames[g_fp],cl->chunk_idx,uvs,cl->uv_count); g_fp++;
                for(int i=0;i<arg_count;i++) push(args_buf[i]);
                break;
            }

            int ci=find_chunk(&g_prog,name);
            if(ci<0) die("line %u: undefined function '%s'",current_line(),name);
            Chunk *fn=&g_prog.chunks[ci];
            if((int)arg_count!=fn->param_count)
                die("line %u: %s() expected %d arguments, got %d",current_line(),name,fn->param_count,arg_count);
            if(g_fp>=MAX_FRAMES) die("call stack overflow");
            frame_init(&g_frames[g_fp],ci,NULL,0); g_fp++;
            for(int i=0;i<arg_count;i++) push(args_buf[i]);
            break;
        }

        case OP_RETURN:     {Value val=pop();g_fp--;frame_free(&g_frames[g_fp]);push(val);break;}
        case OP_RETURN_NONE:{           g_fp--;frame_free(&g_frames[g_fp]);push(val_none());break;}

        case OP_MAKE_ARRAY:{
            uint8_t n=frame_read_byte(); WArray *a=warray_new(n);
            Value tmp[256]; for(int i=n-1;i>=0;i--) tmp[i]=pop();
            for(int i=0;i<n;i++) warray_push(a,tmp[i]);
            push(val_array_own(a)); break;
        }
        case OP_MAKE_DICT:{
            uint8_t n=frame_read_byte(); WDict *d=wdict_new(n);
            typedef struct{Value k;Value v;}KV; KV pairs[128];
            for(int i=n-1;i>=0;i--){pairs[i].v=pop();pairs[i].k=pop();}
            for(int i=0;i<n;i++){WStr *k=to_dict_key(&pairs[i].k);val_drop(&pairs[i].k);wdict_set(d,k,pairs[i].v);}
            push(val_dict_own(d)); break;
        }
        case OP_GET_INDEX:{
            Value idx=pop(),obj=pop();
            if(obj.tag==V_ARRAY){if(idx.tag!=V_NUM)die("array index must be number");size_t i=(size_t)idx.as.num;if(i>=obj.as.array->len)die("line %u: index %zu out of bounds (len %zu)",current_line(),i,obj.as.array->len);push(val_clone(&obj.as.array->items[i]));val_drop(&obj);}
            else if(obj.tag==V_DICT){WStr *k=to_dict_key(&idx);const Value *v=wdict_get(obj.as.dict,k->data,k->len);if(!v)die("line %u: key \"%s\" not found in dict",current_line(),k->data);push(val_clone(v));wstr_dec(k);val_drop(&obj);}
            else die("cannot index %s",type_name(&obj));
            val_drop(&idx); break;
        }
        case OP_SET_INDEX:{
            Value nv=pop(),idx=pop(),obj=pop();
            if(obj.tag==V_ARRAY){if(idx.tag!=V_NUM)die("array index must be number");size_t i=(size_t)idx.as.num;WArray *a=obj.as.array;if(i>=a->len)die("index out of bounds");a=warray_cow(a);val_drop(&a->items[i]);a->items[i]=nv;obj.as.array=a;push(obj);}
            else if(obj.tag==V_DICT){WStr *k=to_dict_key(&idx);WDict *d=wdict_cow(obj.as.dict);wdict_set(d,k,nv);obj.as.dict=d;push(obj);}
            else die("cannot set index on %s",type_name(&obj));
            val_drop(&idx); break;
        }
        case OP_PRINT:{Value v=pop();char *s=val_format(&v);printf("%s\n",s);free(s);val_drop(&v);break;}
        case OP_POP:  {Value v=pop();val_drop(&v);break;}
        case OP_HALT: {g_fp--;frame_free(&g_frames[g_fp]);return;}
        default: die("unknown opcode 0x%02x at line %u",op,current_line());
        }
    }
}

static bool call_builtin(const char *name, int argc, Value *args, Value *result) {
    if(strcmp(name,"length")==0){
        if(argc!=1)die("length() takes 1 argument");
        if(args[0].tag==V_ARRAY)*result=val_num((double)args[0].as.array->len);
        else if(args[0].tag==V_STR)*result=val_num((double)utf8_len(args[0].as.str->data,args[0].as.str->len));
        else if(args[0].tag==V_DICT)*result=val_num((double)args[0].as.dict->len);
        else die("length() expects array, string, or dict");
        return true;
    }
    if(strcmp(name,"push")==0){
        if(argc!=2)die("push() takes 2 arguments");
        if(args[0].tag!=V_ARRAY)die("push() expects array");
        WArray *a=args[0].as.array;warray_inc(a);a=warray_cow(a);warray_push(a,val_clone(&args[1]));*result=val_array_own(a);return true;
    }
    if(strcmp(name,"pop")==0){
        if(argc!=1)die("pop() takes 1 argument");
        if(args[0].tag!=V_ARRAY)die("pop() expects array");
        WArray *a=args[0].as.array;if(a->len==0)die("pop() on empty array");
        *result=val_clone(&a->items[a->len-1]);return true;
    }
    if(strcmp(name,"reverse")==0){
        if(argc!=1)die("reverse() takes 1 argument");
        if(args[0].tag!=V_ARRAY)die("reverse() expects array");
        WArray *a=args[0].as.array;warray_inc(a);a=warray_cow(a);
        if(a->len>1)for(size_t i=0,j=a->len-1;i<j;i++,j--){Value t=a->items[i];a->items[i]=a->items[j];a->items[j]=t;}
        *result=val_array_own(a);return true;
    }
    if(strcmp(name,"slice")==0){
        if(argc!=3)die("slice() takes 3 arguments");
        if(args[0].tag!=V_ARRAY)die("slice() expects array");
        size_t s=(size_t)args[1].as.num,e=(size_t)args[2].as.num;
        WArray *src=args[0].as.array;
        if(s>e)die("invalid slice");
        if(e>src->len)die("slice out of bounds");
        WArray *a=warray_new(e-s);for(size_t i=s;i<e;i++)warray_push(a,val_clone(&src->items[i]));
        *result=val_array_own(a);return true;
    }
    if(strcmp(name,"range")==0){
        if(argc!=2)die("range() takes 2 arguments");
        int64_t s=(int64_t)args[0].as.num,e=(int64_t)args[1].as.num;
        size_t n=(e>s)?(size_t)(e-s):0;
        WArray *a=warray_new(n);for(int64_t i=s;i<e;i++)warray_push(a,val_num((double)i));
        *result=val_array_own(a);return true;
    }
    if(strcmp(name,"keys")==0){
        if(argc!=1)die("keys() takes 1 argument");
        if(args[0].tag!=V_DICT)die("keys() expects dict");
        WDict *d=args[0].as.dict;
        WArray *a=warray_new(d->len);
        size_t *idx=malloc(sizeof(size_t)*d->len);
        for(size_t i=0;i<d->len;i++) idx[i]=i;
        for(size_t i=1;i<d->len;i++){size_t t=idx[i],j=i;while(j>0&&strcmp(d->keys[idx[j-1]]->data,d->keys[t]->data)>0){idx[j]=idx[j-1];j--;}idx[j]=t;}
        for(size_t i=0;i<d->len;i++) warray_push(a,val_str(d->keys[idx[i]]));
        free(idx);*result=val_array_own(a);return true;
    }
    if(strcmp(name,"values")==0){
        if(argc!=1)die("values() takes 1 argument");
        if(args[0].tag!=V_DICT)die("values() expects dict");
        WDict *d=args[0].as.dict;
        size_t *idx=malloc(sizeof(size_t)*d->len);
        for(size_t i=0;i<d->len;i++) idx[i]=i;
        for(size_t i=1;i<d->len;i++){size_t t=idx[i],j=i;while(j>0&&strcmp(d->keys[idx[j-1]]->data,d->keys[t]->data)>0){idx[j]=idx[j-1];j--;}idx[j]=t;}
        WArray *a=warray_new(d->len);for(size_t i=0;i<d->len;i++)warray_push(a,val_clone(&d->vals[idx[i]]));
        free(idx);*result=val_array_own(a);return true;
    }
    if(strcmp(name,"has_key")==0){
        if(argc!=2)die("has_key() takes 2 arguments");
        if(args[0].tag!=V_DICT)die("has_key() expects dict");
        WStr *k=to_dict_key(&args[1]);bool f=wdict_find(args[0].as.dict,k->data,k->len)!=(size_t)-1;wstr_dec(k);*result=val_bool(f);return true;
    }
    if(strcmp(name,"char_at")==0){
        if(argc!=2)die("char_at() takes 2 arguments");
        if(args[0].tag!=V_STR||args[1].tag!=V_NUM)die("char_at() expects (string, number)");
        size_t i=(size_t)args[1].as.num,off,cl;
        if(!utf8_nth(args[0].as.str->data,args[0].as.str->len,i,&off,&cl))die("char_at: index out of bounds");
        *result=val_str_own(wstr_new(args[0].as.str->data+off,cl));return true;
    }
    if(strcmp(name,"substr")==0){
        if(argc!=3)die("substr() takes 3 arguments");
        if(args[0].tag!=V_STR)die("substr() expects string");
        size_t st=(size_t)args[1].as.num,ln=(size_t)args[2].as.num;
        const char *s=args[0].as.str->data;size_t sl=args[0].as.str->len;
        size_t cc=utf8_len(s,sl);
        if(st>cc)st=cc;
        size_t en=st+ln;if(en>cc)en=cc;
        size_t bs=0,be=0,ci=0,bi=0;
        while(bi<sl&&ci<en){if(ci==st)bs=bi;unsigned char c=(unsigned char)s[bi];if(c<0x80)bi++;else if(c<0xE0)bi+=2;else if(c<0xF0)bi+=3;else bi+=4;ci++;}
        if(ci==st)bs=bi;
        be=bi;
        *result=val_str_own(wstr_new(s+bs,be-bs));return true;
    }
    if(strcmp(name,"ord")==0){
        if(argc!=1)die("ord() takes 1 argument");
        if(args[0].tag!=V_STR)die("ord() expects string");
        if(args[0].as.str->len==0)die("ord() on empty string");
        size_t adv;uint32_t cp=utf8_decode(args[0].as.str->data,&adv);
        *result=val_num((double)cp);return true;
    }
    if(strcmp(name,"num_to_str")==0){
        if(argc!=1)die("num_to_str() takes 1 argument");
        if(args[0].tag!=V_NUM)die("num_to_str() expects number");
        char *s=fmt_number(args[0].as.num);*result=val_str_own(wstr_from_cstr(s));free(s);return true;
    }
    if(strcmp(name,"str_to_num")==0){
        if(argc!=1)die("str_to_num() takes 1 argument");
        if(args[0].tag!=V_STR)die("str_to_num() expects string");
        const char *s=args[0].as.str->data;while(*s&&isspace((unsigned char)*s))s++;
        char *end;double n=strtod(s,&end);while(*end&&isspace((unsigned char)*end))end++;
        if(*end!='\0')die("str_to_num: cannot parse \"%s\"",args[0].as.str->data);
        *result=val_num(n);return true;
    }
    if(strcmp(name,"input")==0){
        if(argc>1)die("input() takes 0 or 1 arguments");
        if(argc==1&&args[0].tag==V_STR&&args[0].as.str->len>0){printf("%s",args[0].as.str->data);fflush(stdout);}
        char buf[4096];if(!fgets(buf,sizeof(buf),stdin))buf[0]='\0';
        size_t l=strlen(buf);while(l>0&&(buf[l-1]=='\n'||buf[l-1]=='\r'))l--;buf[l]='\0';
        *result=val_str_own(wstr_from_cstr(buf));return true;
    }
    if(strcmp(name,"read_file")==0){
        if(argc!=1)die("read_file() takes 1 argument");
        if(args[0].tag!=V_STR)die("read_file() expects string");
        FILE *fp=fopen(args[0].as.str->data,"rb");if(!fp)die("cannot open '%s'",args[0].as.str->data);
        fseek(fp,0,SEEK_END);long sz=ftell(fp);fseek(fp,0,SEEK_SET);
        char *c=malloc(sz+1);if((long)fread(c,1,sz,fp)!=sz)die("read failed");c[sz]='\0';fclose(fp);
        *result=val_str_own(wstr_new(c,sz));free(c);return true;
    }
    if(strcmp(name,"write_file")==0){
        if(argc!=2)die("write_file() takes 2 arguments");
        if(args[0].tag!=V_STR)die("write_file() expects string path");
        char *c=val_format(&args[1]);
        FILE *fp=fopen(args[0].as.str->data,"wb");if(!fp)die("cannot open '%s'",args[0].as.str->data);
        fwrite(c,1,strlen(c),fp);fclose(fp);free(c);*result=val_none();return true;
    }
    if(strcmp(name,"args")==0){
        if(argc!=0)die("args() takes 0 arguments");
        WArray *a=warray_new(g_argc);for(int i=0;i<g_argc;i++)warray_push(a,val_str_own(wstr_from_cstr(g_argv[i])));
        *result=val_array_own(a);return true;
    }
    if(strcmp(name,"num_to_hex")==0){
        if(argc!=1)die("num_to_hex() takes 1 argument");
        if(args[0].tag!=V_NUM)die("num_to_hex() expects number");
        uint64_t bits;double n=args[0].as.num;memcpy(&bits,&n,8);
        char buf[17];snprintf(buf,sizeof(buf),"%016llx",(unsigned long long)bits);
        *result=val_str_own(wstr_from_cstr(buf));return true;
    }
    if(strcmp(name,"write_hex")==0){
        if(argc!=2)die("write_hex() takes 2 arguments");
        if(args[0].tag!=V_STR)die("write_hex() path must be string");
        if(args[1].tag!=V_STR)die("write_hex() hex must be string");
        const char *hex=args[1].as.str->data;size_t hl=args[1].as.str->len;size_t nb=hl/2;
        uint8_t *bytes=malloc(nb);
        for(size_t i=0;i<nb;i++){char t[3]={hex[i*2],hex[i*2+1],'\0'};bytes[i]=(uint8_t)strtoul(t,NULL,16);}
        FILE *fp=fopen(args[0].as.str->data,"wb");if(!fp)die("cannot open '%s'",args[0].as.str->data);
        fwrite(bytes,1,nb,fp);fclose(fp);free(bytes);*result=val_none();return true;
    }
    if(strcmp(name,"type_of")==0){
        if(argc!=1)die("type_of() takes 1 argument");
        *result=val_str_own(wstr_from_cstr(type_name(&args[0])));return true;
    }
    if(strcmp(name,"assert")==0){
        if(argc<1||argc>2)die("assert() takes 1 or 2 arguments");
        if(!is_truthy(&args[0])){
            const char *msg=(argc==2&&args[1].tag==V_STR)?args[1].as.str->data:"assertion failed";
            die("line %u: Assertion failed: %s",current_line(),msg);
        }
        *result=val_none();return true;
    }
    if(strcmp(name,"exit")==0){
        if(argc>1)die("exit() takes 0 or 1 arguments");
        int code=(argc==1&&args[0].tag==V_NUM)?(int)args[0].as.num:0;
        exit(code);
    }
    return false;
}

static const char *opcode_name(uint8_t op) {
    switch(op){
    case OP_PUSH_CONST:     return "PUSH_CONST";
    case OP_PUSH_TRUE:      return "PUSH_TRUE";
    case OP_PUSH_FALSE:     return "PUSH_FALSE";
    case OP_PUSH_NONE:      return "PUSH_NONE";
    case OP_LOAD:           return "LOAD";
    case OP_STORE:          return "STORE";
    case OP_LOAD_GLOBAL:    return "LOAD_GLOBAL";
    case OP_LOAD_UPVALUE:   return "LOAD_UPVALUE";
    case OP_STORE_UPVALUE:  return "STORE_UPVALUE";
    case OP_CLOSE_UPVALUE:  return "CLOSE_UPVALUE";
    case OP_ADD:            return "ADD";
    case OP_SUB:            return "SUB";
    case OP_MUL:            return "MUL";
    case OP_DIV:            return "DIV";
    case OP_MOD:            return "MOD";
    case OP_NEG:            return "NEG";
    case OP_EQ:             return "EQ";
    case OP_NEQ:            return "NEQ";
    case OP_LT:             return "LT";
    case OP_LTE:            return "LTE";
    case OP_GT:             return "GT";
    case OP_GTE:            return "GTE";
    case OP_NOT:            return "NOT";
    case OP_JUMP:           return "JUMP";
    case OP_JUMP_IF_FALSE:  return "JUMP_IF_FALSE";
    case OP_JUMP_IF_TRUE:   return "JUMP_IF_TRUE";
    case OP_PEEK_JUMP_FALSE:return "PEEK_JUMP_IF_FALSE";
    case OP_PEEK_JUMP_TRUE: return "PEEK_JUMP_IF_TRUE";
    case OP_CALL:           return "CALL";
    case OP_RETURN:         return "RETURN";
    case OP_RETURN_NONE:    return "RETURN_NONE";
    case OP_MAKE_CLOSURE:   return "MAKE_CLOSURE";
    case OP_MAKE_ARRAY:     return "MAKE_ARRAY";
    case OP_MAKE_DICT:      return "MAKE_DICT";
    case OP_GET_INDEX:      return "GET_INDEX";
    case OP_SET_INDEX:      return "SET_INDEX";
    case OP_PRINT:          return "PRINT";
    case OP_POP:            return "POP";
    case OP_HALT:           return "HALT";
    default:                return NULL;
    }
}
static int operand_size(uint8_t op){
    switch(op){
    case OP_PUSH_CONST:case OP_LOAD:case OP_LOAD_GLOBAL:case OP_STORE:
    case OP_LOAD_UPVALUE:case OP_STORE_UPVALUE:case OP_CLOSE_UPVALUE:
    case OP_MAKE_ARRAY:case OP_MAKE_DICT: return 1;
    case OP_JUMP:case OP_JUMP_IF_FALSE:case OP_JUMP_IF_TRUE:
    case OP_PEEK_JUMP_FALSE:case OP_PEEK_JUMP_TRUE:case OP_CALL: return 2;
    case OP_MAKE_CLOSURE: return -1; /* variable */
    default: return 0;
    }
}

static char *const_annotation(const Chunk *c, int idx){
    if(idx>=c->const_count) return strdup("(out of range)");
    const Value *v=&c->constants[idx]; char buf[256];
    switch(v->tag){
    case V_STR: snprintf(buf,sizeof(buf),"'%s'",v->as.str->data); break;
    case V_NUM: if(v->as.num==(double)(int64_t)v->as.num&&fabs(v->as.num)<1e15)snprintf(buf,sizeof(buf),"'%lld'",(long long)(int64_t)v->as.num);else snprintf(buf,sizeof(buf),"'%g'",v->as.num); break;
    case V_BOOL:snprintf(buf,sizeof(buf),"'%s'",v->as.boolean?"true":"false"); break;
    case V_NONE:return strdup("'none'");
    case V_ARRAY:return strdup("[array]");
    case V_DICT: return strdup("{dict}");
    case V_CLOSURE:return strdup("<closure>");
    }
    return strdup(buf);
}

static void disassemble_chunk(const Chunk *c){
    printf("== %s ==\n",c->name);
    uint32_t offset=0;
    while(offset<c->code_len){
        printf("%04u  ",offset);
        uint32_t line=(offset<c->lines_len)?c->lines[offset]:0;
        if(offset>0&&offset<c->lines_len&&c->lines[offset-1]==line) printf("   |  "); else printf("%4u  ",line);
        uint8_t op=c->code[offset]; const char *name=opcode_name(op);
        if(!name){printf("UNKNOWN(0x%02x)\n",op);offset++;continue;}
        int opsz=operand_size(op);
        if(opsz==0){printf("%s\n",name);offset++;}
        else if(opsz==1){uint8_t i=c->code[offset+1];char *a=const_annotation(c,i);printf("%-20s %3u    %s\n",name,i,a);free(a);offset+=2;}
        else if(opsz==-1){
            uint8_t ni=c->code[offset+1],uvc=c->code[offset+2];
            char *a=const_annotation(c,ni);
            printf("%-20s %3u    %s (%u upvalues)\n",name,ni,a,uvc);free(a);
            uint32_t pos=offset+3;
            for(int ui=0;ui<uvc&&pos<c->code_len;ui++){
                bool isl=(c->code[pos++]!=0);
                if(pos<c->code_len){
                    uint8_t nl=c->code[pos++];
                    char nm[257]={0};
                    for(int ni2=0;ni2<nl&&pos<c->code_len;ni2++) nm[ni2]=(char)c->code[pos++];
                    printf("              [is_local=%d name='%s']\n",(int)isl,nm);
                }
            }
            offset=pos;
        }
        else{
            switch(op){
            case OP_JUMP:case OP_JUMP_IF_FALSE:case OP_JUMP_IF_TRUE:case OP_PEEK_JUMP_FALSE:case OP_PEEK_JUMP_TRUE:{uint16_t t=((uint16_t)c->code[offset+1]<<8)|c->code[offset+2];printf("%-20s        -> %04u\n",name,t);break;}
            case OP_CALL:{uint8_t ni=c->code[offset+1],ac=c->code[offset+2];char *a=const_annotation(c,ni);printf("%-20s %3u    %s (%u args)\n",name,ni,a,ac);free(a);break;}
            default:printf("%-20s 0x%02x 0x%02x\n",name,c->code[offset+1],c->code[offset+2]);break;
            }
            offset+=3;
        }
    }
}
static void disassemble_all(void){
    disassemble_chunk(&g_prog.chunks[0]);
    for(int i=1;i<g_prog.chunk_count;i++){printf("\n");disassemble_chunk(&g_prog.chunks[i]);}
}

#ifndef _WIN32
#include <unistd.h>
#include <sys/wait.h>
#endif

static const char *find_compiler(const char *self_path){
    static char buf[4096];
    const char *sl=strrchr(self_path,'/');
    if(sl){size_t dl=(size_t)(sl-self_path);snprintf(buf,sizeof(buf),"%.*s/compiler/wsc.whbc",(int)dl,self_path);FILE *f=fopen(buf,"rb");if(f){fclose(f);return buf;}}
    {FILE *f=fopen("compiler/wsc.whbc","rb");if(f){fclose(f);return "compiler/wsc.whbc";}}
    return NULL;
}

static void run_repl(const char *self_path){
    const char *wsc=find_compiler(self_path);
    if(!wsc){fprintf(stderr,"Cannot find compiler/wsc.whbc.\nRun from the Whispem project root.\n");exit(1);}
    printf("Whispem v5.0.0 — REPL (C VM)\n");
    printf("Type 'exit' or press Ctrl-D to quit.\n\n");
    size_t sc=4096,sl=0;char *source=malloc(sc);source[0]='\0';
    char tmp_wsp[256],tmp_whbc[256];
    snprintf(tmp_wsp, sizeof(tmp_wsp), "/tmp/wvm_repl_%d.wsp", (int)getpid());
    snprintf(tmp_whbc,sizeof(tmp_whbc),"/tmp/wvm_repl_%d.whbc",(int)getpid());
    for(;;){
        printf(">>> ");fflush(stdout);
        char line[4096];if(!fgets(line,sizeof(line),stdin)){printf("\n");break;}
        char *t=line;while(*t==' '||*t=='\t')t++;
        char *e=t+strlen(t)-1;while(e>=t&&(*e=='\n'||*e=='\r'||*e==' '))e--;*(e+1)='\0';
        if(strcmp(t,"exit")==0||strcmp(t,"quit")==0) break;
        if(*t=='\0') continue;
        char block[16384];snprintf(block,sizeof(block),"%s\n",line);
        if(e>=t&&*e=='{'){for(;;){printf("... ");fflush(stdout);char co[4096];if(!fgets(co,sizeof(co),stdin))break;strncat(block,co,sizeof(block)-strlen(block)-1);char *ct=co;while(*ct==' '||*ct=='\t')ct++;char *ce=ct+strlen(ct)-1;while(ce>=ct&&(*ce=='\n'||*ce=='\r'||*ce==' '))ce--;*(ce+1)='\0';if(strcmp(ct,"}")==0)break;}}
        size_t bl=strlen(block);
        while(sl+bl+1>sc){sc*=2;source=realloc(source,sc);}
        memcpy(source+sl,block,bl);sl+=bl;source[sl]='\0';
        FILE *wsp=fopen(tmp_wsp,"w");if(!wsp){fprintf(stderr,"Cannot write temp file\n");continue;}fputs(source,wsp);fclose(wsp);
        char cmd[8192];snprintf(cmd,sizeof(cmd),"\"%s\" \"%s\" \"%s\" > /dev/null 2>&1",self_path,wsc,tmp_wsp);
        if(system(cmd)!=0){fprintf(stderr,"Compile error.\n");sl-=bl;source[sl]='\0';continue;}
#ifndef _WIN32
        pid_t pid=fork();
        if(pid==0){
            FILE *fp=fopen(tmp_whbc,"rb");if(!fp)_exit(1);
            fseek(fp,0,SEEK_END);long sz=ftell(fp);fseek(fp,0,SEEK_SET);
            uint8_t *data=malloc(sz);if((long)fread(data,1,sz,fp)!=sz)_exit(1);fclose(fp);
            g_prog=load_program(data,sz);free(data);
            g_argc=0;g_argv=NULL;globals_init();
            frame_init(&g_frames[0],0,NULL,0);g_fp=1;g_sp=0;execute();_exit(0);
        } else if(pid>0){int st;waitpid(pid,&st,0);}
        else perror("fork");
#else
        snprintf(cmd,sizeof(cmd),"\"%s\" \"%s\"",self_path,tmp_whbc);system(cmd);
#endif
    }
    unlink(tmp_wsp);unlink(tmp_whbc);free(source);
    printf("Bye!\n");
}

int main(int argc, char **argv){
    if(argc<2){run_repl(argv[0]);return 0;}
    int argi=1;bool dump=false;
    if(strcmp(argv[argi],"--dump")==0){dump=true;argi++;if(argi>=argc){fprintf(stderr,"Usage: wvm --dump <file.whbc>\n");return 1;}}
    const char *path=argv[argi];
    FILE *fp=fopen(path,"rb");if(!fp)die("cannot open '%s'",path);
    fseek(fp,0,SEEK_END);long sz=ftell(fp);fseek(fp,0,SEEK_SET);
    uint8_t *data=malloc(sz);if((long)fread(data,1,sz,fp)!=sz)die("failed to read '%s'",path);fclose(fp);
    g_prog=load_program(data,sz);free(data);
    if(dump){disassemble_all();return 0;}
    g_argc=argc-argi-1;g_argv=argv+argi+1;
    globals_init();frame_init(&g_frames[0],0,NULL,0);g_fp=1;
    execute();
    return 0;
}