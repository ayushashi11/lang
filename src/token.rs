//use std::boxed::Box
#[derive(Clone,Debug)]
pub enum DataType{
    Integer(i32),
    Number(f64),
    String(String),
    Complex(f64,f64),
    Boolean(bool),
    Error(String),
    Null
}
#[derive(Clone)]
pub enum Expr{
    IdentExpr(String),
    DataExpr(DataType),
    AssingmentExpr(String,Box<Expr>),
    InputExpr(Box<Expr>)
}
#[derive(Clone)]
pub enum Stmt{
    ExprStmt(Expr),
    PrintStmt(Expr),
    BlockStmt(Block)
}
pub type Block=Vec<Stmt>;