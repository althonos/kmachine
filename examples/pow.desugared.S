start:
	clr	%rbx
	inc	%rbx
	inc	%rbx
	inc	%rbx
	clr	%rax
	inc	%rax
	inc	%rax
	inc	%rax
	inc	%rax
	jnz	%rbx, exp_nonzero
exp_zero:
	clr	%rax
	clr	%rbx
	inc	%rbx
	jmp	exp_after
exp_nonzero:
	dec	%rbx
	swp	%rax, %rbx
	clr	%rcx
	mov	%rbx, %_cpy
_cpy_1:
	inc	%rbx
	inc	%rcx
	dec	%_cpy
	jnz	%_cpy, _cpy_1
exp_loop:
	clr	%rdx
	mov	%rcx, %_cpy
_cpy_2:
	inc	%rcx
	inc	%rdx
	dec	%_cpy
	jnz	%_cpy, _cpy_2
	mov	%rbx, %_mul1
	clr	%_mul2
	mov	%_mul1, %_cpy
_cpy_3:
	inc	%_mul1
	inc	%_mul2
	dec	%_cpy
	jnz	%_cpy, _cpy_3
_mov_1_loop:
	jz	%rdx, _mov_1_after
	dec	%rdx
	add	%_mul1, %rbx
	clr	%_mul1
	mov	%_mul2, %_cpy
_cpy_4:
	inc	%_mul2
	inc	%_mul1
	dec	%_cpy
	jnz	%_cpy, _cpy_4
	jmp	_mov_1_loop
_mov_1_after:
	clr	%_mul1
	clr	%_mul2
	dec	%rax
	jnz	%rax, exp_loop
exp_after:
	clr	%rcx
	clr	%rdx
