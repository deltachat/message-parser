// this is to protect against https://en.wikipedia.org/wiki/IDN_homograph_attack

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub struct PunycodeWarning {
    pub original_hostname: String,
    pub ascii_hostname: String,
    pub punycode_encoded_url: String,
}

/// encode a host to punycode encoded string
pub fn punycode_encode_host(host: &str) -> String {
    host.split('.')
        .map(|sub| {
            if is_puny(sub) {
                format!(
                    "xn--{}",
                    unic_idna_punycode::encode_str(sub)
                        .unwrap_or_else(|| "[punycode encode failed]".to_owned())
                )
            } else {
                sub.to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join(".")
}

/// Returns host as decoded unicode string
pub fn punycode_decode_host(host: &str) -> String {
    host
        .split('.')
        .map(|sub| {
            if let Some(sub) = sub.strip_prefix("xn--") {
                unic_idna_punycode::decode_to_string(sub)
                    .unwrap_or_else(|| "[punycode decode failed]".to_owned())
            } else {
                sub.to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join(".")
}

/// Returns true if host string contains non ASCII characters
pub fn is_puny(host: &str) -> bool {
    for ch in host.chars() {
        if !(ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-')) {
            return true;
        }
    }
    false
}

/// Return a PunycodeWarning struct if host need punycode encoding else None
pub fn get_puny_code_warning(link: &str, host: &str) -> Option<PunycodeWarning> {
    if is_puny(host) {
        let ascii_hostname = punycode_encode_host(host);
        Some(PunycodeWarning {
            original_hostname: host.to_owned(),
            ascii_hostname: ascii_hostname.to_owned(),
            punycode_encoded_url: link.replacen(host, &ascii_hostname, 1),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{is_puny, punycode_decode_host, punycode_encode_host};


    #[test]
    fn is_puny_positive() {
        assert!(is_puny("münchen.de"));
        assert!(is_puny("wikipediа.org"));
    }

    #[test]
    fn is_puny_negative() {
        assert!(!is_puny("muenchen.de"));
        assert!(!is_puny("delta.chat"));
    }

    #[test]
    fn encode_host(){
        assert_eq!(punycode_encode_host("münchen.de"), "xn--mnchen-3ya.de");
        assert_eq!(punycode_encode_host("wikipediа.org"), "xn--wikipedi-86g.org");
    }

    #[test]
    fn decode_host(){
        assert_eq!(punycode_decode_host("xn--mnchen-3ya.de"), "münchen.de");
        assert_eq!(punycode_decode_host("xn--wikipedi-86g.org"), "wikipediа.org");
    }
}