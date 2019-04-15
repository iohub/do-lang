; ModuleID = '__module'

define i64 @"foo1\F4\7F"(i64, i64) {
entry:
  %a = alloca i64
  store i64* %a, i64 %0
  %b = alloca i64
  store i64* %b, i64 %1
  %c = alloca i64
  %2 = add i64* %a, i64 1001
  store i64* %c, i64* %2
  %d = alloca i64
  %ok = alloca float
  store float* %ok, float 0x405EDD2F20000000
  %3 = fcmp ogt float* %ok, float 0x405907DF40000000
  br i1 %3, label %if-then, label %if-else

if-then:                                          ; preds = %entry
  %val = alloca float
  store float* %val, float 0x405ECF5C20000000
  %4 = add i64* %b, i64 1992
  %5 = add i64* %4, %c
  %6 = add i64* %5, %a
  store i64* %d, i64* %6
  %7 = add float* %val, float 0x3FEBD70A40000000
  store float* %val, float* %7

if-else:                                          ; preds = %entry
  %8 = icmp sgt i64* %c, i64 100
  br i1 %8, label %if-then1, label %if-else2

if-then1:                                         ; preds = %if-else
  %bv = alloca i64
  store i64* %bv, i64 1002
  %9 = add i64* %bv, %c
  store i64* %c, i64* %9

if-else2:                                         ; preds = %if-else
  %10 = add i64* %c, %d
  store i64* %a, i64* %10
  ret i64* %a
}

define i8 @"foo2\D7U"(i64) {
entry:
  %a = alloca i64
  store i64* %a, i64 %0
  %1 = icmp eq i64* %a, i64 100
  ret i1 %1
}

define i64 @"fact\F4\7F"(i64) {
entry:
  %n = alloca i64
  store i64* %n, i64 %0
  %1 = icmp eq i64* %n, i64 1
  br i1 %1, label %if-then, label %if-else

if-then:                                          ; preds = %entry
  ret i64 1

if-else:                                          ; preds = %entry
  %2 = sub i64* %n, i64 1
  %3 = call i64 @"fact\F4\7F"(i64* %2)
  %4 = mul i64 %3, i64* %n
  ret i64 %4
}

define i64 @"main\F4\7F"() {
entry:
  %a = alloca i64
  store i64* %a, i64 1093
  %b = alloca i64
  %0 = call i64 @"foo1\F4\7F"(i64* %a, i64 100)
  %1 = add i64 %0, 123
  %2 = call i64 @"foo1\F4\7F"(i64* %a, i64 12)
  %3 = add i64 %1, %2
  store i64* %a, i64 %3
  %4 = add i64* %a, i64 120
  %5 = call i64 @"foo1\F4\7F"(i64 123, i64* %4)
  store i64* %b, i64 %5
}
