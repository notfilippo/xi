use miette::Result;

use crate::{
    expr::Expr,
    report::{UnexpectedEof, UnexpectedToken},
    token::{Span, Token, TokenKind},
    value::Value,
};

pub struct Parser<'a> {
    source: &'a str,
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: &'a Vec<Token>) -> Self {
        Self {
            source,
            tokens,
            current: 0,
        }
    }

    fn next(&mut self) -> Option<&Token> {
        let result = self.tokens.get(self.current);
        self.current += 1;
        return result;
    }

    fn next_if(&mut self, f: fn(TokenKind) -> bool) -> Option<&Token> {
        match self.peek() {
            Some(token) => match f(token.kind) {
                true => self.next(),
                false => None,
            },
            None => None,
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn peek_is(&mut self, f: fn(TokenKind) -> bool) -> bool {
        match self.peek() {
            Some(token) => f(token.kind),
            None => false,
        }
    }

    fn previous(&mut self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: TokenKind) -> Result<()> {
        match self.peek() {
            Some(token) => {
                if token.kind == kind {
                    self.next();
                    Ok(())
                } else {
                    Err(UnexpectedToken {
                        span: token.span.into(),
                        help: format!("wanted {:?}, found {:?}", kind, token.kind),
                        src: self.source.to_string(),
                    }
                    .into())
                }
            }
            None => Err(UnexpectedEof {
                span: self.previous().span.into(),
                src: self.source.to_string(),
            }
            .into()),
        }
    }

    fn primary(&mut self) -> Result<Box<Expr>> {
        let start = self.current;

        if self.next_if(|k| k == TokenKind::False).is_some() {
            return Ok(Box::new(Expr::Literal {
                value: Value::False,
                span: Span::new_range(start, self.current),
            }));
        }
        if self.next_if(|k| k == TokenKind::True).is_some() {
            return Ok(Box::new(Expr::Literal {
                value: Value::True,
                span: Span::new_range(start, self.current),
            }));
        }
        if self.next_if(|k| k == TokenKind::Nil).is_some() {
            return Ok(Box::new(Expr::Literal {
                value: Value::Nil,
                span: Span::new_range(start, self.current),
            }));
        }

        if let Some(token) =
            self.next_if(|k| matches!(k, TokenKind::String | TokenKind::Float | TokenKind::Integer))
        {
            return Ok(Box::new(Expr::Literal {
                value: token.literal.clone().unwrap().into(),
                span: Span::new_range(start, self.current),
            }));
        }

        if self.next_if(|k| k == TokenKind::LeftParen).is_some() {
            let expr = self.expression()?;
            self.consume(TokenKind::RightParen)?;
            return Ok(Box::new(Expr::Grouping {
                expr,
                span: Span::new_range(start, self.current),
            }));
        }

        match self.peek() {
            Some(token) => Err(UnexpectedToken {
                span: token.span.into(),
                help: format!("wanted primary expr, found {:?}", token.kind),
                src: self.source.to_string(),
            }
            .into()),
            None => Err(UnexpectedEof {
                span: self.previous().span.into(),
                src: self.source.to_string(),
            }
            .into()),
        }
    }

    fn unary(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        if let Some(op) = self.next_if(|a| matches!(a, TokenKind::Bang | TokenKind::Minus)) {
            Ok(Box::new(Expr::Unary {
                op: op.clone(),
                right: self.primary()?,
                span: Span::new_range(start, self.current),
            }))
        } else {
            self.primary()
        }
    }

    fn factor(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        let mut expr = self.unary()?;

        while let Some(op) = self.next_if(|k| matches!(k, TokenKind::Slash | TokenKind::Star)) {
            expr = Box::new(Expr::Binary {
                left: expr,
                op: op.clone(),
                right: self.unary()?,
                span: Span::new_range(start, self.current),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        let mut expr = self.factor()?;

        while let Some(op) = self.next_if(|k| matches!(k, TokenKind::Minus | TokenKind::Plus)) {
            expr = Box::new(Expr::Binary {
                left: expr,
                op: op.clone(),
                right: self.factor()?,
                span: Span::new_range(start, self.current),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        let mut expr = self.term()?;

        while let Some(op) = self.next_if(|k| {
            matches!(
                k,
                TokenKind::Greater
                    | TokenKind::GreaterEqual
                    | TokenKind::Less
                    | TokenKind::LessEqual
            )
        }) {
            expr = Box::new(Expr::Binary {
                left: expr,
                op: op.clone(),
                right: self.term()?,
                span: Span::new_range(start, self.current),
            });
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        let mut expr = self.comparison()?;

        while let Some(op) =
            self.next_if(|k| matches!(k, TokenKind::BangEqual | TokenKind::EqualEqual))
        {
            expr = Box::new(Expr::Binary {
                left: expr,
                op: op.clone(),
                right: self.comparison()?,
                span: Span::new_range(start, self.current),
            });
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Box<Expr>> {
        self.equality()
    }

    fn sync(&mut self) {
        while let Some(next) = self.next() {
            if next.kind == TokenKind::Semicolon {
                return;
            }

            if self.peek_is(|k| {
                matches!(
                    k,
                    TokenKind::Class
                        | TokenKind::Fn
                        | TokenKind::Var
                        | TokenKind::For
                        | TokenKind::If
                        | TokenKind::While
                        | TokenKind::Print
                        | TokenKind::Return
                )
            }) {
                return;
            }
        }
    }

    pub fn scan_exprs(&mut self) -> Result<Box<Expr>> {
        return self.expression();
    }
}
