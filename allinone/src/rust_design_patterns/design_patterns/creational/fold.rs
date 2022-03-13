
mod ast {
  pub enum Stmt {
    Expr(Box<Expr>),
    Let(Box<Name>, Box<Expr>),
  }

  pub struct Name {
    pub(crate) value: String,
  }

  pub enum Expr {
    IntLit(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
  }
}

mod fold {
  use crate::rust_design_patterns::design_patterns::creational::fold::ast::*;

  pub trait Folder {
    fn fold_name(&mut self, n: Box<Name>) -> Box<Name> { n }

    fn fold_stmt(&mut self, s: Box<Stmt>) -> Box<Stmt> {
      match *s {
        Stmt::Expr(e) => {
          Box::new(Stmt::Expr(self.fold_expr(e)))
        },

        Stmt::Let(n, e) => {
          Box::new(Stmt::Let(self.fold_name(n), self.fold_expr(e)))
        }
      }
    }

    fn fold_expr(&mut self, e: Box<Expr>) -> Box<Expr> {
      unimplemented!()
    }
  }
}

use fold::*;
use ast::*;

struct Renamer;

impl Folder for Renamer {
  fn fold_name(&mut self, n: Box<Name>) -> Box<Name> {
    Box::new(Name{
      value: "foo".to_string()
    })
  }
}
