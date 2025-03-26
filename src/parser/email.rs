///! Parsing / Validation of email addresses
///
/// It should follow [RFC 5322](https://datatracker.ietf.org/doc/html/rfc5322#section-1)
/// and [RFC 6532](https://datatracker.ietf.org/doc/html/rfc6532)

#[cfg(test)]
mod test_email_validation {

    // Credits of original test data: https://github.com/Sayan751/email-address-parser
    // License of original test data: MIT
    // original sources: https://github.com/Sayan751/email-address-parser/blob/df6de2b99db5dc31b5eab4258f64ea4809e932cc/rust-lib/.test_data/
    const invalid_domains: Vec<&'static str> = vec![
        r#""#,
        r#"example.com>"#,
        r#"test..com"#,
        r#".test.com"#,
        r#"test.com."#,
        r#"google.."#,
        r#".com"#,
        r#"google-.com"#,
        r#"-google-.com"#,
    ];
    const invalid_local_parts: Vec<&'static str> = vec![
        r#""#,
        r#"a\\ud83c"#,
        r#"\"test"#,
        r#"test."#,
        r#"te..st"#,
        r#"<test"#,
        r#"test>"#,
        r#".test"#,
        r#"\"test\\ t\"est\""#,
        r#"\"test t\"est\""#,
        r#"te\";\"st"#,
        r#"te;st"#,
    ];
    const valid_domains: Vec<&'static str> = vec![
        r#"google.com"#,
        r#"m.maselkowski.pl"#,
        r#"xn--masekowski-d0b.pl"#,
        r#"[127.0.0.0]"#,
        r#"[192.168.0.1]"#,
        r#"[1.2.3.4]"#,
        r#"[0.0.0.1]"#,
        r#"[255.255.255.254]"#,
        r#"masełkowski.pl.com"#,
        r#"中国互联网络信息中心.中国"#,
        r#"中国互联网络信息中心.xn--masekowski-d0b"#,
        r#"bücher.com"#,
        r#"nächste.de"#,
        r#"löwen.de"#,
        r#"افغانستا.com"#,
        r#"বাংলাদেশ.com"#,
        r#"беларусь.com"#,
        r#"belgië.com"#,
        r#"českárepublika.com"#,
        r#"مصر.com"#,
        r#"ελλάδα.com"#,
        r#"ísland.com"#,
        r#"भारत.com"#,
        r#"איקו״ם.ישראל.com"#,
        r#"қазақстан.com"#,
        r#"한국.com"#,
        r#"кыргызстан.com"#,
        r#"ລາວ.com"#,
        r#"македонија.com"#,
        r#"монголулс.com"#,
        r#"россия.иком.com"#,
        r#"இலங்கை.com"#,
        r#"españa.com"#,
        r#"ไทย.com"#,
        r#"việtnam.com"#,
        r#"📻.fm"#,
        r#"🎁.de"#,
        r#"âêîôû.Çéàèù.ëïü"#,
    ];
    const valid_local_parts: Vec<&'static str> = vec![
        r#"test"#,
        r#"a"#,
        r#"!#$%&'*+-/=?^_`{|}~"#,
        r#"\"test test\""#,
        r#"\"test\\ test\""#,
        r#"te.st"#,
        r#"\"te\\,st\""#,
        r#"\"te\\;st\""#,
    ];

    #[test]
    fn domain_part_invalid() {
        // parse invalid domains parts
    }

    #[test]
    fn domain_part_valid() {
        // parse valid domains parts
    }

    #[test]
    fn local_part_invalid() {
        // parse invalid local parts
    }

    #[test]
    fn local_part_valid() {
        // parse valid local parts
    }

    #[test]
    fn local_part_valid() {
        // parse valid local parts
    }

    fn combinde_email_parts(local_parts: Vec<&string>, domain_parts: Vec<&string>) -> Vec<String> {
        let mut emails: Vec<String> = Vec::new();
        for domain_part in domain_parts {
            for local_part in local_parts {
                emails.push(format!("{}@{}", local_part, domain_part));
            }
        }
        emails
    }

    #[test]
    fn parse_valid_emails() {
        let emails = combinde_email_parts(valid_local_parts, valid_domains);

        for email in emails {
            assert_eq(true /* todo parse email */, true)
        }
    }

    fn parse_invalid_emails() {
        let emails = combinde_email_parts(invalid_local_parts, invalid_domains)
            .extend(combinde_email_parts(valid_local_parts, invalid_domains))
            .extend(combinde_email_parts(invalid_local_parts, valid_domains));

        for email in emails {
            assert_eq(false /* todo parse email */, false)
        }
    }
}
