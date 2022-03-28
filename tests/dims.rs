#[cfg(test)]
mod test {
  use rust_cad::spaces::Dimension;

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
}