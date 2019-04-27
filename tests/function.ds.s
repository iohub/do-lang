	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 13
	.section	__TEXT,__literal4,4byte_literals
	.p2align	2               ## -- Begin function foo1
LCPI0_0:
	.long	1063172178              ## float 0.870000004
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_foo1
	.p2align	4, 0x90
_foo1:                                  ## @foo1
	.cfi_startproc
## %bb.0:                               ## %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	movq	%rdi, -16(%rbp)
	movq	%rsi, -32(%rbp)
	addq	$1001, %rdi             ## imm = 0x3E9
	movq	%rdi, -8(%rbp)
	movl	$1123477881, -36(%rbp)  ## imm = 0x42F6E979
	xorl	%eax, %eax
	testb	%al, %al
	jne	LBB0_2
## %bb.1:                               ## %"if:then"
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$1123449569, -16(%rax)  ## imm = 0x42F67AE1
	movq	-32(%rbp), %rcx
	addq	-8(%rbp), %rcx
	movq	-16(%rbp), %rdx
	leaq	1992(%rdx,%rcx), %rcx
	movq	%rcx, -24(%rbp)
	movss	-16(%rax), %xmm0        ## xmm0 = mem[0],zero,zero,zero
	addss	LCPI0_0(%rip), %xmm0
	movss	%xmm0, -16(%rax)
LBB0_2:                                 ## %"if:merge"
	cmpq	$101, -8(%rbp)
	jl	LBB0_4
## %bb.3:                               ## %"if:then1"
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movq	$1002, -16(%rax)        ## imm = 0x3EA
	addq	$1002, -8(%rbp)         ## imm = 0x3EA
LBB0_4:                                 ## %"if:merge3"
	movq	-8(%rbp), %rax
	addq	-24(%rbp), %rax
	movq	%rax, -16(%rbp)
	movq	%rbp, %rsp
	popq	%rbp
	retq
	.cfi_endproc
                                        ## -- End function
	.globl	_foo2                   ## -- Begin function foo2
	.p2align	4, 0x90
_foo2:                                  ## @foo2
	.cfi_startproc
## %bb.0:                               ## %entry
	movq	%rdi, -8(%rsp)
	cmpq	$100, %rdi
	sete	%al
	retq
	.cfi_endproc
                                        ## -- End function
	.globl	_fact                   ## -- Begin function fact
	.p2align	4, 0x90
_fact:                                  ## @fact
	.cfi_startproc
## %bb.0:                               ## %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdi, (%rsp)
	cmpq	$1, %rdi
	jne	LBB2_2
## %bb.1:                               ## %"if:then"
	movl	$1, %eax
	popq	%rcx
	retq
LBB2_2:                                 ## %"if:else"
	movq	(%rsp), %rdi
	decq	%rdi
	callq	_fact
	imulq	(%rsp), %rax
	popq	%rcx
	retq
	.cfi_endproc
                                        ## -- End function
	.globl	_main                   ## -- Begin function main
	.p2align	4, 0x90
_main:                                  ## @main
	.cfi_startproc
## %bb.0:                               ## %entry
	pushq	%rbx
	.cfi_def_cfa_offset 16
	subq	$16, %rsp
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -16
	movq	$1093, (%rsp)           ## imm = 0x445
	movl	$1093, %edi             ## imm = 0x445
	movl	$100, %esi
	callq	_foo1
	movq	%rax, %rbx
	movq	(%rsp), %rdi
	movl	$12, %esi
	callq	_foo1
	leaq	123(%rbx,%rax), %rcx
	movq	%rcx, (%rsp)
	leaq	243(%rbx,%rax), %rsi
	movl	$123, %edi
	callq	_foo1
	jmp	LBB3_1
	.p2align	4, 0x90
LBB3_2:                                 ## %"while:body"
                                        ##   in Loop: Header=BB3_1 Depth=1
	movq	(%rsp), %rdi
	movq	8(%rsp), %rsi
	callq	_foo1
	addq	(%rsp), %rax
LBB3_1:                                 ## %"while:cond"
                                        ## =>This Inner Loop Header: Depth=1
	movq	%rax, 8(%rsp)
	movq	8(%rsp), %rax
	addq	$100, %rax
	cmpq	%rax, (%rsp)
	jg	LBB3_2
## %bb.3:                               ## %"while:merge"
	xorl	%eax, %eax
	addq	$16, %rsp
	popq	%rbx
	retq
	.cfi_endproc
                                        ## -- End function

.subsections_via_symbols
