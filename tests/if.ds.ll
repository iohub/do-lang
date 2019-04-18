; ModuleID = '__module'

define i64 @"main\8C\7F"() {
entry:
  %a = alloca i64
  store i64 1093, i64* %a
  %retval = alloca i64
  %0 = load i64, i64* %a
  %1 = icmp sgt i64 %0, 92
  br i1 %1, label %if-then, label %if-else

if-then:                                          ; preds = %entry
  %2 = load i64, i64* %a
  %3 = add i64 %2, 13
  store i64 %3, i64* %retval
  br label %merge

if-else:                                          ; preds = %entry
  br label %merge

merge:                                            ; preds = %if-else, %if-then
  %4 = load i64, i64* %retval
  ret i64 %4
}
