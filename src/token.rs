//use std::boxed::Box;
use std::borrow::Borrow;
#[derive(Clone)]
pub enum DataType{
    Integer(i32),
    Number(f64),
    String(String),
    Complex(f64,f64),
    Boolean(bool),
    Pointer(String,Box<DataType>),
    Null
}
#[derive(Clone)]
pub enum Expr{
    DataExpr(DataType),
    AssingmentExpr(String,Box<Expr>),
    InputExpr(Box<Expr>)
}
#[derive(Clone)]
pub enum Stmt{
    PrintStmt(Expr)
}