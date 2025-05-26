mod country_tlds;

const ALLOWED_TOP_LEVEL_DOMAINS: &[&str] = &[
    // originals from RFC920 + net
    "com", "org", "net", "edu", "gov", "mil", // for deltachat
    "chat",
];

pub fn check_if_tld_is_allowed(tld: &str) -> bool {
    if ALLOWED_TOP_LEVEL_DOMAINS.iter().any(|item|*item == tld) {
        true
    } else { country_tlds::COUNTRY_TLDS.binary_search(&tld).is_ok() }
}

#[cfg(test)]
mod test {
    use crate::parser::link_url::allowed_tlds::check_if_tld_is_allowed;

    #[test]
    fn test_check_tld() {
        assert!(check_if_tld_is_allowed("chat"));
        assert!(check_if_tld_is_allowed("com"));

        assert!(check_if_tld_is_allowed("de"));
        assert!(check_if_tld_is_allowed("at"));
        assert!(check_if_tld_is_allowed("uk"));
        assert!(check_if_tld_is_allowed("fr"));
    }

    #[test]
    fn test_check_tld_not_allowed() {
        assert!(!check_if_tld_is_allowed("doesnotexist"));
    }
}