// use crate::{expr::Expr, token::Token, token_type::TokenType};

// pub struct Parser {
//     tokens: Vec<Token>,
//     current: usize,
// }

// impl Parser {
//     pub fn new(tokens: Vec<Token>) -> Self {
//         Self {
//             tokens,
//             current: 0,
//         }
//     }

//     fn expression(&mut self) -> Expr {
//         return self.equality();
//     }

//     fn equality(&mut self) -> Expr {
//         let exp = comparison();

//         while match_token() {
            
//         }
//     }
// }