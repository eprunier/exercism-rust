pub fn is_leap_year(year: u16) -> bool {
	is_divisible_by(4, year) && !is_divisible_by(100, year) || is_divisible_by(400, year)
}

/// Test if value is divisible by divider
fn is_divisible_by(divider: u16, value: u16) -> bool {
	value % divider == 0
}
