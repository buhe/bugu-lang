![Untitled](https://tva1.sinaimg.cn/large/008i3skNgy1gxnuqh31vyj30cp08nglj.jpg)

## bugu-lang

[![Rust](https://github.com/buhe/bugu-lang/actions/workflows/rust.yml/badge.svg)](https://github.com/buhe/bugu-lang/actions/workflows/rust.yml)

bugu-lang based buguOS

### step
1. bugu-lang -> risc-v asm
2. risc-v asm -> risc-v bin code
3. risc-v bin code -> elf 
4. based on buguOS filesystem implement(fat32), risc-v bin code save fs as a block

### buguOS

1. parser elf
2. proxy print

### Todo
[ ] gen elf
[ ] link obj
