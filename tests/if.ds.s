	.text
	.file	"tests/if.ds.ll"
	.globl	"maink"
	.align	16, 0x90
	.type	"maink",@function
"maink":                               # @"maink\7F"
	.cfi_startproc
# BB#0:                                 # %entry
	movq	$1093, -8(%rsp)         # imm = 0x445
	xorl	%eax, %eax
	testb	%al, %al
	jne	.LBB0_2
# BB#1:                                 # %if-then
	movq	-8(%rsp), %rax
	addq	$13, %rax
	movq	%rax, -16(%rsp)
.LBB0_2:                                # %merge
	movq	-16(%rsp), %rax
	retq
.Lfunc_end0:
	.size	"maink", .Lfunc_end0-"maink"
	.cfi_endproc


	.section	".note.GNU-stack","",@progbits
