# DeltaChat Message Parser WASM

Parsing of Links, email addresses, simple text formatting (markdown subset), user mentions, hashtags and more in DeltaChat messages.

The specification can be found in [spec.md](https://github.com/deltachat/message-parser/blob/main/message_parser_wasm/spec.md).

The parser is written in rust with the [nom crate](https://github.com/Geal/nom) and compiled to web assembly for this package.

## The Idea behind it

Have the same rich message parsing on all platforms.

The basic idea is that core can use this library to convert messages to an AST format,
that can then be displayed by the UIs how they see fit, for desktop it will be converted to react elements.

> Desktop already uses this package (minus the markdown, because it does not make sense to only have markdown on desktop and not also on iOS and android) as wasm module (see `./message_parser_wasm`), later this will probably be integrated into deltachat core.

Read more about the project on github: https://github.com/deltachat/message-parser

## 🚴 Usage

```ts
function parse_text(s: string, enable_markdown: boolean): ParsedElement[];
```

```js
import init, { parse_text } from "./pkg/message_parser_wasm.js";

init().then(() => {
    let parsed = parse_text("hello **world**", true)

    let result = parsed.map(element => {
        switch element.t {
            case "Bold":
                return `<b>${element.c}</b>`
                break;
            case "Text"
                return element.c
            // ...
            default
                console.error(`type ${element.t} not known/implemented yet`, element);
                return JSON.stringify(element)
        }
    }).join("")

    console.log(result) // "hello <b>world</b>"
})
```

> DO **NOT** actually write html with user input like that, this is for demonstration purposes ONLY!
> It let's you and your users open to **XSS attacks**, the examples bellow are much better suitable for reference or copy+pasting.

also see [example.js](./example.js) and test it live on <https://deltachat.github.io/message-parser/>

For usage in react you can look at how we integrated this package in deltachat-desktop: [deltachat-desktop/src/renderer/components/message/MessageMarkdown.tsx](https://github.com/deltachat/deltachat-desktop/blob/7493f898bc3dff06b20565a48e93564f5996b855/src/renderer/components/message/MessageMarkdown.tsx)

If you want to see it in action in deltachat-desktop, feel free to download it on <https://get.delta.chat>.


### Emoji Helper functions

```js
/** returns first emoji from text if text begins with an emoji */
export function get_first_emoji(input: string): string | undefined;
/** If string contains only emojis count the emojis otherwise retuns null */
export function count_emojis_if_only_contains_emoji(input: string): number | undefined;
```

### For Devs

#### 🛠️ Build with `wasm-pack build`

```
wasm-pack build --scope deltachat --target web
```

#### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

#### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish --target web
```
