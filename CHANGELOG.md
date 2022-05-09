# Changelog

## Unreleased

## 0.4.0 - Fixing Email, Preserve the Dots!

## Changed

- update rust toolchain to `1.60.0`
- enable more clippy lints to prevent panics

## Fixed

- fix: do not parse last dot for email addresses #19

## 0.3.0 - Squashing Link Bugs

## Changed

- `()`, `[]`, `{}`, `<>` brackets are now parsed as part of links (but only if they are opened and then closed again properly)

## Fixed

- count brackets in links to fix #12
- fix links eating trailing `.`,`,`,`;` or `:` #13

## 0.2.0 - Initial version

This version marks the beginning of using the message parser inside of deltacht-desktop.
