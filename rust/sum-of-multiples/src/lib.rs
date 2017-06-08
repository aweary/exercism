pub fn sum_of_multiples(n: i64, multiples: &Vec<i64>) -> i64 {
  (1..n).filter(|x| multiples.iter().any(|i| x % i == 0)).sum()
}