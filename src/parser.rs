use crate::ast::{Type, Token, Literal, Expr, Block};

#[derive(Debug)]
pub struct Parser {
    pub current: usize,
    pub tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let current = 0;
        Parser { current, tokens }
    }

    pub fn parse(&mut self) -> Result<Block, &'static str> {
        let program = self.program();
        /*
        if !self.match_token(vec![Type::EOF]) {
            return Err("unexpected end of input");
        }
        */
        return program;
    }

    fn program(&mut self) -> Result<Block, &'static str> {
        let block = self.block();
        if block.is_err() {
            return block;
        }

        let dot = self.expect(Type::Dot, "expected dot to end program");
        if dot.is_err() {
            return Err(dot.unwrap_err());
        }

        Ok(Block::Program(Box::new(block.unwrap())))
    }

    fn block(&mut self) -> Result<Block, &'static str> {
        let mut const_decs = Vec::new();
        if self.match_token(vec![Type::Const]) {
            loop {
                let ident = self.expect(Type::Identifier, "identifier");
                if ident.is_err() {
                    return Err(ident.unwrap_err());
                }
                let ident = Expr::Var(ident.unwrap().lexeme);

                if !self.match_token(vec![Type::Equal]) {
                    return Err("expected '=' in const expression");
                }

                let number = self.expect(Type::Number, "number");
                if number.is_err() {
                    return Err(number.unwrap_err());
                }
                let number = Expr::Literal(number.unwrap().literal.unwrap());

                let const_dec = Block::Const(ident, number);
                const_decs.push(Box::new(const_dec));

                if !self.match_token(vec![Type::Comma]) {
                    break;
                }
            }
            let semi = self.expect(Type::Semicolon,
                                   "missing semicolon after const decs");
            if semi.is_err() {
                return Err(semi.unwrap_err());
            }
        }

        let mut var_decs = Vec::new();
        if self.match_token(vec![Type::Var]) {
            loop {
                let ident = self.expect(Type::Identifier, "identifier");
                if ident.is_err() {
                    return Err(ident.unwrap_err());
                }
                let ident = Expr::Var(ident.unwrap().lexeme);

                var_decs.push(Box::new(ident));

                if !self.match_token(vec![Type::Comma]) {
                    break;
                }
            }
            let semi = self.expect(Type::Semicolon,
                                   "missing semicolon after var decs");
            if semi.is_err() {
                return Err(semi.unwrap_err());
            }
        }
        let var_decs = Block::VarDecs(var_decs);

        let mut procedures = Vec::new();
        while self.match_token(vec![Type::Procedure]) {
            let ident = self.expect(Type::Identifier, "missing procedure identifier");
            if ident.is_err() {
                return Err(ident.unwrap_err());
            }
            let ident = Expr::Var(ident.unwrap().lexeme);

            let semi = self.expect(Type::Semicolon,
                                   "missing semicolon after procedure identifier");
            if semi.is_err() {
                return Err(semi.unwrap_err());
            }

            let block = self.block();
            if block.is_err() {
                return Err(block.unwrap_err());
            }

            let semi = self.expect(Type::Semicolon,
                                   "missing semicolon after procedure block");
            if semi.is_err() {
                return Err(semi.unwrap_err());
            }

            let procedure = Block::Procedure(ident, Box::new(block.unwrap()));
            procedures.push(Box::new(procedure));
        }

        let statement = self.statement();
        if statement.is_err() {
            return Err(statement.unwrap_err());
        }

        return Ok(Block::Block(const_decs,
                               Box::new(var_decs),
                               procedures,
                               Box::new(statement.unwrap())));
    }

    fn statement(&mut self) -> Result<Block, &'static str> {
        // Assignment
        if self.match_token(vec![Type::Identifier]) {
            let var = self.previous();

            let coloneq = self.expect(Type::ColonEqual, "missing colon equal");
            if coloneq.is_err() {
                return Err(coloneq.unwrap_err())
            }

            let right = self.expression();
            if right.is_ok() {
                return Ok(Block::Assign(Expr::Var(var.lexeme), right.unwrap()));
            } else {
                return Err(right.unwrap_err());
            }

        // Call Statement
        } else if self.match_token(vec![Type::Call]) {
            let ident = self.expect(Type::Identifier, "call missing identifier");
            if ident.is_err() {
                return Err(ident.unwrap_err());
            }

            let ident = Expr::Var(ident.unwrap().lexeme);
            return Ok(Block::Call(ident));

        // Begin block
        } else if self.match_token(vec![Type::Begin]) {
            let mut statements = Vec::new();
            loop {
                let statement = self.statement();
                if statement.is_err() {
                    return statement;
                }

                statements.push(Box::new(statement.unwrap()));
                if !self.match_token(vec![Type::Semicolon]) {
                    break;
                }
            }
            let end = self.expect(Type::End, "missing end keyword or semicolon");
            if end.is_err() {
                return Err(end.unwrap_err());
            }
            return Ok(Block::Begin(statements));

        // If block
        } else if self.match_token(vec![Type::If]) {
            let condition = self.condition();
            if condition.is_err() {
                return Err(condition.unwrap_err());
            }

            let then = self.expect(Type::Then, "missing then keyword");
            if then.is_err() {
                return Err(then.unwrap_err());
            }

            let body = self.statement();

            if body.is_ok() {
                return Ok(Block::If(condition.unwrap(),
                                    Box::new(body.unwrap())));
            } else {
                return condition.and(body);
            }

        // While block
        } else if self.match_token(vec![Type::While]) {
            let condition = self.condition();

            let do_keyword = self.expect(Type::Do, "missing do keyword");
            if do_keyword.is_err() {
                return Err(do_keyword.unwrap_err());
            }

            let body = self.statement();

            if condition.is_ok() && body.is_ok() {
                return Ok(Block::While(condition.unwrap(),
                                       Box::new(body.unwrap())));
            } else {
                return condition.and(body);
            }

        // WriteLn
        } else if self.match_token(vec![Type::Bang]) {
            let expression = self.expression();

            if expression.is_err() {
                return Err(expression.unwrap_err());
            }

            return Ok(Block::WriteLn(expression.unwrap()));
        }

        return Err("statement error");
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

        let mut term = self.term();
        while self.match_token(vec![Type::Plus, Type::Minus]) {
            let operator = self.previous();
            let right = self.term();
            if term.is_ok() && right.is_ok() {
                term = Ok(Expr::Expr(Box::new(term.unwrap()), operator.r#type, Box::new(right.unwrap())));
            } else {
                return term.and(right);
            }
        }

        if term.is_ok() {
            return Ok(Expr::PrefixExpr(prefix, Box::new(term.unwrap())));
        } else {
            return term;
        }
    }

    fn term(&mut self) -> Result<Expr, &'static str> {
        let mut factor = self.factor();
        while self.match_token(vec![Type::Star, Type::Slash]) {
            let operator = self.previous();
            let right = self.factor();

            if factor.is_ok() && right.is_ok() {
                factor = Ok(Expr::Expr(Box::new(factor.unwrap()), operator.r#type, Box::new(right.unwrap())))
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

        // TODO Handle parens

        return Err("expected expression");
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

    fn expect(&mut self,
              token: Type,
              err: &'static str) -> Result<Token, &'static str> {
        if self.check(token) {
            let token: Token = self.tokens[self.current].clone();
            self.advance();
            return Ok(token);
        }
        Err(err)
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
