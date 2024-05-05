use deltachat_message_parser::parser::Element::*;
use deltachat_message_parser::parser::LinkDestination;

fn gopher_link_no_puny<'a>(target: &'a str, hostname: &'a str) -> LinkDestination<'a> {
    LinkDestination {
        target,
        hostname: Some(hostname),
        scheme: "gopher",
        punycode: None,
    }
}

fn http_link_no_puny<'a>(target: &'a str, hostname: &'a str) -> LinkDestination<'a> {
    LinkDestination {
        target,
        hostname: Some(hostname),
        scheme: "http",
        punycode: None,
    }
}

fn ftp_link_no_puny<'a>(target: &'a str, hostname: &'a str) -> LinkDestination<'a> {
    LinkDestination {
        target,
        hostname: Some(hostname),
        scheme: "ftp",
        punycode: None,
    }
}

fn https_link_no_puny<'a>(target: &'a str, hostname: &'a str) -> LinkDestination<'a> {
    LinkDestination {
        target,
        hostname: Some(hostname),
        scheme: "https",
        punycode: None,
    }
}

fn mailto_link_no_puny<'a>(target: &'a str) -> LinkDestination<'a> {
    LinkDestination {
        target,
        hostname: None,
        scheme: "mailto",
        punycode: None,
    }
}

mod desktop_set;
mod markdown;
mod text_only;
