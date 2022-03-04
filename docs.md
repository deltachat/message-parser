### How parsing currently roughly works

We use the nom parser, which basically consists of functions that check the input and when it matches they consume the part of the input that matched.

So currently, the parsing works like this:

pseudocode:

```rs
let mut remaining_input = input;
let output = Vec<Element>

while !remaining_input.is_empty() {
    let res = {
        // try the following parsers in this order (order is important with some parsers)
        1. hashtag(input)
        2. email_address(input)
        3. link(input)
        4. bot_command_suggestion(input)
        5. linebreak(input)
        last option: consumes all text until [parse_text_element] works again
    }
    remaining_input = res.remaining_input
    output.push(res.element)
}
```

### Contributing principles:

The single most important thing for this crate is testing, as long as we cover as many cases in tests as we can the parser stays working.

The second principle is speed, we can test that with benchmarks.

The third priority is binary size, so be careful with huge extra libraries, maybe there is a better way.
