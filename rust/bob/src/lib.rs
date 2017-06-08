pub fn reply(s: &str) -> &str {
  match s {
    x if x.ends_with("?") => "Sure.",
    x if x.is_empty() => "Fine. Be that way!",
    x if x.to_uppercase() == x => "Whoa, chill out!",
    _ => "Whatever.",
  }
}