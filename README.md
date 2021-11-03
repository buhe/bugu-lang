![24b6db39bc9a4dc99b765aa311593d0a (1)](https://tva1.sinaimg.cn/large/008i3skNly1gw1lzpt93nj30b40b4mx2.jpg)

## bugu-lang

[![Rust](https://github.com/buhe/bugu-lang/actions/workflows/rust.yml/badge.svg)](https://github.com/buhe/bugu-lang/actions/workflows/rust.yml)

bugu-lang based buguOS

### step
2. bugu-lang -> risc-v asm
3. risc-v asm -> k210 bin code
3. based on buguOS filesystem implement, k210 bin code save fs as a block
   1. when construct fs, save bin code
   2. add inode and other meta data
