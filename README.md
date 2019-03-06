# ϰmachine

*A toy compiler that produces [Kappa] code from [counter machine] instructions.*

[Kappa]: https://kappalanguage.org/
[counter machine]: https://en.wikipedia.org/wiki/Counter_machine

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

## Usage

Provide the compiler with a pseudo-assembly program, and it will generate a
self-sufficient Kappa source:
```console
$ cargo run -- examples/loop.S > examples/loop.ka
```

This program can then be run with `KaSim` through the command line:
```console
$ KaSim examples/loop.ka
```

Or you can open it with the `KUI` to inspect the generated agents and rules:
![KaSim agents](https://github.com/althonos/kmachine/raw/master/docs/agents.png?sanitize=true)


## Instructions

| Instruction | Supported | Emulated | Example           |
| :---------: | :-------: | :------: | :---------------- |
|    `add`    |     ✓     |          | `add %rax, %rbx`  |
|    `add` *  |           |     ✓    | `add $0x12, %rbx` |
|    `clr`    |     ✓     |          | `clr %rax`        |
|    `cpy`    |           |     ✓    | `cpy %rax, %rbx`  |
|    `inc`    |     ✓     |          | `inc %rcx`        |
|    `jmp`    |     ✓     |          | `jmp label`       |
|    `jnz`    |     ✓     |          | `jnz %rax, label` |
|    `jz`     |     ✓     |          | `jz  %rax, label` |
|    `mov`    |     ✓     |          | `mov %rax, %rbx`  |
|    `mov` *  |           |     ✓    | `mov $5, %rax`    |
|    `mul`    |           |     ✓    | `mul %rax, %rbx`  |


## About

This program was developed by [Martin Larralde](https://github.com/althonos) as the final
project for the *Biochemical Programming* course ([2.19]) of the [MPRI].

[2.19]: https://wikimpri.dptinfo.ens-cachan.fr/doku.php?id=cours:c-2-19
[MPRI]: https://wikimpri.dptinfo.ens-cachan.fr/doku.php
