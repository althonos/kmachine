	clr	%rax
	inc	%rax
	inc	%rax
	inc	%rax
	inc	%rax
	inc	%rax
loop1:
	inc	%rbx
	dec	%rax
	jz	%rax, loop2
	jmp	loop1
loop2:
	inc	%rax
	dec	%rbx
	jz	%rbx, loop1
	jmp	loop2
