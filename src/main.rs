use std::fs;

use lib_rv32_asm::assemble_program;


fn main() -> std::io::Result<()> {
    let i = std::env::args().nth(1).expect("no input");
    let src = fs::read_to_string(i).expect("src not existed.");
    println!("With text:\n{}", src);
    // let bin = assemble_program("").unwrap();
    Ok(())
}
