use lox::{
    expr::{AstPrinter, Expr, Visitor},
    token::{DataType, Token},
    token_type::TokenType,
};

fn main() {
    let l = Expr::Literal(Some(DataType::Number(2.1)));
    let r = Expr::Literal(Some(DataType::Number(1.2)));
    let b = Expr::Binary(
        Box::new(l),
        Token::new(TokenType::Plus, "+".to_string(), None, 1),
        Box::new(r),
    );

    println!("{}", AstPrinter::visit_expr(&b));

    let l = Expr::Unary(
        Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line: 1,
        },
        Box::new(Expr::Literal(Some(DataType::Number(123_f64)))),
    );

    let b = Expr::Binary(
        Box::new(l),
        Token::new(TokenType::Star, "*".to_string(), None, 1),
        Box::new(Expr::Grouping(Box::new(Expr::Literal(Some(
            DataType::Number(45.67),
        ))))),
    );

    println!("{}", AstPrinter::visit_expr(&b));
}
