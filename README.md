# ϰmachine

*A toy compiler that produces Kappa code from counter machine instructions.*

## Syntax

The syntax is heavily inspired by the AT&T syntax of the [GNU Assembler]. The following example
program copies the content of register `%rax` to register `%rbx` using a temporary register `%rcx`:

```asm
        ; clear content of %rcx
        clr %rcx
cpy1:   ; move %rax to both %rbx and %rcx
        inc %rcx
        inc %rbx
        dec %rax
	jz  %rax, cpy2
        jmp cpy1
cpy2:   ; move %rcx back into %rax
        inc %rax
        dec %rcx
        jz  %rcx, cpy3
	jmp cpy2
cpy3:
```

[GNU Assembler]: https://en.wikipedia.org/wiki/GNU_Assembler

## Instructions

| Instruction | Supported | Emulated |
| ----------- | --------- | -------- |
|    `clr`    |     ✓     |          |
|    `dec`    |     ✓     |          |
|    `inc`    |     ✓     |          |
|    `jmp`    |     ✓     |          |
|    `jz`     |     ✓     |          |


## About

This program was developed by [Martin Larralde](https://github.com/althonos) as the final
project for the *Biochemical Programming* course ([2.19]) of the [MPRI].

[2.19]: https://wikimpri.dptinfo.ens-cachan.fr/doku.php?id=cours:c-2-19
[MPRI]: https://wikimpri.dptinfo.ens-cachan.fr/doku.php
