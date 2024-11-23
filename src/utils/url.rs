use regex::Regex;
use url::Url;

const MAX_URL_LENGTH: usize = 2048;

pub async fn url_format_check(url: &str) -> bool {
    if let Ok(re) = Regex::new(r"^https?:\/\/[\w/:%#\$&\?\(\)~\.=\+\-]+$") {
        return re.is_match(url);
    }

    false
}

pub fn character_limit_check(url: &str) -> bool {
	return url.len() > MAX_URL_LENGTH;
}

pub fn domain_check(url: &str) -> bool {
	if let Ok(issue_list_url) = Url::parse(url) {
		return issue_list_url.host_str() == Some("jli.li");
	}

	false
}
