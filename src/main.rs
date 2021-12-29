use std::{fs, io::BufWriter};

use buguc::run;
use lib_rv32_asm::assemble_program;
use byteorder::{LittleEndian, WriteBytesExt};


fn main() -> std::io::Result<()> {
    let i = std::env::args().nth(1).expect("no input");
    let src = fs::read_to_string(&i).expect("src not existed.");
    let mut buf = BufWriter::new(Vec::new());
    let target = &i.replace(".bugu", "");
    println!("With text:\n{}", src);
    run(src, &mut buf)?;
    let string = String::from_utf8(buf.into_inner()?).unwrap();
    let mut bin: Vec<u8> = Vec::new();
    for n in assemble_program(&string).unwrap() {
        let _ = bin.write_u32::<LittleEndian>(n);
    }
    fs::write(target, bin)?;
    Ok(())
}
