use deltachat_message_parser::parser::Element::*;
use deltachat_message_parser::parser::{LinkDestination, PunycodeWarning};

fn http_link_no_puny(target: &str, hostname: &str) -> LinkDestination {
    LinkDestination {
        target,
        hostname: Some(hostname),
        scheme: "http",
        punycode: None
    }
}

fn https_link_no_puny(target: &str, hostname: &str) -> LinkDestination {
    LinkDestination {
        target,
        hostname: Some(hostname),
        scheme: "http",
        punycode: None
    }
}

fn http_link_no_puny(target: &str, hostname: &str) -> LinkDestination {
    LinkDestination {
        target,
        hostname: Some(hostname),
        scheme: "ftp",
        punycode: None
    }
}

fn mailto_link_no_puny(target: &str, hostname: &str) -> LinkDestination {
    LinkDestination {
        target,
        hostname: Some(hostname),
        scheme: "mailto",
        punycode: None,
    }
}

mod desktop_set;
mod markdown;
mod text_only;
