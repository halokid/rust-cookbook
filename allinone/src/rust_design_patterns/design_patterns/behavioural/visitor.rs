/*
use crate::rust_design_patterns::design_patterns::behavioural::visitor::ast::{Expr, Name, Stmt};
use crate::rust_design_patterns::design_patterns::behavioural::visitor::visit::Visitor;

// The data we will visit
mod ast {
  pub enum Stmt {
    Expr(Expr),
    Let(Name, Expr),
  }

  pub struct Name {
    value: String,
  }

  pub enum Expr {
    IntLit(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
  }
}

// The abstract visitor
mod visit {
  use crate::rust_design_patterns::design_patterns::behavioural::visitor::ast;
  use ast::Name;
  use ast::Stmt;
  use ast::Expr;

  pub trait Visitor<T> {
    fn visit_name(&mut self, n: &Name) -> T;
    fn visit_stmt(&mut self, s: &Stmt) -> T;
    fn visit_expr(&mut self, e: &Expr) -> T;
  }
}

struct Interpreter;

impl Visitor<i64> for Interpreter {
  fn visit_name(&mut self, n: &Name) -> i64 {
    panic!()
  }

  fn visit_stmt(&mut self, s: &Stmt) -> i64 {
    match *s {
      Stmt::Expr(ref e_) => self.visit_expr(e),
      Stmt::Let(..) => unimplemented!(),
    }
  }

  fn visit_expr(&mut self, e: &Expr) -> i64 {
    match *e {
      Expr::IntLit(n) => n,

      Expr::Add(ref lhs, ref rhs) => {
        self.visit_expr(lhs) + self.visit_expr(rhs)
      },

      Expr::Sub(ref lhs, ref rhs) => {
        self.visit_expr(lhs) - self.visit_expr(rhs)
      }
    }
  }
}

 */





