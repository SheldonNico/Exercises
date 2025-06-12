#import "@preview/codly:1.3.0": *
#show: codly-init.with()

#include "notes.chap03.typ"

= Chapter 4
== 4.1
#table(
  columns: 2,
  `.pos 0x100`, ``,
  `    irmovq $15,%rbx`, `30 f3 0f 00 00 00 00 00 00 00`,
  `    rrmovq %rbx,%rcx`, `20 31`,
  `loop:`, `pos at 0x100 + 12 = 0x10c`,
  `    rmmovq %rcx,-3(%rbx)`, `40 13 fd ff ff ff ff ff ff ff`,
  `    addq %rbx,%rcx`, `60 31`,
  `    jmp loop`, `70 0c 01 00 00 00 00 00 00`,
)

== 4.2
#[
#set enum(numbering: "A.")
+ `0x100: 30f3fcffffffffffffff40630008000000000000`
  - `irmovq $0xfffffffffffffffc,%rbx`
  - `rmmovq %rsi,0x800(%rbx)`
+ `0x200: a06f800c020000000000000030f30a00000000000000`
  - `pushq %rsi`
  - `call 0x20c`
  - `halt`
  - `irmovq 0xa,%rbx`
+ `0x300: 5054070000000000000010f0b01f`
  - `mrmovq 0x7(%rsp),%rbp`
  - `nop`
  - #text(fill: red)[`f0b01f`]
+ `0x400: 611373000400000000000000`
  - `subq %rcx,%rbx`
  - `je 0x400`
  - `halt`
+ `0x500: 6362a0f0`
  - `xorq %rsi,%rdx`
  - #text(fill: red)[`a0f0`]
]

== 4.3
```asm
sum:
  xorq %rax,%rax
  andq %rsi,%rsi
  jmp test
loop:
  mrmovq (%rdi),%r10
  addq %r10,%rax
  iaddq $8,%rdi
  iaddq $-1,%rsi
test:
  jne loop
  ret
```

== 4.4
The problem describe show `count <= 1` should be `count <= 0`. Original description leads to wrong answer.
```asm
# start in %rdi, count in %rsi
rproduct:
  irmovq $8,%r8
  irmovq $1,%r9
  irmovq $1,%rax
  subq %r9,%rsi
  jle done
  mrmovq (%rdi),%r10
  addq %r8,%rdi
  call rproduct
  mulq %r10,%rax
done:
  ret
```

== 4.5
```asm
# start in %rdi, count in %rsi
absSum:
  irmovq $8,%r8
  irmovq $1,%r9
  xorq %rax,%rax
  andq %rsi,%rsi
  jmp test
loop:
  mrmovq (%rdi),%r10
  xorq %r11,%r11
  subq %r10,%r11 # 0-x = -x
  jmple pos
  movq %r11,%r10
pos:
  addq %r10,%rax
  addq %r8,%rdi
  subq %r9,%rsi
test:
  jne loop
  ret
```

== 4.6
```asm
# start in %rdi, count in %rsi
absSum:
  irmovq $8,%r8
  irmovq $1,%r9
  xorq %rax,%rax
  andq %rsi,%rsi
  jmp test
loop:
  mrmovq (%rdi),%r10
  irmovq $-1,%r11
  andq %r10,%r10
  cmovge %r9,%r11
  mulq %11,%10
  addq %r10,%rax
  addq %r8,%rdi
  subq %r9,%rsi
test:
  jne loop
  ret
```
