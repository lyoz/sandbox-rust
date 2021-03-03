pub fn sort<T>(x: &mut [T], up: bool) {
  unimplemented!()
}

fn sub_sort<T>(x: &mut [T], up: bool) {
  unimplemented!()
}

fn compare_and_swap<T>(x: &mut [T], up: bool) {
  unimplemented!()
}

#[cfg(test)]
mod tests {
  use super::sort;

  #[test]
  fn sort_u32_ascending() {
    let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
    sort(&mut x, true);
    assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
  }

  #[test]
  fn sort_u32_descending() {
    let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
    sort(&mut x, false);
    assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
  }

  #[test]
  fn sort_str_ascending() {
    let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient"];
    sort(&mut x, true);
    assert_eq!(x, vec!["Rust", "and", "fast", "is", "memory-efficient"]);
  }

  #[test]
  fn sort_str_descending() {
    let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient"];
    sort(&mut x, true);
    assert_eq!(x, vec!["memory-efficient", "is", "fast", "and", "Rust"]);
  }
}
