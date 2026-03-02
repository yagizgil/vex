use crate::ast::{expr::Expr, expr::LiteralValue, stmt::Stmt};
use crate::lexer::token::TokenType;
use crate::memory::Environment;
use crate::utils::logger::error::ErrorCode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
    pub current: Option<Stmt>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            current: None,
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) {
        for statement in statements {
            self.current = Some(statement.clone());
            self.execute(statement);
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Var {
                name,
                vtype,
                initializer,
            } => {
                let value = self.evaluate(initializer);
                self.environment.borrow_mut().define(name.lexeme.clone(), value);
            }
            Stmt::Expression(Expr::Call { callee, arguments }) => {
                if let Expr::Variable { name, index, .. } = &**callee {
                    if name.lexeme == "print" {
                        for arg in arguments {
                            let val = self.evaluate(arg);
                            println!("{:?}", val);
                        }
                    }
                }
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr);
            }
            Stmt::Block(statements) => {
                self.execute_block(statements);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_val = self.evaluate(condition);
                if self.is_truthy(&condition_val) {
                    self.execute_block(then_branch);
                } else if let Some(else_stmt) = else_branch {
                    self.execute(else_stmt);
                }
            }
            Stmt::While { condition, body } => loop {
                let condition_val = self.evaluate(condition);
                if !self.is_truthy(&condition_val) {
                    break;
                }
                self.execute_block(body);
            },
            _ => {}
        }
    }

    fn execute_block(&mut self, statements: &[Stmt]) {
        let previous = Rc::clone(&self.environment);
        let local_env = Environment::with_enclosing(previous);

        self.environment = local_env;
        self.interpret(statements);

        let parent = self
            .environment
            .borrow()
            .enclosing
            .clone()
            .expect("Parent scope not found.");
        self.environment = parent;
    }

    pub fn evaluate(&mut self, expr: &Expr) -> LiteralValue {
        match expr {
            Expr::Literal(value) => value.clone(),
            Expr::Variable { name, index } => {
                if let Some((distance, slot)) = index {
                    self.environment.borrow().get_at(*distance, *slot)
                } else {
                    self.environment.borrow().get_global(&name.lexeme)
                    // vex_int_panic!(
                    //     self.current.as_ref().map(|s| s.line()).unwrap_or(0),
                    //     ErrorCode::VarNotResolv,
                    //     Some(format!("Variable '{}' not resolved!", name.lexeme))
                    // );
                }
            }
            Expr::Assign { name, value, index } => {
                let val = self.evaluate(value);
                if let Some((distance, slot)) = index {
                    self.environment
                        .borrow_mut()
                        .assign_at(*distance, *slot, val.clone());
                } else {
                    self.environment.borrow_mut().assign_global(name.lexeme.clone(), val.clone());
                    // vex_int_panic!(
                    //     self.current.as_ref().map(|s| s.line()).unwrap_or(0),
                    //     ErrorCode::VarNotResolv,
                    //     Some(format!("Assignment to '{}' not resolved!", name.lexeme))
                    // );
                }
                val
            }
            Expr::Grouping(inner) => self.evaluate(inner),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let l = self.evaluate(left);
                let r = self.evaluate(right);
                self.apply_binary(l, operator.token_type.clone(), r)
            }
            Expr::Unary { operator, right } => {
                let r = self.evaluate(right);
                match operator.token_type {
                    TokenType::Minus => self.negate(r),
                    TokenType::Bang => LiteralValue::Bool(!self.is_truthy(&r)),
                    _ => LiteralValue::Null,
                }
            }
            _ => LiteralValue::Null,
        }
    }

    fn is_truthy(&self, val: &LiteralValue) -> bool {
        match val {
            LiteralValue::Null => false,
            LiteralValue::Bool(b) => *b,
            _ => true,
        }
    }

    fn apply_binary(&mut self, l: LiteralValue, op: TokenType, r: LiteralValue) -> LiteralValue {
        match (l, op, r) {
            (LiteralValue::Number(n1), TokenType::Plus, LiteralValue::Number(n2)) => {
                LiteralValue::Number(n1 + n2)
            }
            (LiteralValue::Number(n1), TokenType::Minus, LiteralValue::Number(n2)) => {
                LiteralValue::Number(n1 - n2)
            }
            (LiteralValue::Number(n1), TokenType::Star, LiteralValue::Number(n2)) => {
                LiteralValue::Number(n1 * n2)
            }
            (LiteralValue::Number(n1), TokenType::Slash, LiteralValue::Number(n2)) => {
                if n2 == 0.0 {
                    vex_int_panic!(
                        self.current.as_ref().map(|s| s.line()).unwrap_or(0),
                        ErrorCode::MathDivideByZero,
                        None
                    );
                }
                LiteralValue::Number(n1 / n2)
            }
            (LiteralValue::Number(n1), TokenType::Greater, LiteralValue::Number(n2)) => {
                LiteralValue::Bool(n1 > n2)
            }
            (LiteralValue::Number(n1), TokenType::GreaterEqual, LiteralValue::Number(n2)) => {
                LiteralValue::Bool(n1 >= n2)
            }
            (LiteralValue::Number(n1), TokenType::Less, LiteralValue::Number(n2)) => {
                LiteralValue::Bool(n1 < n2)
            }
            (LiteralValue::Number(n1), TokenType::LessEqual, LiteralValue::Number(n2)) => {
                LiteralValue::Bool(n1 <= n2)
            }
            (LiteralValue::Str(s1), TokenType::Plus, LiteralValue::Str(s2)) => {
                LiteralValue::Str(format!("{}{}", s1, s2))
            }
            _ => LiteralValue::Null,
        }
    }

    fn negate(&mut self, val: LiteralValue) -> LiteralValue {
        match val {
            LiteralValue::Number(n) => LiteralValue::Number(-n),
            _ => {
                vex_int_err!(
                    self.current.as_ref().map(|s| s.line()).unwrap_or(0),
                    ErrorCode::Unknown,
                    Some(format!(
                        "Error: '-' operator only works with numbers, incoming value: {:?}",
                        val
                    ))
                );
                LiteralValue::Null
            }
        }
    }
}
