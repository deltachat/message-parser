use deltachat_message_parser::parser::is_emoji::{
    count_emojis_if_only_contains_emoji, emoji, get_first_emoji,
};

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
        print!("{}", failed.join(", "));
    }
    assert_eq!(
        failed_count, 0,
        "out of {total} cases {failed_count} failed"
    );
}

#[test]
fn test_all_desktop_emoji_picker_emojis_are_single_emojis() {
    let testcases = include_str!("./all_desktop_emojis.txt").split(',');
    let mut total = 0;
    let mut failed = vec![];
    for case in testcases {
        if count_emojis_if_only_contains_emoji(case) != Some(1) {
            let result = get_first_emoji(case);
            if result != Some(case) {
                print!(
                    "{case}:\n{:?}\n{:?}\n",
                    result.map(|r| r.chars()),
                    case.chars()
                );
                failed.push(case);
            }
        }
        total += 1;
    }
    let failed_count = failed.len();
    if !failed.is_empty() {
        println!("Failed Cases");
        print!("{}", failed.join(", "));
    }
    assert_eq!(
        failed_count, 0,
        "out of {total} cases {failed_count} failed"
    );
}
