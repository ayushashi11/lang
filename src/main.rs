mod token;
mod executor;
use token::*;
use executor::*;
fn main() {
    let program=Stmt::PrintStmt(Expr::DataExpr(DataType::String(String::from("hello world!"))));
    let exec=Executor::new(program);
    exec.visitRoot();
}
