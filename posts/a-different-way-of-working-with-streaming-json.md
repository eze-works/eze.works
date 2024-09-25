{
    "title": "A different way of working with streaming JSON",
    "date": "2024-02-28",
    "labels": ["json", "rust"]
}
+++
I have implemented [a streaming JSON parser](https://crates.io/crates/jsn), and with it, a curious new approach to working with streaming JSON.

[Almost](https://www.baeldung.com/jackson-streaming-api#bd-parsing-json) all [existing](https://rapidjson.org/md_doc_sax.html#Reader) streaming [json parser](https://github.com/dscape/clarinet?tab=readme-ov-file#basics) implementations yield JSON "tokens".
<details>
    <summary>Oboejs</summary>
    <p>
    <a href="https://web.archive.org/web/20210814153523/http://oboejs.com/">Oboejs</a> is a notable exception.
    The website is down, but you can still peruse the API <a href="https://github.com/jimhigson/oboe.js-website/blob/master/pdf/examples.pdf">on the github page</a>.
    </p>
</details>

Given the following JSON:

```json
{ "hello": "world" }
```

The following tokens will be yielded one after the other: `{`, `"hello"`, "`:`", `"world"`, `}`.

That's the API. I find this sad because it's basically leaving the work of doing something useful with the JSON as "an exercise for the reader".

I get why this is the API. The point of streaming JSON is that you don't want to load the whole thing into memory. But wouldn't it be nice if we could still "query" streaming JSON tokens somehow? That is where I come in.

Here is the pitch:

If you can use a bitwise AND to extract a pattern from bits:

```
input   : 0101 0101
AND
bitmask : 0000 1111
=
pattern : 0000 0101
```

Why can't you do the same and extract a pattern from JSON tokens?

```
input     : { "hello": { "name" : "world" } }
AND
json mask : {something that extracts a "hello" key}
=
pattern   : _ ________ { "name" : "world" } _
```

Well, [`jsn`](https://crates.io/crates/jsn) allows you to do just that. It's a rust crate I have been working on for the past few weeks. It lets you apply a combination of selectors/masks to a stream of JSON tokens without loading everything into memory. It allows you to write code like this: 

```rust
use jsn::{Tokens, select::*};

// This example uses an in-memory string, but this could be a file
// or TcpStream.
let json = r#"{ "hello": [1,2], "bye": [3,4] }"#;

let mut iter = Tokens::new(json.as_bytes())
    .with_selector(key("hello") & index(0));

assert_eq!(iter.next().unwrap(), 1);
assert_eq!(iter.next(), None);
```

I have added a CLI example to the repository to help demonstrate. I'll leave you with a few examples to whet your appetite: 

```sh
echo '{ "key": { "nested": [ 1, 2 ] } }' > test.json

cargo run --example cli -- --file test.json --key "key"
# { "nested" : [ 1 , 2 ] }

cargo run --example cli -- --file test.json --depth 3
# [ 1 , 2 ]

cargo run --example cli -- --file test.json --keys
# "key" "nested"

cargo run --example cli -- --file test.json --values
# 1 2

cargo run --example cli -- --file test.json --values --index 0
# 1 
```

---
_Comments on [/r/rust](https://www.reddit.com/r/rust/comments/1b2d41c/jsn_a_different_way_of_working_with_streaming_json/)_

