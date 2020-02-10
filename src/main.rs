mod token;
mod executor;
mod scope;
use token::*;
use scope::*;
use executor::*;
fn main() {
    let mut program:Block=Block::new();
    program.push(
        Stmt::ExprStmt(
            Expr::AssingmentExpr(
                String::from("name"),
                Box::new(
                    Expr::InputExpr(
                        Box::new(
                            Expr::DataExpr(
                                DataType::String(
                                    String::from("Enter your name")
                                )
                            )
                        )
                    )
                )
            )
        )
    );
    program.push(
        Stmt::PrintStmt(
            Expr::DataExpr(
                DataType::String(
                    String::from("hello")
                )
            )
        )
    );
    program.push(
        Stmt::PrintStmt(
            Expr::IdentExpr(String::from("name"))
        )
    );
    let mut exec=Executor::new();
    exec.visit_main(program);
    /*
    let mut sc=Scope::new();
    sc.add_var(String::from("name")); sc.set_var(String::from("name"), DataType::Complex(4.0,-5.0));
    exec.visit_stmt(Stmt::PrintStmt(
        Expr::DataExpr(
            sc.get_var(String::from("name"))
        )
    ));
    */
}
