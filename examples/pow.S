// compute %rax ^ %rbx into %rbx

start:

	mov $3, %rbx
	mov $4, %rax


	jnz %rbx, exp_nonzero

exp_zero:
	clr %rax
	mov $1, %rbx
	jmp exp_after

exp_nonzero:
	dec %rbx
	swp %rax, %rbx
	cpy %rbx, %rcx

exp_loop:
	cpy %rcx, %rdx
	mul %rdx, %rbx
	dec %rax
	jnz %rax, exp_loop

exp_after:
	clr %rcx
	clr %rdx



