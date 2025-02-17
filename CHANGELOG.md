# Changelog

## Unreleased

- Add functions for punycode
  - `parser::punycode_encode_host(host)`
  - `parser::punycode_decode_host(host)`
  - `parser::is_puny(host)`

## 0.12.0 - Bug fixes

- fix: parse fediverse addresses as text, so they are not mistaken for email addresses ([issue #82](https://github.com/deltachat/message-parser/issues/82))
- fix: don't consume/eat the the exlaimation mark at the end of a link #85

## 0.11.0 - Bug fixes for Link Parsing

### Fixed
- fix: restrict elements that can appear inside a label for a labeled link ([issue #59](https://github.com/deltachat/message-parser/issues/59))
- fix: Generic schemes were linkified even without content & also a lone http:// was linkified ([issue #71](https://github.com/deltachat/message-parser/issues/71))

## 0.10.0 - Specification compliant detection for internationalized links

### Added
- Add new methods for working with emojis (they are standalone helper functions and not part of message parsing):
  - `parser::is_emoji::emoji`(rust only) - nom parser that eats one emoji
  - `parser::is_emoji::get_first_emoji(text)` - get first emoji if text begins with an emoji
  - `parser::is_emoji::count_emojis_if_only_contains_emoji(text)` - counts emojis in texts that contain only emojis
- Parse IRI links (Links that contain non ASCII characters in location part) - link parsing is now RFC3987 and RFC3988 compliant.

### Changed
- upgrade rust toolchain to 1.77.2
- improved example page (added example text)

### Fixed
- fix absolute unix paths being detected as bot commands suggestions
- fix parenthesis in target of labeled link

## 0.9.0 - Improve BotCommandSuggestion Parsing

### Fixed
- fix bot command suggestion with `@`- char was detected as email address

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
