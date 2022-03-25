use std::{marker::PhantomData, any::type_name, fmt};

pub struct Dimension<'a, T: 'a> {
  dtype: PhantomData<T>,
  name: &'a str,
  description: &'a str,
  frozen: bool,
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

  pub fn get_type(&self) -> &'static str {
    type_name::<T>()
  }
}

impl<'a, T> fmt::Debug for Dimension<'a, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let first_word = if !self.frozen { "Mutable" } else { "Frozen" };
    match self {
      d if !self.description.is_empty() => write!(f, "{} dimension {} has data type {} and the following description:\n{}\n", first_word, d.name, d.get_type(), d.description),
      d => write!(f, "{} dimension {} has data type {} and no description", first_word, d.name, d.get_type()),
    }
  }
}