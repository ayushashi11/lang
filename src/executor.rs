use crate::token::*;
use crate::scope::*;
use std::io;
#[derive(Debug, Clone)]
pub struct Executor{
    pub memory: Box<Scope>
}
impl Executor{
    pub fn new() -> Self{
        Executor{memory: Box::new(Scope::new())}
    }
    pub fn visit_main(&self, block: Block) -> DataType{
        let mut val=None;
        for stmt in block.iter(){
            val=Some(self.visit_stmt(stmt.clone()));
        }
        match val{
            Some(v) => v,
            None => DataType::Null
        }
    }
    pub fn visit_stmt(mut self, stmt: Stmt) -> DataType{
        return match stmt{
            Stmt::PrintStmt(expr) => self.visit_print_stmt(expr),
            Stmt::BlockStmt(block) => self.visit_main(block),
            Stmt::ExprStmt(expr) => self.visit_expr(expr)
        };
    }
    fn visit_expr(&mut self, expr: Expr) -> DataType{
        return match expr{
            Expr::DataExpr(data) => data,
            Expr::InputExpr(expr) => {
                self.clone().visit_print_stmt(*expr);
                let mut reff=String::new();
                match io::stdin().read_line(&mut reff){
                    Ok(_) => DataType::String(reff),
                    Err(err) => DataType::Error(err.to_string())
                }
            },
            Expr::AssingmentExpr(name,expr) => {
                let dat=self.visit_expr(*expr);
                self.memory.add_var(name.clone());
                self.memory.set_var(name, dat.clone());
                dat
            },
            Expr::IdentExpr(name) => {
                let env = self.clone().memory;
                env.get_var(name)
            }
        }
    }
    fn visit_print_stmt(mut self, expr: Expr) -> DataType{
        let val=self.visit_expr(expr);
        match val.clone(){
            DataType::Boolean(b) => println!("{}",b),
            DataType::Complex(re,im) => println!("{0}+{1}i",re,im),
            DataType::Integer(i) => println!("{}",i),
            DataType::Null => println!("null"),
            DataType::Number(n) => println!("{}",n),
            DataType::String(string) => println!("{}",string),
            DataType::Error(ec) => println!("{}",ec)
        }
        val
    }
}