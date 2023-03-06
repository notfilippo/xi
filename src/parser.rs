use miette::Result;

use crate::{
    expr::{Expr, ExprKind, Stmt, StmtKind},
    report::{InvalidAssignmentTarget, UnexpectedEof, UnexpectedToken},
    token::{Literal, Span, Token, TokenKind},
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
        result
    }

    fn next_is(&mut self, f: fn(TokenKind) -> bool) -> Option<&Token> {
        match self.peek() {
            Some(token) => match f(token.kind) {
                true => self.next(),
                false => None,
            },
            None => None,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn peek_force(&mut self) -> Result<&Token> {
        self.peek().ok_or(
            UnexpectedEof {
                span: self.previous().span.into(),
                src: self.source.to_string(),
            }
            .into(),
        )
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: TokenKind) -> Result<()> {
        let token = self.peek_force()?;
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

    fn previous_identifier(&self) -> String {
        if let Literal::Identifier(name) = self.previous().clone().literal.unwrap() {
            name
        } else {
            unreachable!();
        }
    }

    fn span(&mut self, start: usize) -> Span {
        let start_span = self.tokens[start].span;
        let end_span = self.previous().span;
        Span::new_range(start_span.offset, end_span.offset + end_span.length)
    }

    fn primary(&mut self) -> Result<Box<Expr>> {
        let start = self.current;

        if self.next_is(|k| k == TokenKind::False).is_some() {
            return Ok(Box::new(Expr {
                kind: ExprKind::Literal {
                    value: Value::False,
                },
                span: self.span(start),
            }));
        }

        if self.next_is(|k| k == TokenKind::True).is_some() {
            return Ok(Box::new(Expr {
                kind: ExprKind::Literal { value: Value::True },
                span: self.span(start),
            }));
        }

        if self.next_is(|k| k == TokenKind::Nil).is_some() {
            return Ok(Box::new(Expr {
                kind: ExprKind::Literal { value: Value::Nil },
                span: self.span(start),
            }));
        }

        if let Some(token) =
            self.next_is(|k| matches!(k, TokenKind::String | TokenKind::Float | TokenKind::Integer))
        {
            return Ok(Box::new(Expr {
                kind: ExprKind::Literal {
                    value: token.literal.clone().unwrap().into(),
                },
                span: self.span(start),
            }));
        }

        if self.next_is(|k| k == TokenKind::Identifier).is_some() {
            return Ok(Box::new(Expr {
                kind: ExprKind::Variable {
                    name: self.previous_identifier(),
                },
                span: self.span(start),
            }));
        }

        if self.next_is(|k| k == TokenKind::LeftParen).is_some() {
            let expr = self.expression()?;
            self.consume(TokenKind::RightParen)?;
            return Ok(Box::new(Expr {
                kind: ExprKind::Grouping { expr },
                span: self.span(start),
            }));
        }

        let token = self.peek_force()?;
        Err(UnexpectedToken {
            span: token.span.into(),
            help: format!("wanted primary expr, found {:?}", token.kind),
            src: self.source.to_string(),
        }
        .into())
    }

    fn unary(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        if let Some(op) = self.next_is(|a| matches!(a, TokenKind::Bang | TokenKind::Minus)) {
            Ok(Box::new(Expr {
                kind: ExprKind::Unary {
                    op: op.clone(),
                    right: self.primary()?,
                },
                span: self.span(start),
            }))
        } else {
            self.primary()
        }
    }

    fn factor(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        let mut expr = self.unary()?;

        while let Some(op) = self.next_is(|k| matches!(k, TokenKind::Slash | TokenKind::Star)) {
            expr = Box::new(Expr {
                kind: ExprKind::Binary {
                    left: expr,
                    op: op.clone(),
                    right: self.unary()?,
                },
                span: self.span(start),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        let mut expr = self.factor()?;

        while let Some(op) = self.next_is(|k| matches!(k, TokenKind::Minus | TokenKind::Plus)) {
            expr = Box::new(Expr {
                kind: ExprKind::Binary {
                    left: expr,
                    op: op.clone(),
                    right: self.factor()?,
                },
                span: self.span(start),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        let mut expr = self.term()?;

        while let Some(op) = self.next_is(|k| {
            matches!(
                k,
                TokenKind::Greater
                    | TokenKind::GreaterEqual
                    | TokenKind::Less
                    | TokenKind::LessEqual
            )
        }) {
            expr = Box::new(Expr {
                kind: ExprKind::Binary {
                    left: expr,
                    op: op.clone(),
                    right: self.term()?,
                },
                span: self.span(start),
            });
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        let mut expr = self.comparison()?;

        while let Some(op) =
            self.next_is(|k| matches!(k, TokenKind::BangEqual | TokenKind::EqualEqual))
        {
            expr = Box::new(Expr {
                kind: ExprKind::Binary {
                    left: expr,
                    op: op.clone(),
                    right: self.comparison()?,
                },
                span: self.span(start),
            });
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        let mut expr = self.equality()?;

        while self.next_is(|k| k == TokenKind::And).is_some() {
            let op = self.previous().clone();
            let right = self.equality()?;
            expr = Box::new(Expr {
                kind: ExprKind::Logical {
                    left: expr,
                    op,
                    right,
                },
                span: self.span(start),
            })
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        let mut expr = self.and()?;

        while self.next_is(|k| k == TokenKind::Or).is_some() {
            let op = self.previous().clone();
            let right = self.and()?;
            expr = Box::new(Expr {
                kind: ExprKind::Logical {
                    left: expr,
                    op,
                    right,
                },
                span: self.span(start),
            })
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        let to = self.or()?;

        if self.next_is(|k| k == TokenKind::Equal).is_some() {
            let expr = self.assignment()?;

            if let ExprKind::Variable { name } = to.kind {
                Ok(Box::new(Expr {
                    kind: ExprKind::Assign { name, expr },
                    span: self.span(start),
                }))
            } else {
                Err(InvalidAssignmentTarget {
                    span: self.span(start).into(),
                    src: self.source.to_string(),
                }
                .into())
            }
        } else {
            Ok(to)
        }
    }

    fn expression(&mut self) -> Result<Box<Expr>> {
        self.assignment()
    }

    fn _sync(&mut self) -> Result<()> {
        while let Some(next) = self.next() {
            if next.kind == TokenKind::Semicolon {
                break;
            }

            if matches!(
                self.peek_force()?.kind,
                TokenKind::Class
                    | TokenKind::Fn
                    | TokenKind::Let
                    | TokenKind::For
                    | TokenKind::If
                    | TokenKind::While
                    | TokenKind::Print
                    | TokenKind::Return
            ) {
                break;
            }
        }

        Ok(())
    }

    pub fn expression_statement(&mut self) -> Result<Box<Stmt>> {
        let start = self.current;
        let value = self.expression()?;

        if self.peek().is_some() {
            self.consume(TokenKind::Semicolon)?;
        }

        Ok(Box::new(Stmt {
            kind: StmtKind::Expression { expr: value },
            span: self.span(start),
        }))
    }

    pub fn if_statement(&mut self) -> Result<Box<Stmt>> {
        let start = self.current;
        self.consume(TokenKind::LeftParen)?;
        let cond = self.expression()?;
        self.consume(TokenKind::RightParen)?;

        let then_branch = self.statement()?;
        let else_branch = if self.next_is(|k| k == TokenKind::Else).is_some() {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Box::new(Stmt {
            kind: StmtKind::If {
                cond,
                then_branch,
                else_branch,
            },
            span: self.span(start),
        }))
    }

    pub fn for_statement(&mut self) -> Result<Box<Stmt>> {
        let start = self.current;

        self.consume(TokenKind::LeftParen)?;

        let initializer = if self.next_is(|k| k == TokenKind::Semicolon).is_some() {
            None
        } else if self.next_is(|k| k == TokenKind::Let).is_some() {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let cond = match self.peek_force()?.kind {
            TokenKind::Semicolon => Box::new(Expr {
                kind: ExprKind::Literal { value: Value::True },
                span: self.peek_force()?.span,
            }),
            _ => self.expression()?,
        };

        self.consume(TokenKind::Semicolon)?;

        let increment = match self.peek_force()?.kind {
            TokenKind::RightParen => None,
            _ => Some(self.expression()?),
        };

        self.consume(TokenKind::RightParen)?;

        let mut body = self.statement()?;
        let end = self.current;

        if let Some(increment) = increment {
            body = Box::new(Stmt {
                kind: StmtKind::Block {
                    statements: vec![
                        *body,
                        Stmt {
                            kind: StmtKind::Expression { expr: increment },
                            span: Span::new_range(start, end),
                        },
                    ],
                },
                span: Span::new_range(start, end),
            });
        }

        body = Box::new(Stmt {
            kind: StmtKind::While { cond, body },
            span: Span::new_range(start, end),
        });

        if let Some(initializer) = initializer {
            body = Box::new(Stmt {
                kind: StmtKind::Block {
                    statements: vec![*initializer, *body],
                },
                span: Span::new_range(start, end),
            })
        }

        println!("{:#?}", body);

        Ok(body)
    }

    fn print_statement(&mut self) -> Result<Box<Stmt>> {
        let start = self.current;
        let value = self.expression()?;

        if self.peek().is_some() {
            self.consume(TokenKind::Semicolon)?;
        }

        Ok(Box::new(Stmt {
            kind: StmtKind::Print { expr: value },
            span: self.span(start),
        }))
    }

    fn while_statement(&mut self) -> Result<Box<Stmt>> {
        let start = self.current;
        self.consume(TokenKind::LeftParen)?;
        let cond = self.expression()?;
        self.consume(TokenKind::RightParen)?;

        let body = self.statement()?;

        Ok(Box::new(Stmt {
            kind: StmtKind::While { cond, body },
            span: self.span(start),
        }))
    }

    fn get_block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        while self.peek_force()?.kind != TokenKind::RightBrace {
            statements.push(*self.declaration()?);
        }

        self.consume(TokenKind::RightBrace)?;

        Ok(statements)
    }

    fn block(&mut self) -> Result<Box<Stmt>> {
        let start = self.current;
        let statements = self.get_block()?;

        Ok(Box::new(Stmt {
            kind: StmtKind::Block { statements },
            span: self.span(start),
        }))
    }

    fn statement(&mut self) -> Result<Box<Stmt>> {
        if self.next_is(|k| k == TokenKind::For).is_some() {
            self.for_statement()
        } else if self.next_is(|k| k == TokenKind::If).is_some() {
            self.if_statement()
        } else if self.next_is(|k| k == TokenKind::Print).is_some() {
            self.print_statement()
        } else if self.next_is(|k| k == TokenKind::While).is_some() {
            self.while_statement()
        } else if self.next_is(|k| k == TokenKind::LeftBrace).is_some() {
            self.block()
        } else {
            self.expression_statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Box<Stmt>> {
        let start = self.current;

        self.consume(TokenKind::Identifier)?;
        let name = self.previous_identifier();

        let initializer = if self.next_is(|k| k == TokenKind::Equal).is_some() {
            Some(self.expression()?)
        } else {
            None
        };

        if self.peek().is_some() {
            self.consume(TokenKind::Semicolon)?;
        }

        Ok(Box::new(Stmt {
            kind: StmtKind::Var { name, initializer },
            span: self.span(start),
        }))
    }

    fn declaration(&mut self) -> Result<Box<Stmt>> {
        if self.next_is(|k| k == TokenKind::Let).is_some() {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        while self.peek().is_some() {
            statements.push(*self.declaration()?);
        }
        Ok(statements)
    }
}
