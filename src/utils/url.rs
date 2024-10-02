use regex::Regex;

pub fn url_format_check(url: &str) -> bool {
	if let Ok(re) = Regex::new(r"^https?:\/\/[\w/:%#\$&\?\(\)~\.=\+\-]+$") {
		return re.is_match(url);
	}

	false
}