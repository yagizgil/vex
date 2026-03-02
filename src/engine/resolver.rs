use crate::ast::expr::{Expr, LiteralValue};
use crate::ast::stmt::Stmt;
use std::collections::HashMap;

pub struct Resolver {
    scopes: Vec<HashMap<String, usize>>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
        }
    }

    pub fn resolve_statements(&mut self, statements: &mut [Stmt]) {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: String) {
        if let Some(scope) = self.scopes.last_mut() {
            let index = scope.len();
            scope.insert(name, index);
        }
    }

    fn resolve_stmt(&mut self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Var { name, initializer, .. } => {
                self.resolve_expr(initializer);
                self.declare(name.lexeme.clone());
            }

            Stmt::Fn { name, params, body } => {
                self.declare(name.lexeme.clone());
                
                self.begin_scope();
                for param in params {
                    self.declare(param.lexeme.clone());
                }
                self.resolve_statements(body);
                self.end_scope();
            }

            Stmt::Block(statements) => {
                self.begin_scope();
                self.resolve_statements(statements);
                self.end_scope();
            }

            Stmt::Expression(expression) => {
                self.resolve_expr(expression);
            }

            Stmt::If { condition, then_branch, else_branch } => {
                self.resolve_expr(condition);
                self.resolve_statements(then_branch);
                if let Some(else_b) = else_branch {
                    self.resolve_stmt(else_b);
                }
            }

            Stmt::While { condition, body } => {
                self.resolve_expr(condition);
                self.resolve_statements(body);
            }

            Stmt::For { item, iterable, body } => {
                self.resolve_expr(iterable);
                self.begin_scope();
                self.declare(item.lexeme.clone());
                self.resolve_statements(body);
                self.end_scope();
            }

            Stmt::Return { value, .. } => {
                if let Some(expr) = value {
                    self.resolve_expr(expr);
                }
            }

            Stmt::Break | Stmt::Continue => {}

            _ =>{}
        }
    }

    fn resolve_expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Variable { name, index } => {
                for (distance, scope) in self.scopes.iter().rev().enumerate() {
                    if let Some(slot) = scope.get(&name.lexeme) {
                        *index = Some((distance, *slot));
                        return;
                    }
                }
            }

            Expr::Assign { name, value, index } => {
                self.resolve_expr(value);
                for (distance, scope) in self.scopes.iter().rev().enumerate() {
                    if let Some(slot) = scope.get(&name.lexeme) {
                        *index = Some((distance, *slot));
                        return;
                    }
                }
            }

            Expr::Binary { left, right, .. } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }

            Expr::Unary { right, .. } => {
                self.resolve_expr(right);
            }

            Expr::Grouping(inner) => {
                self.resolve_expr(inner);
            }

            Expr::Call { callee, arguments } => {
                self.resolve_expr(callee);
                for arg in arguments {
                    self.resolve_expr(arg);
                }
            }

            Expr::Literal(_) => {}
        }
    }
}