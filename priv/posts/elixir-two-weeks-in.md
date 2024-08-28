+++
[
    title: "Elixir, two weeks in",
    date: ~D(2024-08-27),
    labels: ["elixir"],
    stage: :draft
]
+++

The [first thing I wrote](/post/rust-stockholm-syndrome) about on this site compared writing Rust to stockholm syndrome.

I still stand by this, but I think my days of captivity are over.

Over the past two weeks, I have been learning elixir.

This is the most productive I've been with a language in a long, long time.

So far, I hve:

- built a toy command line parsing library that uses module definitions to represent argument blueprints.

- published my [first hex.pm library](https://hex.pm/packages/pile) for templating HTML in Elixir.

- Used that library to redo my website. 


In my first post I mentioned that Rust had an un-nameable property (or properties!) that made it appealing.

I now think a big part of the appeal is tooling.

When evaluating if a programming language is for me, apart from the syntax, I consider the tooling around managing dependencies, formatting code, running tests, publishing libraries and creating deployable artifacts.

Writing code involves all these things, and the less friction there is around them, the better the experience.

Elixir, like Rust and many other modern languages handle these concerns in a straightforward and cohesive way.

Elixir goes a bit further and addresses the main problem I had with Rust; it makes designing pleasant APIs easy.

I like writing small libraries for fun. They are mostly targetted for my personal use. Some live up to that, others get abandoned. But I've written [a dozen](https://crates.io/users/eze-works).

The language lacks levers library writers can use to make interacting with an API pleaseant. This post isn't about that, but I'll rattle off a few of my pet peeves:

- `std::ops::Range` and friends aren't `Copy`. This was a pain when designing a "range" combinator in a parser combinator library.
- Functions can't have multiple arguments. This was a pain when I was writing a HTML generation library that tried to avoid macros. 
- The built-in `Iterator` can't mutably borrow the thing being iterated over. And even when I decide to eschew `Iterator` and just have a `next()` method that returns a `&mut self`, this invariably leads to borrow checker issues that will only be fixed with the elusive next-gen borrow-checker, polonius.
  - This prevented me from writing libraries where I wanted to have the typesystem enforce "the thing returned by `next()` is only available until the next call to `next()` (e.g. this fits really nicely with how entires in tar files are structured on disk).

Writing libraries in Rust feels like death by a thousand cuts, and I was tired of it.

Elixir in contrast has been a breath of fresh air.

This now in: "Garbage collected, dynamic language found to be more flexible than a non GC-ed, statically typed, systems language"

Perhaps the lesson here is not to use a systems language for non-systemsy things, no matter how nice the tooling is.

It goes beyond syntax flexibility though, it's also what the runtime/standard library exposes.

Elixir exposes code for [compiling](https://hexdocs.pm/elixir/1.17.2/Code.html#compile_file/2) _and_ [evaluating](https://hexdocs.pm/elixir/1.17.2/Code.html#eval_file/2) Elixir code.

This is what the build tool, mix uses under the hood to [compile its own configuration file](https://github.com/elixir-lang/elixir/blob/74bfab8ee271e53d24cb0012b5db1e2a931e0470/lib/mix/lib/mix/cli.ex#L42) (i.e. `mix.exs`) 

The language also lets you run basically whatever code you wish at build/compilie time.

It exposes compile hooks so you can do stuff after a module has finished compiling. 

Don't get me started about how straightforward macros are.

I understand Elixir macros as functions that output the Elixir AST.

This is how to get the AST of the expression `a + b * c` and store it in a value:

```elixir
ast = quote do
    a + b * c
end
```

Coming from the dark Rust world of `macro_use!`... I almost cried learning this.

It feels like as if my only source of light had been fires I had to manually start myself with flint stones.Meanwhile my neighbour had electricity. 

I'm looking forward to the new projects this tool inspires
