pub fn reply(input: &str) -> &str {
	let result;

	if input.is_empty() {
		result = "Fine. Be that way!";
	} else if input.to_uppercase() == input && input.to_lowercase() != input {
		result = "Whoa, chill out!";
	} else if input.ends_with("?") {
		result = "Sure.";
	} else {
		result = "Whatever.";
	}

	result
}