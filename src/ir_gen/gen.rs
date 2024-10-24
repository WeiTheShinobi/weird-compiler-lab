use koopa::front::ast::*;
use koopa::ir::builder_traits::{BasicBlockBuilder, LocalInstBuilder, ValueBuilder};
use koopa::ir::{BinaryOp, FunctionData, Program, Type, Value};

use crate::ast::*;
use crate::ast::Stmt::Return;

pub trait Generate {
    fn generate(&self, program: &mut Program);
}

impl Generate for CompUnit {
    fn generate(&self, program: &mut Program) {
        self.func_def.generate(program);
    }
}

impl Generate for FuncDef {
    fn generate(&self, program: &mut Program) {
        let func = program.new_func(FunctionData::new(
            format!("@{}", self.ident),
            Vec::new(),
            Type::get_i32(),
        ));
        let func_data = program.func_mut(func);

        let entry = func_data
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%entry".into()));
        func_data.layout_mut().bbs_mut().extend([entry]);

        let ret_val = func_data.dfg_mut().new_value().integer(self.block.stmt.num);

        let ret = func_data.dfg_mut().new_value().ret(Some(ret_val));
        func_data
            .layout_mut()
            .bb_mut(entry)
            .insts_mut()
            .extend([ret]);
    }
}



impl Stmt {
    fn compile(&self, func_data: &mut FunctionData, insts :&mut Vec<Value>) {
        match self {
            Stmt::Return(exp) => {
                exp.compile(func_data, insts);
            }
        }
    }
}

impl Exp {
    fn compile(&self, func_data: &mut FunctionData, insts :&mut Vec<Value>) {
        match self {
            Exp::UnaryExp(exp) => {
                exp.compile(func_data, insts);
            }
        }
    }
}

impl UnaryExp {
    fn compile(&self, func_data: &mut FunctionData, insts :&mut Vec<Value>) {
        match self {
            UnaryExp::PrimaryExp(primary_exp) => match primary_exp {
                PrimaryExp::Expression(exp) => {}
                PrimaryExp::Number(n) => {}
            },
            UnaryExp::UnaryOp(unary_op, unary_exp) => {
                match unary_op {
                    UnaryOp::Add => (),
                    UnaryOp::Minus => {
                        let l_value = func_data.dfg_mut().new_value().integer(unary_exp.);
                        let r_value = func_data.dfg_mut().new_value().integer(0);
                        let inst = func_data.dfg_mut().new_value().binary(BinaryOp::Sub, l_value, r_value);
                        insts.push(inst);
                    },
                    UnaryOp::Not => {
                        let l_value = func_data.dfg_mut().new_value().integer(unary_exp.);
                        let r_value = func_data.dfg_mut().new_value().integer(0);
                        let inst = func_data.dfg_mut().new_value().binary(BinaryOp::Eq, l_value, r_value);
                        insts.push(inst);
                    }
                }
            },
        }
    }
}

impl PrimaryExp {
    fn compile(&self, func_data: &mut FunctionData, insts :&mut Vec<Value>) -> Value {
        match self {
            PrimaryExp::Expression(exp) => exp.compile(func_data),
            PrimaryExp::Number(n) => return *n,
        }
    }
}

pub enum ASTType {
    Number(i32),
    Array(Box<Type>, i32),
    Pointer,
    Function
}
