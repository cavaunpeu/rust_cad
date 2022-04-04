#[cfg(test)]
mod test {
  use core::panic;
  use rust_cad::{spaces::Dimension, errors::FreezingError};

  #[test]
  fn test_dim_creation() {
    let dim_a = Dimension::<String>::new("a", "", false);
    assert_eq!(dim_a.get_dtype(), "alloc::string::String");
  }

  #[test]
  fn test_dim_print() {
    let dim_a = Dimension::<String>::new("a", "", false);
    let dim_b = Dimension::<i64>::new("b", "Desc B", false);
    let dim_c = Dimension::<f64>::new("c", "Desc C", true);
    let dim_d = Dimension::<i64>::new("d", "", true);

    assert_eq!(format!("{:?}", dim_a), "Mutable dimension a has data type alloc::string::String and no description");
    assert_eq!(format!("{:?}", dim_b), "Mutable dimension b has data type i64 and the following description:\nDesc B\n");
    assert_eq!(format!("{:?}", dim_c), "Frozen dimension c has data type f64 and the following description:\nDesc C\n");
    assert_eq!(format!("{:?}", dim_d), "Frozen dimension d has data type i64 and no description");
  }

  #[test]
  fn test_dim_get_set() {
    let mut dim_a = Dimension::<String>::new("a", "", false);
    let dim_b = Dimension::<i64>::new("b", "Desc B", false);
    let dim_c = Dimension::<f64>::new("c", "Desc C", true);
    let dim_d = Dimension::<i64>::new("d", "", true);

    assert_eq!(dim_b.get_name(), "b");
    assert_eq!(dim_a.get_description(), "");
    assert_eq!(dim_c.get_description(), "Desc C");
    assert_eq!(dim_d.get_dtype(), "i64");

    assert!(!dim_a.is_frozen());
    assert!(!dim_b.is_frozen());
    assert!(dim_c.is_frozen());
    assert!(dim_d.is_frozen());

    dim_a.set_name("New Name A").unwrap();
    assert_eq!(dim_a.get_name(), "New Name A");

    dim_a.set_name("New Description A").unwrap();
    assert_eq!(dim_a.get_name(), "New Description A");

    dim_a.freeze();
    assert!(dim_a.is_frozen())
  }

  #[test]
  fn test_dim_freezing() {
    let mut dim_c = Dimension::<f64>::new("c", "Desc C", true);

    match dim_c.set_name("New Name C") {
      Err(e) => assert_eq!(e, FreezingError{ description: "Cannot set name on frozen dimension" }),
      Ok(()) => panic!("Test should not reach here")
    }

    match dim_c.set_description("New Description C") {
      Err(e) => assert_eq!(e, FreezingError{ description: "Cannot set description on frozen dimension" }),
      Ok(()) => panic!("Test should not reach here")
    }
  }

  #[test]
  fn test_dim_equality() {
    let dim_a = Dimension::<i64>::new("a", "", false);
    let dim_b = Dimension::<i64>::new("b", "Description", false);
    let dim_c = Dimension::<f64>::new("d", "Description", true);
    let dim_d = Dimension::<f64>::new("d", "Description", true);

    assert_ne!(dim_a, dim_b);
    assert_eq!(dim_c, dim_d);
  }
}