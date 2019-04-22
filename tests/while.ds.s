	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 13
	.globl	_add
	.p2align	4, 0x90
_add:                                   ## @add
	.cfi_startproc
## BB#0:                                ## %entry
	movq	%rdi, -8(%rsp)
	cmpq	$1, %rdi
	jne	LBB0_2
## BB#1:                                ## %"if:then"
	movl	$1, %eax
	retq
LBB0_2:                                 ## %"if:else"
	movq	-8(%rsp), %rax
	addq	$2, %rax
	retq
	.cfi_endproc

	.globl	_main
	.p2align	4, 0x90
_main:                                  ## @main
	.cfi_startproc
## BB#0:                                ## %entry
	pushq	%rbp
Ltmp0:
	.cfi_def_cfa_offset 16
Ltmp1:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Ltmp2:
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	$2, -8(%rbp)
	jmp	LBB1_1
	.p2align	4, 0x90
LBB1_2:                                 ## %"while:body"
                                        ##   in Loop: Header=BB1_1 Depth=1
	movq	-8(%rbp), %rdi
	callq	_add
	addq	%rax, -8(%rbp)
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rax
	movq	%rax, %rsp
	movq	-8(%rbp), %rdx
	movq	%rdx, -16(%rcx)
	jmp	LBB1_3
	.p2align	4, 0x90
LBB1_4:                                 ## %"while:body2"
                                        ##   in Loop: Header=BB1_3 Depth=2
	incq	(%rax)
LBB1_3:                                 ## %"while:cond1"
                                        ##   Parent Loop BB1_1 Depth=1
                                        ## =>  This Inner Loop Header: Depth=2
	cmpq	$999, (%rax)            ## imm = 0x3E7
	jle	LBB1_4
LBB1_1:                                 ## %"while:cond"
                                        ## =>This Loop Header: Depth=1
                                        ##     Child Loop BB1_3 Depth 2
	cmpq	$99, -8(%rbp)
	jle	LBB1_2
## BB#5:                                ## %"while:merge"
	xorl	%eax, %eax
	movq	%rbp, %rsp
	popq	%rbp
	retq
	.cfi_endproc


.subsections_via_symbols
