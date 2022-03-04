# DeltaChat Message Parser

Parsing of Links, Email adresses, simple text formatting (markdown subset), user mentions, hashtags and more in DeltaChat messages.

The specification can be found in [spec.md](./spec.md).

WASM Demo: <https://deltachat.github.io/message-parser/>

## Idea behind it

Have the same rich message parsing on all platforms.

The basic idea is that core can use this library to convert messages to an AST format,
that can then be displayed by the UIs how they see fit, for desktop it will be converted to react elements.

> Desktop already uses this package (minus the markdown, because it does not make sense to only have markdown only on one platform) as wasm module (see `./message_parser_wasm`), later this will probably be integrated into deltachat core.

## Coding Principles

- **many** test cases
- aim to be fast - so also benchmarks to make sure the lib stays fast enough

## Recomendations:

If used for message parsing, don't parse messages that are over `10 000` chars in size to ensure performance stays excelent. (the lib could and should support more than that and should aim to be fast enough for it, but on slow devices or transpiled to wasm or asmjs limiting it makes sense to avoid laggy/freezed interface)

### Benchmarking:

```
cargo install cargo-criterion
```

benchmark:

```
cargo criterion
```

docs about benchmarking: https://bheisler.github.io/criterion.rs/book/criterion_rs.html

## References

- Older discussion on introducing markdown into deltachat: https://github.com/deltachat/interface/pull/20
- Feature request for message markdown in the forum: https://support.delta.chat/t/markdown-support-in-chat/159
