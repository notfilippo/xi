use std::rc::Rc;

use miette::Result;

use crate::{
    expr::{Expr, ExprKind, Stmt, StmtKind},
    report::{InvalidAssignmentTarget, UnexpectedEof, UnexpectedToken},
    token::{Literal, Span, Token, TokenKind},
    value::Value,
};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
    current_id: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            current_id: 0,
        }
    }

    fn next_id(&mut self) -> usize {
        self.current_id += 1;
        self.current_id
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

                id: self.next_id(),
            }));
        }

        if self.next_is(|k| k == TokenKind::True).is_some() {
            return Ok(Box::new(Expr {
                kind: ExprKind::Literal { value: Value::True },
                span: self.span(start),

                id: self.next_id(),
            }));
        }

        if self.next_is(|k| k == TokenKind::Nil).is_some() {
            return Ok(Box::new(Expr {
                kind: ExprKind::Literal { value: Value::Nil },
                span: self.span(start),

                id: self.next_id(),
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

                id: self.next_id(),
            }));
        }

        if self.next_is(|k| k == TokenKind::Identifier).is_some() {
            return Ok(Box::new(Expr {
                kind: ExprKind::Variable {
                    name: self.previous_identifier(),
                },
                span: self.span(start),

                id: self.next_id(),
            }));
        }

        if self.next_is(|k| k == TokenKind::LeftParen).is_some() {
            let value = self.expression()?;
            self.consume(TokenKind::RightParen)?;
            return Ok(Box::new(Expr {
                kind: ExprKind::Grouping { value },
                span: self.span(start),

                id: self.next_id(),
            }));
        }

        let token = self.peek_force()?;
        Err(UnexpectedToken {
            span: token.span.into(),
            help: format!("wanted primary expr, found {:?}", token.kind),
        }
        .into())
    }

    fn finish_call(&mut self, start: usize, callee: Box<Expr>) -> Result<Box<Expr>> {
        let mut args = Vec::new();
        if self.peek_force()?.kind != TokenKind::RightParen {
            loop {
                args.push(*self.expression()?);
                if self.next_is(|k| k == TokenKind::Comma).is_none() {
                    break;
                }
            }
        }

        self.consume(TokenKind::RightParen)?;

        Ok(Box::new(Expr {
            kind: ExprKind::Call { callee, args },
            span: self.span(start),
            id: self.next_id(),
        }))
    }

    fn call(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        let mut expr = self.primary()?;

        loop {
            if self.next_is(|k| k == TokenKind::LeftParen).is_some() {
                expr = self.finish_call(start, expr)?;
            } else if self.next_is(|k| k == TokenKind::LeftSquare).is_some() {
                let index = self.expression()?;
                self.consume(TokenKind::RightSquare)?;

                expr = Box::new(Expr {
                    kind: ExprKind::GetIndex { obj: expr, index },
                    span: self.span(start),
                    id: self.next_id(),
                })
            } else if self.next_is(|k| k == TokenKind::Dot).is_some() {
                self.consume(TokenKind::Identifier)?;
                let name = self.previous_identifier();
                expr = Box::new(Expr {
                    kind: ExprKind::Get { obj: expr, name },
                    span: self.span(start),
                    id: self.next_id(),
                })
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn list(&mut self) -> Result<Box<Expr>> {
        let start = self.current;

        let mut items = Vec::new();
        if self.peek_force()?.kind != TokenKind::RightSquare {
            loop {
                items.push(*self.expression()?);
                if self.next_is(|k| k == TokenKind::Comma).is_none() {
                    break;
                }
            }
        }

        self.consume(TokenKind::RightSquare)?;

        Ok(Box::new(Expr {
            kind: ExprKind::List { items },
            span: self.span(start),
            id: self.next_id(),
        }))
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

                id: self.next_id(),
            }))
        } else if let Some(_op) = self.next_is(|a| matches!(a, TokenKind::LeftSquare)) {
            self.list()
        } else {
            self.call()
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

                id: self.next_id(),
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

                id: self.next_id(),
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

                id: self.next_id(),
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

                id: self.next_id(),
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

                id: self.next_id(),
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

                id: self.next_id(),
            })
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Box<Expr>> {
        let start = self.current;
        let expr = self.or()?;

        if self.next_is(|k| k == TokenKind::Equal).is_some() {
            let value = self.assignment()?;

            match expr.kind {
                ExprKind::Get { obj, name } => Ok(Box::new(Expr {
                    kind: ExprKind::Set { obj, name, value },
                    span: self.span(start),
                    id: self.next_id(),
                })),
                ExprKind::Variable { name } => Ok(Box::new(Expr {
                    kind: ExprKind::Assign { name, value },
                    span: self.span(start),
                    id: self.next_id(),
                })),
                ExprKind::GetIndex { obj, index } => Ok(Box::new(Expr {
                    kind: ExprKind::SetIndex { obj, index, value },
                    span: self.span(start),
                    id: self.next_id(),
                })),
                _ => Err(InvalidAssignmentTarget {
                    span: self.span(start).into(),
                }
                .into()),
            }
        } else {
            Ok(expr)
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
                TokenKind::Fn
                    | TokenKind::Let
                    | TokenKind::For
                    | TokenKind::If
                    | TokenKind::While
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

            id: self.next_id(),
        }))
    }

    pub fn if_statement(&mut self) -> Result<Box<Stmt>> {
        let start = self.current - 1;
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

            id: self.next_id(),
        }))
    }

    pub fn for_statement(&mut self) -> Result<Box<Stmt>> {
        let start = self.current - 1;

        self.consume(TokenKind::LeftParen)?;

        let initializer = if self.next_is(|k| k == TokenKind::Semicolon).is_some() {
            None
        } else if self.next_is(|k| k == TokenKind::Let).is_some() {
            Some(self.let_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let cond = match self.peek_force()?.kind {
            TokenKind::Semicolon => Box::new(Expr {
                kind: ExprKind::Literal { value: Value::True },
                span: self.peek_force()?.span,
                id: self.next_id(),
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

        if let Some(increment) = increment {
            body = Box::new(Stmt {
                kind: StmtKind::Block {
                    statements: vec![
                        *body,
                        Stmt {
                            kind: StmtKind::Expression { expr: increment },
                            span: self.span(start),
                            id: self.next_id(),
                        },
                    ],
                },
                span: self.span(start),
                id: self.next_id(),
            });
        }

        body = Box::new(Stmt {
            kind: StmtKind::While { cond, body },
            span: self.span(start),
            id: self.next_id(),
        });

        if let Some(initializer) = initializer {
            body = Box::new(Stmt {
                kind: StmtKind::Block {
                    statements: vec![*initializer, *body],
                },
                span: self.span(start),
                id: self.next_id(),
            })
        }

        Ok(body)
    }

    fn return_statement(&mut self) -> Result<Box<Stmt>> {
        let start = self.current - 1;
        let expr = if self.peek().is_some() {
            if self.peek_force()?.kind == TokenKind::Semicolon {
                None
            } else {
                Some(self.expression()?)
            }
        } else {
            None
        };

        if self.peek().is_some() {
            self.consume(TokenKind::Semicolon)?;
        }

        Ok(Box::new(Stmt {
            kind: StmtKind::Return { expr },
            span: self.span(start),

            id: self.next_id(),
        }))
    }

    fn while_statement(&mut self) -> Result<Box<Stmt>> {
        let start = self.current - 1;
        self.consume(TokenKind::LeftParen)?;
        let cond = self.expression()?;
        self.consume(TokenKind::RightParen)?;

        let body = self.statement()?;

        Ok(Box::new(Stmt {
            kind: StmtKind::While { cond, body },
            span: self.span(start),
            id: self.next_id(),
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
            id: self.next_id(),
        }))
    }

    fn statement(&mut self) -> Result<Box<Stmt>> {
        if self.next_is(|k| k == TokenKind::For).is_some() {
            self.for_statement()
        } else if self.next_is(|k| k == TokenKind::If).is_some() {
            self.if_statement()
        } else if self.next_is(|k| k == TokenKind::Return).is_some() {
            self.return_statement()
        } else if self.next_is(|k| k == TokenKind::While).is_some() {
            self.while_statement()
        } else if self.next_is(|k| k == TokenKind::LeftBrace).is_some() {
            self.block()
        } else {
            self.expression_statement()
        }
    }

    fn function(&mut self) -> Result<Box<Stmt>> {
        let start = self.current;

        self.consume(TokenKind::Identifier)?;
        let name = self.previous_identifier();

        self.consume(TokenKind::LeftParen)?;

        let mut params = Vec::new();
        if self.peek_force()?.kind != TokenKind::RightParen {
            loop {
                self.consume(TokenKind::Identifier)?;
                params.push(self.previous_identifier());

                if self.next_is(|k| k == TokenKind::Comma).is_none() {
                    break;
                }
            }
        }

        self.consume(TokenKind::RightParen)?;
        self.consume(TokenKind::LeftBrace)?;
        let body = self.get_block()?;

        Ok(Box::new(Stmt {
            kind: StmtKind::Function {
                name,
                params: Rc::new(params),
                body: Rc::new(body),
            },
            span: self.span(start),
            id: self.next_id(),
        }))
    }

    fn let_declaration(&mut self) -> Result<Box<Stmt>> {
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
            kind: StmtKind::Let { name, initializer },
            span: self.span(start),

            id: self.next_id(),
        }))
    }

    fn declaration(&mut self) -> Result<Box<Stmt>> {
        if self.next_is(|k| k == TokenKind::Fn).is_some() {
            self.function()
        } else if self.next_is(|k| k == TokenKind::Let).is_some() {
            self.let_declaration()
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
