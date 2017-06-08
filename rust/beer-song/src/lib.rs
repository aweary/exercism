fn bottle_count<'a>(count: u8) -> String {
  match count {
    1 => String::from("1 bottle"),
    _ => format!("{} bottles", count),
  }
}

pub fn verse<'a>(count: u8) -> String {
    match count {
    0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
    1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
    _ => format!("{0} of beer on the wall, {0} of beer.\nTake one down and pass it around, {1} of beer on the wall.\n", bottle_count(count), bottle_count(count - 1)),
  }
}

pub fn sing<'a>(start: u8, end: u8) -> String {
  (end..start + 1)
    .map(|count| verse(count))
    .rev()
    .collect::<Vec<String>>()
    .join("\n")
}