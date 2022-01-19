use std::{fs, io::{BufWriter}, process::Command};

use buguc::run;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    s: bool,

    input: Option<String>,
}


fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let input = &cli.input.unwrap();
    let asm_created = cli.s;
    // println!("name: {:?}", );
    let src = fs::read_to_string(input).expect("src not existed.");
    let mut buf = BufWriter::new(Vec::new());
    let target = input.replace(".bugu", "");
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
    if !asm_created {
        fs::remove_file(&asm_file)?;
    }
    Ok(())
}

    #[test]
    fn ra_1() {
        buguc::run("int gcd(int a, int b) {
                                while (a != 0) {
                                    int c;
                                    c = b % a;
                                    b = a;
                                    a = 1;
                                    a = 2;
                                    a = c;
                                }
                                return b;
                            }".to_string(), 
        &mut std::io::stdout()).unwrap();
    }
