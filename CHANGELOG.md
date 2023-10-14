# Changelog

## Unreleased

- add @mentions
  - new `Element::Mention{ address: &str }`
  - new api `extract_mention_addresses` to extract

## 0.8.0 - Nom 7 and more Generic URI Schemes

### Changed
- upgraded nom to 7
- The following generic schemes (schemes that don't end in `://`) get linkified now:
    `mailto:`, `news:`, `feed:`, `tel:`, `sms:`, `geo:`, `maps:`, `bitcoin:`, `bitcoincash:`, `eth:`, `ethereum:`, `magnet:`
- added `scheme` property to `LinkDestination` 

## 0.7.0 - All the Hashtags

### Added

 - hashtag parsing per UAX31

## 0.6.0 - Fix email in parentheses and wrong IPv6 puny code warning

### Fixed
 - Fixed problem of IPv6 links being detected as punycode
 - fixed: Fixed the bug of brackets being parsed as part of the email address(#34)

## 0.5.0 - Delimited email addresses and fixing greedy codeblock

### Added

- support for `<delimited@email.address>`

### Fixed

- fix: code block with emojis removed some chars at the end

## 0.4.0 - Fixing Email, Preserve the Dots!

### Changed

- update rust toolchain to `1.60.0`
- enable more clippy lints to prevent panics

### Fixed

- fix: do not parse last dot for email addresses #19

## 0.3.0 - Squashing Link Bugs

### Changed

- `()`, `[]`, `{}`, `<>` brackets are now parsed as part of links (but only if they are opened and then closed again properly)

### Fixed

- count brackets in links to fix #12
- fix links eating trailing `.`,`,`,`;` or `:` #13

## 0.2.0 - Initial version

This version marks the beginning of using the message parser inside of deltacht-desktop.
