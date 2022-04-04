use std::{fmt, error::Error};

#[derive(Debug, Clone, PartialEq)]
pub struct FreezingError<'a> {
  pub description: &'a str
}

impl<'a> Error for FreezingError<'a> {}

impl<'a> fmt::Display for FreezingError<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.description)
  }
}