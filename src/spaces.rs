use std::{marker::PhantomData, any::type_name, fmt};

pub struct Dimension<'a, T> {
  dtype: PhantomData<T>,
  name: &'a str,
  description: &'a str,
  frozen: bool
}

impl<'a, T> Dimension<'a, T> {
  pub fn new(name: &'a str, description: &'a str, frozen: bool) -> Self {
    Dimension {
      dtype: PhantomData {},
      name,
      description,
      frozen
    }
  }

  pub fn is_frozen(&self) -> bool {
    self.frozen
  }

  pub fn freeze(&mut self) {
    self.frozen = true;
  }

  pub fn get_dtype(&self) -> &'static str {
    type_name::<T>()
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_description(&self) -> &str {
    &self.description
  }

  pub fn set_name(&mut self, name: &'a str) -> Result<(), String> {
    if !self.frozen {
      self.name = name;
      Ok(())
    } else {
      Err("Cannot set name on frozen dimension".into())
    }
  }

  pub fn set_description(&mut self, description: &'a str) -> Result<(), String> {
    if !self.frozen {
      self.description = description;
      Ok(())
    } else {
      Err("Cannot set description on frozen dimension".into())
    }
  }
}


impl<'a, T> fmt::Debug for Dimension<'a, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let first_word = if self.frozen { "Frozen" } else { "Mutable" };
    if !self.description.is_empty() {
      write!(f, "{} dimension {} has data type {} and the following description:\n{}\n", first_word, self.name, self.get_dtype(), self.description)
    } else {
      write!(f, "{} dimension {} has data type {} and no description", first_word, self.name, self.get_dtype())
    }
  }
}