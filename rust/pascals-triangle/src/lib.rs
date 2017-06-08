pub struct PascalsTriangle(u32);

fn factorial(n: u32) -> u32 {
    (1..n + 1).fold(1, |acc, i| acc * i)
}

fn choose(n: u32, k: u32) -> u32 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.0)
            .map(|x| (x, vec![1u32; (x + 1) as usize]))
            .map(|(x, v)| v.iter().enumerate().map(|(i, _)| choose(x, i as u32)).collect())
            .collect()
    }
}
