use deltachat_message_parser::parser::is_emoji::emoji;

#[test]
fn test_all_desktop_emoji_picker_emojis() {
    let testcases = include_str!("./all_desktop_emojis.txt").split(',');
    let mut total = 0;
    let mut failed = vec![];
    for case in testcases {
        if emoji(case).is_err() {
            failed.push(case);
        }
        total += 1;
    }
    let failed_count = failed.len();
    if !failed.is_empty() {
        println!("Failed Cases");
        print!("{}",failed.join(", "));
    }
    assert_eq!(
        failed_count, 0,
        "out of {total} cases {failed_count} failed"
    );
}
