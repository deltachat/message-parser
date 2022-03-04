## Inline links with `,` at end #13:

`.`,`,`,`;`,`:` and co should not be parsed as an ending char of an inline-link



## Parentheses #12:

`()`, `[]`, `{}`, `<>` should not be parsed as part of an inline structure, but can appear in parsed text of other structures (in a lable in an labled link for example).

```
https://delta.chat/page(this is the link to our site)
[Link(https://delta.chat/page), Text("(this is the link to our site)")]
```


idea: as normal delimited parser with highest priority?

or we need to count because delimited won't work with `(see also https://delta.chat/en/help(and)+FAQ)"`

//TODO ask for the EXACT test cases and what they should result in 

OR maybe count inside of the link when we have it and then parse it again only until valid `(`+`)` combo,
like:
- link can only close as many parentheses as it opened before and hasn't closed already,
- `(` will become forbidden as last link char

## what about labled links that have a destination with parentheses:
we do not care I'd say - if you hide the destination under a label then you can also url escape your link properly

# markdown only: allow escaping * and other chars with \ #9

can be achieved with a new parser before text: if \ is found parse it and the next char after it

new info for gh issue:
- (discussion tag)
- (markdown only)
- should have a whitelist of char it does this with, so that emotes like `\o/` still work without double escape (`\\`) 