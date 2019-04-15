
define i32 @_Z6squarei(i32) #0 {
  %2 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %3 = load i32, i32* %2, align 4
  %4 = add nsw i32 %3, 1001
  ret i32 %4
}

define i32 @main() #2 {
  %1 = alloca i32, align 4
  store i32 10, i32* %1, align 4
  %2 = load i32, i32* %1, align 4
  %3 = call i32 @_Z6squarei(i32 2)
  store i32 %3, i32* %1, align 4
  ret i32 0
}

