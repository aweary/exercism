pub fn square_of_sum(n: i64) -> i64 {
  (1..n + 1).sum::<i64>().pow(2)
}

pub fn sum_of_squares(n: i64) -> i64 {
  (1..n + 1).map(|x| x.pow(2)).sum()
}

pub fn difference(n: i64) -> i64 {
  square_of_sum(n) - sum_of_squares(n)
}