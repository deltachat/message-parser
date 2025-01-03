## How to make a new release


1. `git pull` - make sure your local checkout is up to date
2. Ensure changelog entries exist for all relevant changes.
3. Make a new version in the changelog -> semver + some title that roughly describes the changes
4. update version in `Cargo.toml` and in `message_parser_wasm/Cargo.toml`. (if you run rust-analyzer then `Cargo.lock` is updated automatically)
5. make a commit with the changes "prepare <version>"
6. make a tag for the version
7. push both commit and tag
8. run `cargo publish`
9. run `cd message_parser_wasm && wasm-pack build --scope deltachat --target web && wasm-pack publish --target web`
10. make a release on github, copy the changelog for that version over to there