pub fn square(s: u32) -> u64 {
  if s > 64 || s < 1 {
    panic!("Square must be between 1 and 64");
  }
  2u64.pow(s - 1)
}

pub fn total() -> u64 {
  // This is essentially a constant, so we
  // can just inline the number and return it.
  18_446_744_073_709_551_615
}
