{
    "title": "HTTP Headers are dark and full of terrors",
    "date": "2023-12-20",
    "labels": ["http"]
}
+++
As is expected from a naive software person fully on-board the Not Invented Here train, I am attempting to implement the [http/1.1](https://httpwg.org/specs/rfc9112.html) spec so I can finally get to writing web applications and services (naturally, i had to write [my own parser first](https://mistlenote.com/reflections-on-writing-3-parser-combinator-libraries/)). So far, parsing HTTP headers has been the most troublesome part of the process.

On the surface, headers look like a dictionary of `(header name, header value)` pairs:

```
My-Header: some value
Some-Other-Header: value
```

But the the spec [allows for repeated header names](https://mistlenote.com/reflections-on-writing-3-parser-combinator-libraries/), so it's more of `(header name, list of header values)` pairs:

Additionally, depending on the header, its values can be collapsed into one. So this:

```
My-Header: value1
My-Header: value2,
```

could be transformed to this:

```
My-Header: value1, value2
```

Because of reasons, this does not apply to the `Set-Cookie` header.

Additionally, there is a special ["quoted-string"](https://httpwg.org/specs/rfc9110.html#quoted.strings) construct that allows a header value to be quoted. Within quotes, the comma does not separate two values. More annoyingly, when inside a quoted-string, you can backslash-escape certain values (including the double quote and comma):

```
// This
My-Header: "hello, world", "bye \"now\""
// is the same as this
My-Header: "hello, world"
My-Header: "bye \"now\""
```

Oh, and you can sometimes have [comments](https://httpwg.org/specs/rfc9110.html#comments) Yea, that stuff within parenthesis in a `User-Agent`: 

```
User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/...
```

I don't even have the patience to figure out [parameters](https://httpwg.org/specs/rfc9110.html#parameter) and how they differ from [auth-parameters](https://httpwg.org/specs/rfc9110.html#auth.params).

To top it off, some headers don't adhere to any of the above rules:

```
Date: Tue, 15 Nov 1994 08:12:31 GMT
```

The `Date` header for example has commas, but is not double quoted. A generic parser would probably treat that as:

```
Date: Tue
Date: 15 Nov 1994 08:12:31 GMT
```

It's pretty gnarly. For an HTTP client/server library the only generic transformation you can do to headers is to join their values together with a comma if the header appears multiple times...unless it's `Set-Cookie`. Everything else has to be special-cased to the header name.

This feels a bit discouraging. I wanted my library to expose headers as structured objects, but it is looking like that is way more work than i bargained for. For now, I have opted to only inspect/parse the headers necessary for the library to validate requests and responses. All the other headers are exposed raw. So if I get these headers...

```
Transfer-Header: gzip, magic
Transfer-Header: chunked
```

... consumers of the library that lookup the `Transfer-Header` would get two values: "gzip, magic" and "chunked".

I think what gets me the most is how simple headers look at first. How did we go from a straightforward key/value map to this mess? 
