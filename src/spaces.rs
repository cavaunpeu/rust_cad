use std::{marker::PhantomData, any::type_name, fmt};

pub struct Dimension<T> {
  dtype: PhantomData<T>,
  name: String,
  description: String,
  frozen: bool,
}

impl<T> Dimension<T> {
  pub fn new(name: String, description: String, frozen: bool) -> Self {
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

impl<T> fmt::Debug for Dimension<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let first_word = if !self.frozen { "Mutable" } else { "Frozen" };
    match self {
      d if !self.description.is_empty() => write!(f, "{} dimension {} has data type {} and the following description:\n{}\n", first_word, d.name, d.get_type(), d.description),
      d => write!(f, "{} dimension {} has data type {} and no description", first_word, d.name, d.get_type()),
    }
  }
}