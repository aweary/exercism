
pub fn is_pangram(s: &str) -> bool {
    let mut chars: Vec<char> = s.to_lowercase()
        .chars()
        .filter(|&x| match x {
            'a'...'z' => true,
            _ => false,
        })
        .collect();
    chars.sort();
    chars.dedup();
    println!("{:?}", chars);
    chars.len() == 26
}
