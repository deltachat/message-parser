# Message formatting / parsing of interactive elements and Specification of the format

> This list is for every formating thing that goes beyond plain-text.

## Modes of the parser

- Text only
  - [Email addresses: `hello@delta.chat`](#email-addresses)
  - [Mentions `@hello@delta.chat`](#mentions)
  - [Links: `https://delta.chat` and `mailto:hello@delta.chat`](#links)
  - [Bot `/commands`](#bot-commands)
  - [Hashtags: `#tag`](#hashtag)
- Desktop (made for what desktop now supports, so we can use the wasm version of the message parser as drop-in replacement)
  - All from Text (see above)
  - [Delimited Email addresses: `<hello@delta.chat>`](#delimited-email-addresses)
  - [Delimited Links: `<http://example.org>`](#delimited-links)
  - [Labeled Links: `[Name](url)`](#labled-links)
- (everything) Markdown
  - [_italics_: `*italics*`](#italics)
  - [**bold**: `**bold**`](#bold)
  - [~~strikethrough~~: `~~strikethrough~~`](#strikethrough)
  - [`inline-code`: `` `inline-code` ``](#inline-code)
  - [Code Block: ` ``` fence code block ``` `](#code-block)
  - [Delimited Email addresses: `<hello@delta.chat>`](#delimited-email-addresses)
  - [Delimited Links: `<http://example.org>`](#delimited-links)
  - [Labeled Links: `[Name](url)`](#labled-links)

## Text Enhancements

Text elements that are displayed as is with no change to the content, just enhanced (made clickable) if necessary.

<a name="email-addresses" id="email-addresses"></a>

### `hello@delta.chat` - Email addresses

Make email addresses clickable, opens the chat with that contact and creates it if it does not already exist.

#### Format

- format should follow standards (as time of writing the current implementation is still fairly dumb)
- trailing `.` is not part of the email address and should not be parsed with it.

<a name="mentions" id="mentions"></a>

### Mentions `@hello@delta.chat`

Clickable mentions, opens profile view for a contact.
UI may replace email address by display name
and highlight it to distinguish it from normal text.
(like other messengers do it, look at telegram, discord, element and so on)

#### Format

A valid email address preceded by an `@` character.
Reuses parsing logic from [email address](#email-addresses).

#### Other uses

There will be a dedicated api that just extracts mentions from a text that will be used by deltachat core to be able to notify users when they are mentioned.

#### In Message Composer

the message input field should provide autocomletions as user types @Displayname or @user@email.address

<a name="links" id="links"></a>

### `https://delta.chat` and `mailto:example@example.com` - Links

Make URLs clickable.

- detect all valid hyperlink URLs that have the `://` (protocol://host).

- other links like `mailto:` (note there is just a single `:`, no `://`) will get separate parsing that includes a whitelisted protocol name, otherwise there will likely be unexpected behavior if user types `hello:world` - will be recognized as link.

- `.`,`,`,`;`,`:` should not be parsed as an ending char of an inline-link(this rule is only for standalone/inline links)

#### Linkified schemes:

- all Common Internet Scheme links (containing `//` after scheme),
- `mailto:`, `news:`, `feed:`
- `tel:`, `sms:`, `geo:`, `maps:`
- `bitcoin:`, `bitcoincash:`, `eth:`, `ethereum:`
- `magnet:`

##### `mailto:email@address.example.com`

Make mailto links clickable with all parameters: `?subject=Sample%20Subject&body=Sample%20Body`
Should open in delta chat directly.

##### Custom Deltachat URI Scheme

see https://support.delta.chat/t/custom-deltachat-url-scheme/346
Should open in deltachat directly.

<a name="bot-commands" id="bot-commands"></a>

### Bot `/commands`

On click, the command gets prefilled as the draft, so it can be easily send.
Also if the draft is not empty it should ask before replacing it.

```regex
/(?<=^|\\s)/[a-zA-Z][a-zA-Z@\\d_/.-]{0,254}/
```

<a name="hashtag" id="hashtag"></a>

### `#tag`

`/#[\w]+/i`

> later we want something like `/#[^ \n\r\t#]+/` (`#` then everything (besides `#`) until space/line break/tab) to also allow for chars from other locales and emojis, see https://github.com/deltachat/message-parser/issues/8 for more info

Basically a clickable search shortcut. On click, it opens the message search prefilled with that tag.

Inspired by twitters and telegrams #hashtag functionality.

### other / internal

- Text (what remains if nothing else could be detected)
- line breaks

## Markdown subset

The subset of Markdown that Deltachat is going to support, this contains everything that needs to be displayed differently, not only made clickable.

<a name="italics" id="italics"></a>

### `*italics*` and `_italics_`

No whitespace as first nor as end char:
correct:

```
*italics* test
*italics test*
```

wrong:

```
* italics* test
```

<a name="bold" id="bold"></a>

### `**bold**` and `__bold__`

No whitespace as first nor as end char: see italics examples.

<a name="strikethrough" id="strikethrough"></a>

### `~~strikethrough~~`

No whitespace as first nor as end char: see italics examples.

<a name="inline-code" id="inline-code"></a>

### `` `inline-code` ``

Useful to send non Markdown text in your message, like source code snippets.
Should get rendered in a monospace font and with a different background.
In contrast to bold, italics and strike through the content of inline-code can contain spaces at beginning and ending.

<a name="code-block" id="code-block"></a>

### ` ``` fence code block ``` `

```
Similar to `inline-code` but not inline, and it may support code highlighting.
```

` ```[lang?] [content]``` `
A bit modified from the common syntax to allow one-liners.
Also get displayed with a monospace font (a side effect of this is that it allows to display small ASCII art).
The code **highlighting** is **optional** as implementation (time)cost
may not be worth the small gain.
The `language` definition should be parsed separately and omitted in this case.

If no language is set in the single line variant, the content must begin with a space:
WRONG: ` ```hello world``` ` (because hello will be taken as language)
RIGHT: ` ``` hello world``` `

see https://github.com/adam-p/markdown-here/wiki/Markdown-Cheatsheet#code-and-syntax-highlighting

Parsing will remove trailing whitespaces and linebreaks at the end of the codeblock content.

<a name="delimited-email-addresses" id="delimited-email-addresses"></a>

### `<hello@delta.chat>` - Delimited Email addresses

same format as normal email addresses enclosed in `<>`.

<a name="delimited-links" id="delimited-links"></a>

### `<http://example.org>` - Delimited Link

same format as normal Links enclosed in `<>`.

URL parsing allows all valid URLs, no restrictions on schemes, no whitelist is needed, because the format already specifies that it is a link.

<a name="labled-links" id="labled-links"></a>

### Labeled Links: `[Name](url)` links

When implementing this, make sure to show the user the hidden URL in a confirmation dialog to make scamming harder.
Also show the URL as encode puny code to make puny code attacks useless.
Optionally, a client can implement a system to trust a domain (a "don't ask again for links on this domain" checkbox in the confirmation dialog)

URL parsing allows all valid URLs, no restrictions on schemes, no whitelist is needed, because the format already specifies that it is a link.

## Ideas For The Future:

### `:emoji:`

- ':' + [A-z0-9_-] + ':' ?
- could also be used for custom DC emojis in the future
- In Message Composer: should provide auto completions that replace the text by the unicode emoji, or the complete code if it is a custom emoji.

### `$[inline TeX]$` and `$$[Tex displayed in block(new line)]$$`

For sharing math/physics equations in LaTeX format.
see https://support.delta.chat/t/latex-code-in-deltachat/558

## Things that will NOT be supported:

- Inline HTML
- underline - can be confused with links
