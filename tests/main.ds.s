	.text
	.file	"tests/main.ds.ll"
	.globl	_Z6squarei
	.align	16, 0x90
	.type	_Z6squarei,@function
_Z6squarei:                             # @_Z6squarei
	.cfi_startproc
# BB#0:
	movl	%edi, -4(%rsp)
	leal	1001(%rdi), %eax
	retq
.Lfunc_end0:
	.size	_Z6squarei, .Lfunc_end0-_Z6squarei
	.cfi_endproc

	.globl	main
	.align	16, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# BB#0:
	pushq	%rax
.Ltmp0:
	.cfi_def_cfa_offset 16
	movl	$10, 4(%rsp)
	movl	$2, %edi
	callq	_Z6squarei
	movl	%eax, 4(%rsp)
	xorl	%eax, %eax
	popq	%rcx
	retq
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc


	.section	".note.GNU-stack","",@progbits
