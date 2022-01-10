use std::{fs, io::{BufWriter}, process::Command};

use buguc::run;



fn main() -> std::io::Result<()> {
    let i = std::env::args().nth(1).expect("no input");
    let src = fs::read_to_string(&i).expect("src not existed.");
    let mut buf = BufWriter::new(Vec::new());
    let target = &i.replace(".bugu", "");
    println!("src:\n{}", src);
    run(src, &mut buf)?;
    let string = String::from_utf8(buf.into_inner()?).unwrap();
    println!("asm:\n {}", &string);
    let asm_file = format!("{}.S", target);
    fs::write(&asm_file, &string)?;
    let mut r = Command::new("riscv-gcc/bin/riscv64-unknown-elf-gcc")
        .arg("-march=rv32im")
        .arg("-mabi=ilp32")
        .arg(&asm_file)
        .arg("-o")
        .arg(target)
        .spawn()?;
    r.wait()?;
    fs::remove_file(&asm_file)?;
    Ok(())
}
