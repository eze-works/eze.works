[
    title: "Reflections on writing parser combinator libraries",
    date: ~D(2023-12-13),
    labels: ["parsing", "rust"],
]
+++
I have [this article](https://www.theorangeduck.com/page/you-could-have-invented-parser-combinators) to thank for dragging me down the parser combinator rabbit hole. The result has been [three](https://crates.io/crates/ruminant) [separate](https://crates.io/crates/parser-compose) [attempts](https://crates.io/crates/bparse) at writing a parsing  library, so here are a some thoughts.


_Warning: Unsubstantiated claims ahead._


__You might be better off without error handling__

A lot of parsing crates out there tout their ability to let you configure how errors are returned from a parse call. This always [mucks up the return type](https://github.com/rust-bakery/nom/blob/main/doc/error_management.md), forces you to have a bunch of error-related combinators, and makes the code harder to read. Dispense with error handling from the library, and let users handle errors themselves.

__You don't need to be generic over input__

Wouldn't it be nice if your library could accept any type of "input stream" and still work? Yes...but [no](https://docs.rs/chumsky/latest/chumsky/stream/index.html). It isn't worth the pain. The input is almost always bytes. Discard the "abstract stream" concept and force your users to give you bytes.

__You don't need to be generic over output__

One of the draws of parser combinators is that they let you write [declarative parsers in an imperative language](https://gitlab.com/wake-sleeper/parser-compose/-/blob/dd51e3dcd4f090163cbebf53999deea770926440/tests/json.rs#L204). After all the setup and ceremony is done, it feels a little bit like magic. But the magic comes at the cost of debuggability. `console.log`-style debugging is out the window because the execution looks like a tree, with each node have 0 context about how it got there. Even stepping through with a debugger is pain. One way out of this mess is to always output bytes from your parsers. This forces the library users to parse things one token at a time, in an imperative manner.

Taken together, these ideas lead to designing a library whose only focus is "identifying patterns in bytes". Bytes in, bytes out. Simple. Understandable. Still useful.

> Perfection is achieved, not when there is nothing more to add, but when there is nothing left to take away.
