#![allow(unreachable_patterns)]

use std::fmt;

extern crate lalrpop_util;
#[macro_use] extern crate enum_methods;

pub mod ast;
pub mod tok;
pub mod parser;

pub unsafe trait SyntaxTrait { }

pub struct Postgres;
unsafe impl SyntaxTrait for Postgres { }
