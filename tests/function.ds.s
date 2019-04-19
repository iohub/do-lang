	.text
	.file	"tests/function.ds.ll"
	.section	.rodata.cst4,"aM",@progbits,4
	.align	4
.LCPI0_0:
	.long	1063172178              # float 0.870000004
	.text
	.globl	"foo1%"
	.align	16, 0x90
	.type	"foo1%",@function
"foo1%":                               # @"foo1%\7F"
	.cfi_startproc
# BB#0:                                 # %entry
	pushq	%rbp
.Ltmp0:
	.cfi_def_cfa_offset 16
.Ltmp1:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
.Ltmp2:
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	movq	%rdi, -8(%rbp)
	movq	%rsi, -16(%rbp)
	movl	$1001, %eax             # imm = 0x3E9
	addq	-8(%rbp), %rax
	movq	%rax, -24(%rbp)
	movl	$1123477881, -36(%rbp)  # imm = 0x42F6E979
	xorl	%eax, %eax
	testb	%al, %al
	jne	.LBB0_2
# BB#1:                                 # %if-then
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$1123449569, -16(%rax)  # imm = 0x42F67AE1
	movq	-16(%rbp), %rcx
	addq	-24(%rbp), %rcx
	movq	-8(%rbp), %rdx
	leaq	1992(%rdx,%rcx), %rcx
	movq	%rcx, -32(%rbp)
	movss	-16(%rax), %xmm0        # xmm0 = mem[0],zero,zero,zero
	addss	.LCPI0_0(%rip), %xmm0
	movss	%xmm0, -16(%rax)
.LBB0_2:                                # %if-merge
	cmpq	$101, -24(%rbp)
	jl	.LBB0_4
# BB#3:                                 # %if-then1
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movq	$1002, -16(%rax)        # imm = 0x3EA
	addq	$1002, -24(%rbp)        # imm = 0x3EA
.LBB0_4:                                # %if-merge3
	movq	-24(%rbp), %rax
	addq	-32(%rbp), %rax
	movq	%rax, -8(%rbp)
	movq	%rbp, %rsp
	popq	%rbp
	retq
.Lfunc_end0:
	.size	"foo1%", .Lfunc_end0-"foo1%"
	.cfi_endproc

	.globl	"foo2ÒU"
	.align	16, 0x90
	.type	"foo2ÒU",@function
"foo2ÒU":                               # @"foo2\D2U"
	.cfi_startproc
# BB#0:                                 # %entry
	movq	%rdi, -8(%rsp)
	cmpq	$100, %rdi
	sete	%al
	retq
.Lfunc_end1:
	.size	"foo2ÒU", .Lfunc_end1-"foo2ÒU"
	.cfi_endproc

	.globl	"fact%"
	.align	16, 0x90
	.type	"fact%",@function
"fact%":                               # @"fact%\7F"
	.cfi_startproc
# BB#0:                                 # %entry
	pushq	%rax
.Ltmp3:
	.cfi_def_cfa_offset 16
	movq	%rdi, (%rsp)
	cmpq	$1, %rdi
	jne	.LBB2_2
# BB#1:                                 # %if-then
	movl	$1, %eax
	popq	%rcx
	retq
.LBB2_2:                                # %if-else
	movq	(%rsp), %rdi
	decq	%rdi
	callq	"fact%"
	imulq	(%rsp), %rax
	popq	%rcx
	retq
.Lfunc_end2:
	.size	"fact%", .Lfunc_end2-"fact%"
	.cfi_endproc

	.globl	"main%"
	.align	16, 0x90
	.type	"main%",@function
"main%":                               # @"main%\7F"
	.cfi_startproc
# BB#0:                                 # %entry
	pushq	%rbx
.Ltmp4:
	.cfi_def_cfa_offset 16
	subq	$16, %rsp
.Ltmp5:
	.cfi_def_cfa_offset 32
.Ltmp6:
	.cfi_offset %rbx, -16
	movq	$1093, 8(%rsp)          # imm = 0x445
	movl	$1093, %edi             # imm = 0x445
	movl	$100, %esi
	callq	"foo1%"
	movq	%rax, %rbx
	movq	8(%rsp), %rdi
	movl	$12, %esi
	callq	"foo1%"
	leaq	123(%rbx,%rax), %rcx
	movq	%rcx, 8(%rsp)
	leaq	243(%rbx,%rax), %rsi
	movl	$123, %edi
	callq	"foo1%"
	movq	%rax, (%rsp)
	xorl	%eax, %eax
	addq	$16, %rsp
	popq	%rbx
	retq
.Lfunc_end3:
	.size	"main%", .Lfunc_end3-"main%"
	.cfi_endproc


	.section	".note.GNU-stack","",@progbits
