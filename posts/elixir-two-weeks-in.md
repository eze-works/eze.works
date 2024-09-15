{
    "title": "Elixir, two weeks in",
    "date": "2024-08-27",
    "labels": ["elixir"]
}
+++

The [first thing I wrote](/post/rust-stockholm-syndrome) about on this site compared writing Rust to suffering of Stockholm Syndrome.
I still stand by that statement, but I think my days of captivity are over: Over the past two weeks, I have been learning Elixir.

This is the most productive I've been with a language in a long, long time. So far, I have:
- built a toy command line parsing library that uses module definitions to represent argument blueprints.
- published my [first hex.pm library](https://hex.pm/packages/pile) for templating HTML in Elixir.
- Used that library to redo my website.


To evaluate how much I'll enjoy a programming language, I consider the tooling around managing dependencies, formatting code, running tests, publishing libraries and creating deployable artifacts.
Writing code involves all these things, and the less friction there is around them, the better the experience.
Elixir, like Rust and many modern languages, handles these concerns in a straightforward and cohesive way.
Elixir goes even further and addresses the main problem I had with Rust by easing the process of designing pleasant library APIs.

I like writing small libraries for fun.
They are for my personal use.
Some live up to that, others get abandoned, but I've written [a dozen](https://crates.io/users/eze-works) in Rust.
The language lacks some levers library authors need to make interacting with their API pleasant.
I'll list a few of my pet peeves. It is a tangent, but it feels good getting it off my chest:

- `std::ops::Range` and friends aren't `Copy`. This was a pain when designing a "range" combinator in a parser combinator library.
- No variadic functions (i.e. functions that can take a variable number of arguments). This was a pain when I was writing a HTML generation library that tried to avoid macros.
- The built-in `Iterator` can't mutably borrow the thing being iterated over. And even when I decide to eschew `Iterator` and just have a `next()` method that returns a `&mut self`, this invariably leads to borrow checker issues that will only be fixed with the elusive next-gen borrow-checker, polonius.
  - This prevented me from writing libraries where I wanted to have the type system enforce "the thing returned by `next()` is only available until the next call to `next()` (e.g. this fits really nicely with how entires in tar files are structured on disk).

Writing libraries in Rust feels like death by a thousand cuts, and I am tired of it. Elixir in contrast has been a breath of fresh air.

_This now in: "Garbage collected, dynamic language found to be more flexible than a non GC-ed, statically typed, systems language"_

Perhaps the lesson here is not to use a systems language for non-systemsy things, no matter how nice the tooling is.
But, I think it goes beyond syntax flexibility, it's also about what the runtime/stdlib exposes.

Elixir exposes code for [compiling](https://hexdocs.pm/elixir/1.17.2/Code.html#compile_file/2) _and_ [evaluating](https://hexdocs.pm/elixir/1.17.2/Code.html#eval_file/2) Elixir code.
This is what the build tool, mix uses under the hood to [compile its own configuration file](https://github.com/elixir-lang/elixir/blob/74bfab8ee271e53d24cb0012b5db1e2a931e0470/lib/mix/lib/mix/cli.ex#L42) (i.e. `mix.exs`)
The language also lets you run basically whatever code you wish at build/compile time.
It exposes compile hooks so you can do stuff after a module has finished compiling.
Don't get me started about how straightforward macros are.
Elixir macros are functions that output the Elixir AST.
This is how to get the AST of the expression `a + b * c` and store it in a value:

```elixir
ast = quote do
    a + b * c
end
```

Coming from the dark Rust world of `macro_use!`... I almost cried learning this.
It is as if my only source of light had been fires I had to manually start with flint stones, meanwhile my neighbour had electricity.

Extensibility is a core part of Elixir, and a feature I've sorely missed in programming.
It makes the language a joy to use, and I'm looking forward to the new projects it inspires.
