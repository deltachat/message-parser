use deltachat_message_parser::parser::Element::*;
use deltachat_message_parser::parser::LinkDestination;

pub fn link_destination_for_testing(trusted_real_url: &str) -> LinkDestination {
    LinkDestination::parse(trusted_real_url).unwrap().1
}

mod desktop_set;
mod markdown;
mod text_only;
