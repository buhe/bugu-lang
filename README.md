![Untitled](https://tva1.sinaimg.cn/large/008i3skNgy1gxnuqh31vyj30cp08nglj.jpg)

## bugu-lang

[![Rust](https://github.com/buhe/bugu-lang/actions/workflows/rust.yml/badge.svg)](https://github.com/buhe/bugu-lang/actions/workflows/rust.yml)

bugu-lang based buguOS

### step
2. bugu-lang -> risc-v asm
3. risc-v asm -> k210 bin code
3. based on buguOS filesystem implement, k210 bin code save fs as a block
   1. when construct fs, save bin code
   2. add inode and other meta data

### buguOS

1. parser elf
2. proxy print
