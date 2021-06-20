# [Draft] DeltaChat Message Parser

Parsing of Links, Email adresses, simple text formatting (markdown subset), user mentions, hashtags and more in DeltaChat messages.

The specification can be found in [spec.md](./spec.md).

## Idea behind it

Have the same rich message parsing on all platforms.

The basic idea is that core can use this library to convert messages to an AST format,
that can then be displayed by the UIs how they see fit, for desktop it will be converted to react elements.

## Coding Principles

- **many** test cases
- aim to be fast - so also benchmarks to make sure the lib stays fast enough

## Recomendations:

If used for message parsing, don't parse messages that are over `10 000` chars in size to ensure performance stays excelent. (the lib could and should support more than that and should aim to be fast enough for it, but on slow devices or transpiled to wasm or asmjs limiting it makes sense to avoid laggy/freezed interface)
