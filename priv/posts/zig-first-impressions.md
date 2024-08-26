+++
[
    title: "Zig: First impressions",
    date: ~D(2023-10-18),
    labels: ["zig"]
]
+++
From [Rust is hard](https://hirrolot.github.io/post/rust-is-hard-or-the-misery-of-mainstream-programming.html#): 

> When you use Rust, it is sometimes outright preposterous how much knowledge of language, and how much of programming ingenuity and curiosity you need in order to accomplish the most trivial things. When you feel particularly desperate, you go to rust/issues and search for a solution for your problem. Suddenly, you find an issue with an explanation that it is theoretically impossible to design your API in this way, owing to some subtle language bug. The issue is Open and dated Apr 5, 2017.


This deeply resonated to the point I shed a tear or two. The language actively prevents me from designing  APIs the way I want. I ran into this constantly when I was working on my [parser combinator crate](https://crates.io/crates/parser-compose).

From the [Zig website](https://ziglang.org/):

> Focus on debugging your application rather than debugging your programming language knowledge.

Needless to say, I have been enjoying Zig very much.

__It's easy to pick it up:__

In the past week I have written just shy of  [~1k lines of Zig](https://gitlab.com/wake-sleeper/gambit). Compare this to ~5k lines of Rust in the past 3~4 months. Lines of code is a dubious metric, but I mention it because it illustrates how much easier it is to get your thoughts into code in Zig.

__It leads to a more concrete understanding of your program:__

I don't have a systems programming background, so it wasn't until I started dabbling with Rust that the concepts of heap vs stack memory started being relevant. But even then, Rust hides the dirty work of handling memory allocations from you. Zig pulls the curtain away by forcing you to think about [where your bytes are](https://ziglang.org/documentation/0.11.0/#Where-are-the-bytes).
If I am working on a side-project, I think I'd rather have "segmentation fault" runtime errors than "unconstrained lifetime parameter" compile-time  errors.
The latter is a made up rule enforced by the Rust compiler. Understanding why it happens and fixing it leads to a tiny boost in understanding the Rust type system. The former is a fundamental operating system rule that i violated. Understanding why it happens and fixing it leads to a better understanding of program memory in general, regardless of  programming language.


__comptime:__

I value good API design.  Inferring _stuff_ at compile time leads to APIs that are easier to use. For example, [clap](https://crates.io/crates/clap) lets you define your command line interface by declaring a struct and adding attribute macros. This is great from the user's point of view, but the the barrier to entry on the library writer's side is high; you need to walk the dark alleys of of Rust's proc-macro system. Zig sidesteps this by making meta-programming part of the language with the `comptime` keyword. The best way I have to describe it is that it allows you to "program your own type system".  I'll leave you with a taste of Zig code to whet your appetite.

The following is a function that takes in a type as argument and returns the number of fields it has. It fails _at compile time_ if the type is not struct-like

```zig
// Count the number of fields in `T`
fn countFields(comptime T: type) usize {
    // If `T` is not a struct emit a compile error
    if (@typeInfo(T) != .Struct) {
        @compileError("Type " ++ @typeName(T) ++ " is not a struct");
    }

    // Loop through the struct fields, incrementing the counter
    comptime var count = 0;
    inline for (@typeInfo(T).Struct.fields) |f| {
        _ = f;
        count += 1;
    }

    return count;
}
```

You would use it like this:
```zig
// Define a type called OneField. Instances of this type would 
// have a field called `one` with an 8-bit value.
const OneField = struct { one: u8 };
const ThreeFields = struct { one: u8, two: u16, three: u32 };

test "countFields" {
    try testing.expect(countFields(OneField) == 1);
    try testing.expect(countFields(ThreeFields) == 3);
    // this fails to compile
    // try testing.expect(countFields(u8) == 3);
}
```
