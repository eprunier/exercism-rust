extern crate chrono;
use chrono::*;

pub fn after(date: DateTime<UTC>) -> chrono::DateTime<chrono::UTC> {
	date + Duration::seconds(1_000_000_000)
}