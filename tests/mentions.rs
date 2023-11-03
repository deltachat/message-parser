use deltachat_message_parser::parser::extract_mention_addresses;

#[test]
fn extract_mentions() {
    let mention_text = "Ping @email@address.tld and @email1@address.tld!";
    assert_eq!(
        extract_mention_addresses(mention_text),
        vec!["email1@address.tld", "email@address.tld"]
    )
}

#[test]
fn extract_mentions_are_deduped_and_sorted() {
    let mention_text =
        "Ping @email@address.tld, @abc@example.com, @abc@example.com and @email1@address.tld!\n@email1@address.tld your opinion would be especially helpful.";
    assert_eq!(
        extract_mention_addresses(mention_text),
        vec!["abc@example.com", "email1@address.tld", "email@address.tld"]
    )
}

#[test]
fn extract_mentions_false_positive() {
    let mention_text = "my text@example@example.com, more text";
    assert_eq!(
        extract_mention_addresses(mention_text),
        Vec::<String>::new()
    );
}
