; ModuleID = '__module'

define i64 @"foo1\8A\7F"(i64, i64) {
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
  br i1 %5, label %if-then, label %if-else

if-then:                                          ; preds = %entry
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
  %13 = add float %12, 0x3FEBD70A40000000
  store float %13, float* %val

if-else:                                          ; preds = %entry
  %14 = load i64, i64* %c
  %15 = icmp sgt i64 %14, 100
  br i1 %15, label %if-then1, label %if-else2

if-then1:                                         ; preds = %if-else
  %bv = alloca i64
  store i64 1002, i64* %bv
  %16 = load i64, i64* %bv
  %17 = load i64, i64* %c
  %18 = add i64 %16, %17
  store i64 %18, i64* %c

if-else2:                                         ; preds = %if-else
  %19 = load i64, i64* %c
  %20 = load i64, i64* %d
  %21 = add i64 %19, %20
  store i64 %21, i64* %a
  ret i64* %a
}

define i8 @foo24V(i64) {
entry:
  %a = alloca i64
  store i64 %0, i64* %a
  %1 = load i64, i64* %a
  %2 = icmp eq i64 %1, 100
  ret i1 %2
}

define i64 @"fact\8A\7F"(i64) {
entry:
  %n = alloca i64
  store i64 %0, i64* %n
  %1 = load i64, i64* %n
  %2 = icmp eq i64 %1, 1
  br i1 %2, label %if-then, label %if-else

if-then:                                          ; preds = %entry
  ret i64 1

if-else:                                          ; preds = %entry
  %3 = load i64, i64* %n
  %4 = sub i64 %3, 1
  %5 = call i64 @"fact\8A\7F"(i64 %4)
  %6 = load i64, i64* %n
  %7 = mul i64 %5, %6
  ret i64 %7
}

define i64 @"main\8A\7F"() {
entry:
  %a = alloca i64
  store i64 1093, i64* %a
  %b = alloca i64
  %0 = call i64 @"foo1\8A\7F"(i64* %a, i64 100)
  %1 = add i64 %0, 123
  %2 = call i64 @"foo1\8A\7F"(i64* %a, i64 12)
  %3 = add i64 %1, %2
  store i64 %3, i64* %a
  %4 = load i64, i64* %a
  %5 = add i64 %4, 120
  %6 = call i64 @"foo1\8A\7F"(i64 123, i64 %5)
  store i64 %6, i64* %b
}
