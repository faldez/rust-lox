use crate::token::{DataType, Token};

macro_rules! parenthesize {
    ($name:expr, $($exprs:ident),+) => {{
        let mut result = String::new();
        result.push_str("(");
        result.push_str(&$name);
        $(result.push_str(" ");result.push_str(AstPrinter::visit_expr(&$exprs).as_str());)+
        result.push_str(")");
        result
    }};
}

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Option<DataType>),
    Unary(Token, Box<Expr>),
}

pub trait Visitor<T> {
    fn visit_expr(expr: &Expr) -> T;
}

pub struct AstPrinter {}

impl Visitor<String> for AstPrinter {
    fn visit_expr(expr: &Expr) -> String {
        match expr {
            Expr::Binary(left, operator, right) => parenthesize!(operator.lexeme, left, right),
            Expr::Grouping(expression) => parenthesize!("group", expression),
            Expr::Literal(value) => {
                if let Some(value) = value {
                    format!("{}", value)
                } else {
                    "None".to_string()
                }
            }
            Expr::Unary(operator, right) => parenthesize!(operator.lexeme, right),
        }
    }
}
