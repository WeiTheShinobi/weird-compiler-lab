use crate::riscv_gen::context::Context;
use gen::*;

mod context;
mod gen;
mod inst;
mod optimizer;
mod reg;

#[derive(Debug)]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub fn generate_riscv<'a>(program: koopa::ir::Program, args: Vec<String>) -> Result<Program> {
    let mut riscv = gen::Program::new();
    let mut cx = Context::new();
    program.generate(&mut riscv, &mut cx);

    if args.contains(&"-p".to_string()) {
        riscv = optimizer::peephole(riscv);
    }
    Ok(riscv)
}
