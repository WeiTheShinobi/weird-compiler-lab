use std::env::args;
use std::fmt::{self};

use std::fs::{read_to_string, File};

use koopa::back::KoopaGenerator;
use lalrpop_util::lalrpop_mod;
use std::process::exit;
use std::{io, vec};

mod ast;
mod ir_gen;
mod riscv_gen;
mod analysis;

lalrpop_mod!(sysy);
fn main() {
    let mut args = args();
    args.next();
    let mode = args.next().unwrap();
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();

    let mut ss = vec![];
    while let Some(arg) = args.next() {
        ss.push(arg);
    }
    if let Err(err) = try_main(Args {
        mode,
        input,
        output,
        args: ss,
    }) {
        eprintln!("{}", err);
        exit(-1);
    }
}

#[derive(Debug)]
struct Args {
    mode: String,
    input: String,
    output: String,
    args: Vec<String>,
}

fn try_main(args: Args) -> Result<(), Error> {
    let input = read_to_string(args.input).map_err(Error::File)?;
    let ast = sysy::CompUnitParser::new()
        .parse(&input)
        .map_err(|_| Error::Parse)?;

    println!(
        "\ninput source code:\n======================\n{}======================\n",
        input
    );
    match args.mode.as_str() {
        "-koopa" => {
            let output_file = File::create(args.output).map_err(Error::File)?;
            let program = ir_gen::generate_program(&ast).map_err(Error::KoopaGen)?;
            KoopaGenerator::new(output_file)
                .generate_on(&program)
                .unwrap();
            Ok(())
        }
        "-riscv" => {
            let output_file = File::create(args.output).map_err(Error::File)?;
            let koopa = ir_gen::generate_program(&ast).map_err(Error::KoopaGen)?;
            let _ = riscv_gen::generate_riscv(koopa, args.args)
                .map_err(Error::RiscvGen)?
                .generate_on(output_file);
            Ok(())
        }
        _ => {
            unreachable!("unsupport mode: {}", args.mode);
        }
    }
}

enum Error {
    File(io::Error),
    Parse,
    KoopaGen(ir_gen::Error),
    RiscvGen(riscv_gen::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Parse => write!(f, "error occurred while parsing"),
            Self::File(err) => write!(f, "invalid input SysY file: {}", err),
            Self::KoopaGen(err) => write!(f, "koopa gen error: {:?}", err),
            Self::RiscvGen(err) => write!(f, "gen isa error: {:?}", err),
        }
    }
}

#[cfg(test)]
mod test {
    macro_rules! test_koopa {
        ($file_name: ident) => {
            #[test]
            fn $file_name() {
                let file_name = stringify!($file_name);
                if !PathBuf::from("./tests/output").exists() {
                    fs::create_dir("./tests/output").unwrap();
                }

                let old_koopa = format!("./tests/output/{}{}", file_name, ".koopa");
                if PathBuf::from(&old_koopa).exists() {
                    fs::remove_file(&old_koopa).unwrap();
                }

                println!("start compile {}", &file_name);
                let args = Args {
                    mode: "-koopa".to_string(),
                    input: format!("{}{}{}", "./tests/input/", file_name, ".c"),
                    output: format!("{}{}{}", "./tests/output/", file_name, ".koopa"),
                    args: vec!["-p".to_string()],
                };
                println!("koopa {}", file_name);
                if let Err(e) = try_main(args) {
                    panic!("{}", e.to_string());
                }

                let file_name_og = format!("{}_og", file_name);
                let args = Args {
                    mode: "-koopa".to_string(),
                    input: format!("{}{}{}", "./tests/input/", file_name, ".c"),
                    output: format!("{}{}{}", "./tests/output/", file_name_og, ".koopa"),
                    args: vec!["-p".to_string()],
                };
                println!("koopa {}", file_name);
                if let Err(e) = try_main(args) {
                    panic!("{}", e.to_string());
                }
            }
        };
    }

    macro_rules! test_riscv {
        ($file_name: ident) => {
            #[test]
            fn $file_name() {
                let file_name = stringify!($file_name);
                if !PathBuf::from("./tests/output").exists() {
                    fs::create_dir("./tests/output").unwrap();
                }

                let old_riscv = format!("./tests/output/{}{}", file_name, ".riscv");
                if PathBuf::from(&old_riscv).exists() {
                    fs::remove_file(&old_riscv).unwrap();
                }

                let args = Args {
                    mode: "-riscv".to_string(),
                    input: format!("{}{}{}", "./tests/input/", file_name, ".c"),
                    output: format!("{}{}{}", "./tests/output/", file_name, ".riscv"),
                    args: vec!["-p".to_string()],
                };
                println!("riscv {}", file_name);
                if let Err(e) = try_main(args) {
                    panic!("{}", e.to_string());
                }

                let file_name_og = format!("{}_og", file_name);
                let args = Args {
                    mode: "-riscv".to_string(),
                    input: format!("{}{}{}", "./tests/input/", file_name, ".c"),
                    output: format!("{}{}{}", "./tests/output/", file_name_og, ".riscv"),
                    args: vec![],
                };
                println!("riscv {}", file_name);
                if let Err(e) = try_main(args) {
                    panic!("{}", e.to_string());
                }
            }
        };
    }

    mod koopa {
        use crate::{try_main, Args};
        use std::{
            fs::{self},
            path::PathBuf,
        };

        test_koopa!(arithmetic);
        test_koopa!(const1);
        test_koopa!(const2);
        test_koopa!(hello);
        test_koopa!(land);
        test_koopa!(logic);
        test_koopa!(lor);
        test_koopa!(unary_exp);
        test_koopa!(var);
        test_koopa!(var2);
        test_koopa!(block);
        test_koopa!(block2);
        test_koopa!(if_else);
        test_koopa!(if_else2);
        test_koopa!(if_else3);
        test_koopa!(if_else4);
        test_koopa!(if_else5);
        test_koopa!(dangling_else);
        test_koopa!(short_circuit_or);
        test_koopa!(short_circuit_and);
        test_koopa!(while1);
        test_koopa!(break1);
        test_koopa!(continue1);
        test_koopa!(function1);
        test_koopa!(global_var1);
        test_koopa!(buildin);
        test_koopa!(peephole);
    }
    mod riscv {
        use crate::{try_main, Args};
        use std::{
            fs::{self},
            path::PathBuf,
        };

        test_riscv!(arithmetic);
        test_riscv!(const1);
        test_riscv!(const2);
        test_riscv!(hello);
        test_riscv!(land);
        test_riscv!(logic);
        test_riscv!(lor);
        test_riscv!(unary_exp);
        test_riscv!(var);
        test_riscv!(var2);
        test_riscv!(block);
        test_riscv!(block2);
        test_riscv!(if_else);
        test_riscv!(if_else2);
        test_riscv!(if_else3);
        test_riscv!(if_else4);
        test_riscv!(if_else5);
        test_riscv!(dangling_else);
        test_riscv!(short_circuit_or);
        test_riscv!(short_circuit_and);
        test_riscv!(while1);
        test_riscv!(break1);
        test_riscv!(continue1);
        test_riscv!(function1);
        test_riscv!(global_var1);
        test_riscv!(buildin);
        test_riscv!(peephole);
    }
}
