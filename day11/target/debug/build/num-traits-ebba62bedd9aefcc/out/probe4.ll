; ModuleID = 'probe4.7edd4aba83608bbd-cgu.0'
source_filename = "probe4.7edd4aba83608bbd-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

@alloc_f54355b456c22ada5a76d6a735dd5709 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/43bc152626bbca1fcef7517f9445c02f654d7ce8\\library\\core\\src\\num\\mod.rs" }>, align 1
@alloc_e9c80db69e62e496028ec555f9d77df2 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_f54355b456c22ada5a76d6a735dd5709, [16 x i8] c"K\00\00\00\00\00\00\00v\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: sspstrong uwtable
define void @_ZN6probe45probe17h0238bc86faa9e88bE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h74f4b289b50df024E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h25ef5fae2df12666E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_e9c80db69e62e496028ec555f9d77df2) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h74f4b289b50df024E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn sspstrong uwtable
declare void @_ZN4core9panicking5panic17h25ef5fae2df12666E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { sspstrong uwtable "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn sspstrong uwtable "target-cpu"="x86-64" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.74.0 (43bc152626 2023-11-16) (1.74.0-ms-20231117.1+43bc152626)"}
