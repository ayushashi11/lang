use crate::token::*;
#[derive(Clone)]
pub struct Executor{
    prog: Stmt
}
impl Executor{
    pub fn new(program: Stmt) -> Self{
        Executor{prog: program}
    }
    
    pub fn visitRoot(self){
        self.visitStmt(self.prog);
    }
    
    pub fn visitStmt(self, stmt: Stmt){
        match stmt{
            Stmt::PrintStmt(expr) => self.visitPrintStmt(expr)
        };
    }
    fn visitExpr(self, expr: Expr) -> DataType{
        return match expr{
            Expr::DataExpr(data) => data,
            _ => DataType::Null
        }
    }
    fn visitPrintStmt(self, expr: Expr){
        let val=self.visitExpr(expr);
        match val{
            DataType::Boolean(b) => println!("{}",b),
            DataType::Complex(re,im) => println!("{0}+{1}i",re,im),
            DataType::Integer(i) => println!("{}",i),
            DataType::Null => println!("null"),
            DataType::Number(n) => println!("{}",n),
            DataType::Pointer(id,_) => println!("ptr{}",id),
            DataType::String(string) => println!("{}",string)
        }
    }
}