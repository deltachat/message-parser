use deltachat_message_parser::parser::Element::*;
use deltachat_message_parser::parser::LinkDestination;

pub(crate) fn gopher_link_no_puny<'a>(target: &'a str, hostname: &'a str) -> LinkDestination<'a> {
    LinkDestination {
        target,
        hostname: Some(hostname),
        scheme: Some("gopher"),
        punycode: None,
    }
}

pub(crate) fn http_link_no_puny<'a>(target: &'a str, hostname: &'a str) -> LinkDestination<'a> {
    LinkDestination {
        target,
        hostname: Some(hostname),
        scheme: Some("http"),
        punycode: None,
    }
}

pub(crate) fn ftp_link_no_puny<'a>(target: &'a str, hostname: &'a str) -> LinkDestination<'a> {
    LinkDestination {
        target,
        hostname: Some(hostname),
        scheme: Some("ftp"),
        punycode: None,
    }
}

pub(crate) fn https_link_no_puny<'a>(target: &'a str, hostname: &'a str) -> LinkDestination<'a> {
    LinkDestination {
        target,
        hostname: Some(hostname),
        scheme: Some("https"),
        punycode: None,
    }
}

pub(crate) fn mailto_link_no_puny(target: &str) -> LinkDestination<'_> {
    LinkDestination {
        target,
        hostname: None,
        scheme: Some("mailto"),
        punycode: None,
    }
}

mod desktop_set;
mod markdown;
mod text_only;
