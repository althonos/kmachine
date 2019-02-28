// this program makes the values of %rax and %rbx
// oscillate between 0 and 5 in an infinite loop.

	mov $5, %rax

loop1:  // decrement %rax and increment %rbx

	inc %rbx
	dec %rax
	jz %rax, loop2
	jmp loop1

loop2:  // decrement %rbx and increment %rax

	inc %rax
	dec %rbx
	jz %rbx, loop1
	jmp loop2
