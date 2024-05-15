# DeltaChat Message Parser

Parsing of Links, Email adresses, simple text formatting (markdown subset), user mentions, hashtags and more in DeltaChat messages.

The specification can be found in [spec.md](./spec.md).

WASM Demo: <https://deltachat.github.io/message-parser/>

## The idea behind it

Have the same rich message parsing on all platforms.

The basic idea is that core can use this library to convert messages to an AST format,
that can then be displayed by the UIs how they see fit, for desktop it will be converted to react elements.

> Desktop already uses this package (minus the markdown, because it does not make sense to only have markdown only on one platform) as wasm module (see `./message_parser_wasm`), later this will probably be integrated into deltachat core.

## Coding Principles

- **many** test cases
- aim to be fast - so also benchmarks to make sure the lib stays fast enough

## Recomendations

If used for message parsing, don't parse messages that are over `10 000` chars in size to ensure performance stays excellent. (the lib could and should support more than that and should aim to be fast enough for it, but on slow devices or transpiled to wasm or asmjs limiting it makes sense to avoid laggy/freezed interface)

### Benchmarking:

```
cargo install cargo-criterion
```

benchmark:

```
cargo criterion
```

docs about benchmarking: https://bheisler.github.io/criterion.rs/book/criterion_rs.html

### Changing CPU power settings for consistent results

These days most CPUs change their performance according to some rules to save power.
To produce consistent benchmark results, CPU performance must not change between benchmarks. There are various ways to achieve this.
If you've got a laptop, the first step might be connecting the AC adapter to ensure your laptop won't go on power saving mode and thus changing the 
CPU frequency. The next step is to change CPU frequency to a constant value *under the maximum frequency CPU can handle*. Because the CPUs usually can't handle the maximum possible frequency on all cores.

On Linux, you can set the CPU frequency using `cpupower` utility:

```sh
cpupower frequency-set --min 3500 --max 3500 # this to set maximum and minimum to the same value
cpupower frequency-set -f 3500 # set frequency explicitly if the kernel module is available
```

## References

- Older discussion on introducing markdown into deltachat: https://github.com/deltachat/interface/pull/20
- Feature request for message markdown in the forum: https://support.delta.chat/t/markdown-support-in-chat/159


## Emoji Helpers

Additionally to message parsing this crate also contains some useful functions for working with emojis.

-  `parser::is_emoji::emoji` (rust only) - nom parser that eats one emoji
    - idea: could potentially be used by core to filter reactions to only emojis
- `parser::is_emoji::get_first_emoji(text)` - get first emoji if text begins with an emoji
    - idea: can be used by UI to get the first emoji of a chat name to display it as text avatar
- `parser::is_emoji::count_emojis_if_only_contains_emoji(text)` - counts emojis in texts that contain only emojis
    - useful for jumbomoji logic (if you send a small message with just emojis the emojis get displayed larger).
    - this function does not fail on too long strings, so to keep good performance check the length beforehand and if it is too long the message would not be big anyway so you don't need to call this function.
