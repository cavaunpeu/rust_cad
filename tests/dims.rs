#[cfg(test)]
mod test {
  use rust_cad::spaces::Dimension;

  #[test]
  fn test_dim_print() {
    let dim_a = Dimension::<String>::new(String::from("a"), String::from(""), false);
    let dim_b = Dimension::<i64>::new(String::from("b"), String::from("Desc B"), false);
    let dim_c = Dimension::<f64>::new(String::from("c"), String::from("Desc C"), true);
    let dim_d = Dimension::<i64>::new(String::from("d"), String::from(""), true);

    assert_eq!(format!("{:?}", dim_a), "Mutable dimension a has data type alloc::string::String and no description");
    assert_eq!(format!("{:?}", dim_b), "Mutable dimension b has data type i64 and the following description:\nDesc B\n");
    assert_eq!(format!("{:?}", dim_c), "Frozen dimension c has data type f64 and the following description:\nDesc C\n");
    assert_eq!(format!("{:?}", dim_d), "Frozen dimension d has data type i64 and no description");
  }
}