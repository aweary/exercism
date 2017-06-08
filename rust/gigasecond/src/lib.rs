extern crate chrono;
use chrono::*;
use std::ops::Add;

pub fn after(start_date: DateTime<UTC>) -> DateTime<UTC> {
  start_date.add(chrono::Duration::seconds(1_000_000_000))
} 
