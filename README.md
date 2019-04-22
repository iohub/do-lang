
# Dolang                                                                         
a toy programming language

## example
* code
```rust
fn foo1(a: int, b: int) -> int {
    let c = a + 1001;
    let d: int;
    let ok = 123.456;
    if ok > 100.123 {
        let val = 123.24;
        d = b + 1992 + c + a;
        val = val + 0.87;
    }
    if c > 100 {
        let bv = 1002;
        c = bv + c;
    }
    a = c + d;
    return a;
}

fn foo2(a: int) -> bool {
    return a == 100;
}

fn fact(n: int) -> int {
    if n == 1 { return 1; }
    else { return fact(n - 1) * n; }
}


fn main() -> int {
    let a = 1001 + 92;
    let b: int;
    a = foo1(a, 100) + 123 + foo1(a, 12);
    b = foo1(123, a + 120);
    while a > b + 100 {
        b = a + foo1(a, b);
    }
    return 0;
}
```

* LLVM IR
```llvm
; ModuleID = '__module'
source_filename = "__module"

define i64 @foo1(i64, i64) {
entry:
  %a = alloca i64
  store i64 %0, i64* %a
  %b = alloca i64
  store i64 %1, i64* %b
  %c = alloca i64
  %2 = load i64, i64* %a
  %3 = add i64 %2, 1001
  store i64 %3, i64* %c
  %d = alloca i64
  %ok = alloca float
  store float 0x405EDD2F20000000, float* %ok
  %4 = load float, float* %ok
  %5 = fcmp ogt float %4, 0x405907DF40000000
  br i1 %5, label %"if:then", label %"if:else"

"if:then":                                        ; preds = %entry
  %val = alloca float
  store float 0x405ECF5C20000000, float* %val
  %6 = load i64, i64* %b
  %7 = add i64 %6, 1992
  %8 = load i64, i64* %c
  %9 = add i64 %7, %8
  %10 = load i64, i64* %a
  %11 = add i64 %9, %10
  store i64 %11, i64* %d
  %12 = load float, float* %val
  %13 = fadd float %12, 0x3FEBD70A40000000
  store float %13, float* %val
  br label %"if:merge"

"if:else":                                        ; preds = %entry
  br label %"if:merge"

"if:merge":                                       ; preds = %"if:else", %"if:then"
  %14 = load i64, i64* %c
  %15 = icmp sgt i64 %14, 100
  br i1 %15, label %"if:then1", label %"if:else2"

"if:then1":                                       ; preds = %"if:merge"
  %bv = alloca i64
  store i64 1002, i64* %bv
  %16 = load i64, i64* %bv
  %17 = load i64, i64* %c
  %18 = add i64 %16, %17
  store i64 %18, i64* %c
  br label %"if:merge3"

"if:else2":                                       ; preds = %"if:merge"
  br label %"if:merge3"

"if:merge3":                                      ; preds = %"if:else2", %"if:then1"
  %19 = load i64, i64* %c
  %20 = load i64, i64* %d
  %21 = add i64 %19, %20
  store i64 %21, i64* %a
  %22 = load i64, i64* %a
  ret i64 %22
}

define i1 @foo2(i64) {
entry:
  %a = alloca i64
  store i64 %0, i64* %a
  %1 = load i64, i64* %a
  %2 = icmp eq i64 %1, 100
  ret i1 %2
}

define i64 @fact(i64) {
entry:
  %n = alloca i64
  store i64 %0, i64* %n
  %1 = load i64, i64* %n
  %2 = icmp eq i64 %1, 1
  br i1 %2, label %"if:then", label %"if:else"

"if:then":                                        ; preds = %entry
  ret i64 1

"if:else":                                        ; preds = %entry
  %3 = load i64, i64* %n
  %4 = sub i64 %3, 1
  %5 = call i64 @fact(i64 %4)
  %6 = load i64, i64* %n
  %7 = mul i64 %5, %6
  ret i64 %7
}

define i64 @main() {
entry:
  %a = alloca i64
  store i64 1093, i64* %a
  %b = alloca i64
  %0 = load i64, i64* %a
  %1 = call i64 @foo1(i64 %0, i64 100)
  %2 = add i64 %1, 123
  %3 = load i64, i64* %a
  %4 = call i64 @foo1(i64 %3, i64 12)
  %5 = add i64 %2, %4
  store i64 %5, i64* %a
  %6 = load i64, i64* %a
  %7 = add i64 %6, 120
  %8 = call i64 @foo1(i64 123, i64 %7)
  store i64 %8, i64* %b
  br label %"while:cond"

"while:cond":                                     ; preds = %"while:body", %entry
  %9 = load i64, i64* %b
  %10 = add i64 %9, 100
  %11 = load i64, i64* %a
  %12 = icmp sgt i64 %11, %10
  br i1 %12, label %"while:body", label %"while:merge"

"while:body":                                     ; preds = %"while:cond"
  %13 = load i64, i64* %a
  %14 = load i64, i64* %b
  %15 = call i64 @foo1(i64 %13, i64 %14)
  %16 = load i64, i64* %a
  %17 = add i64 %16, %15
  store i64 %17, i64* %b
  br label %"while:cond"

"while:merge":                                    ; preds = %"while:cond"
  ret i64 0
}
```
                                       
