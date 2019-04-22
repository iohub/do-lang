; ModuleID = '__module'
source_filename = "__module"

define i64 @add(i64) {
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
  %4 = add i64 2, %3
  ret i64 %4
}

define i64 @main() {
entry:
  %a = alloca i64
  store i64 2, i64* %a
  br label %"while:cond"

"while:cond":                                     ; preds = %"while:merge3", %entry
  %0 = load i64, i64* %a
  %1 = icmp slt i64 %0, 100
  br i1 %1, label %"while:body", label %"while:merge"

"while:body":                                     ; preds = %"while:cond"
  %2 = load i64, i64* %a
  %3 = call i64 @add(i64 %2)
  %4 = load i64, i64* %a
  %5 = add i64 %4, %3
  store i64 %5, i64* %a
  %b = alloca i64
  %6 = load i64, i64* %a
  store i64 %6, i64* %b
  br label %"while:cond1"

"while:cond1":                                    ; preds = %"while:body2", %"while:body"
  %7 = load i64, i64* %b
  %8 = icmp slt i64 %7, 1000
  br i1 %8, label %"while:body2", label %"while:merge3"

"while:body2":                                    ; preds = %"while:cond1"
  %9 = load i64, i64* %b
  %10 = add i64 %9, 1
  store i64 %10, i64* %b
  br label %"while:cond1"

"while:merge3":                                   ; preds = %"while:cond1"
  br label %"while:cond"

"while:merge":                                    ; preds = %"while:cond"
  ret i64 0
}
