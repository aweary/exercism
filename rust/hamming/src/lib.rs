
pub fn hamming_distance(a: &str, b: &str) -> Result<usize, String> {
  match a.len() == b.len() {
    true => Ok(a.chars().zip(b.chars()).filter(|x| x.0 != x.1).count()),
    false => Err("Strings need to be of equal length".to_string())
  }
}