use crate::ast::{Type, Token, Literal, Expr};

pub struct Parser {
    current: usize,
    pub tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let current = 0;
        Parser { current, tokens }
    }

    pub fn parse(&mut self) -> Result<Expr, &'static str> {
        let expr = self.expression();
        if !self.is_at_end() {
            return Err("unexpected end of input");
        }
        return expr;
    }

    fn condition(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![Type::Odd]) {
            let expr = self.expression();
            if expr.is_ok() {
                return Ok(Expr::OddExpr(Box::new(expr.unwrap())));
            } else {
                return expr;
            }
        } else {
            let expr = self.expression();
            if self.match_token(vec![Type::Less, Type::LessEqual,
                                     Type::Greater, Type::GreaterEqual,
                                     Type::Hash, Type::Equal]) {
                let operator = self.previous();
                let right = self.expression();
                if expr.is_ok() && right.is_ok() {
                    return Ok(Expr::Expr(Box::new(expr.unwrap()), operator.r#type, Box::new(right.unwrap())));
                } else {
                    return expr.and(right);
                }
            } else {
                return Err("invalid condition");
            }
        }
    }

    fn expression(&mut self) -> Result<Expr, &'static str> {
        let prefix;
        if self.match_token(vec![Type::Plus, Type::Minus]) {
            prefix = Some(self.previous().r#type);
        } else {
            prefix = None
        }

        let term = self.term();
        while self.match_token(vec![Type::Plus, Type::Minus]) {
            let operator = self.previous();
            let right = self.term();
            if term.is_ok() && right.is_ok() {
                return Ok(Expr::PrefixExpr(prefix, Box::new(Expr::Expr(Box::new(term.unwrap()), operator.r#type, Box::new(right.unwrap())))))
            } else {
                return term.and(right);
            }
        }

        if term.is_ok() {
            return Ok(Expr::PrefixExpr(prefix, Box::new(term.unwrap())));
        } else {
            return Err("")
        }
    }

    fn term(&mut self) -> Result<Expr, &'static str> {
        let factor = self.factor();
        while self.match_token(vec![Type::Star, Type::Slash]) {
            let operator = self.previous();
            let right = self.factor();

            if factor.is_ok() && right.is_ok() {
                return Ok(Expr::Expr(Box::new(factor.unwrap()), operator.r#type, Box::new(right.unwrap())))
            } else {
                return factor.and(right);
            }
        }

        factor
    }

    fn factor(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![Type::Identifier]) {
            return Ok(Expr::Var(self.previous().lexeme));
        }

        if self.match_token(vec![Type::Number]) {
            return Ok(Expr::Literal(self.previous().literal.unwrap()));
        }

        return Err("expected identifier or number");
    }

    fn match_token(&mut self, tokens: Vec<Type>) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&mut self, token_type: Type) -> bool {
        if self.is_at_end() {
            return false;
        }
        let token = self.peek();
        token.r#type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        let token = self.peek();
        token.r#type == Type::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        return self.tokens[self.current - 1].clone();
    }
}
