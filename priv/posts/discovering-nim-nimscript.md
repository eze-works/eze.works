+++
[
    title: "Discovering Nim: NimScript",
    date: ~D(2024-05-13),
    labels: ["nim-lang"]
]
+++
It feels like the [Nim](https://nim-lang.org/) is a hidden gem of a programming language. And within this hidden gem lies another hidden gem, [`NimScript`](https://nim-lang.org/docs/nims.html). I figured I'd write about it in order to understand it better.

NimScript is a subset of the Nim language that you write in files that have the `.nims` extension. But how is it different from a regular `.nim` file? Let's find out.

Create a file called `hello.nim` with the following content:

```nim
echo "HELLO FROM NIM"
```

Run it: `nim compile --hints:off --run hello.nim` and you get the output you expect.
```text
HELLO FROM NIM
```

Now change the file name to `hello.nims` and run it: `mv hello.nim hello.nims && nim compile --hints:off --run hello.nims`.  
You'll get the following
```text
HELLO FROM NIM
HELLO FROM NIM
```

Wait a minute...why does it print twice?

Running it again without turning off hints will provide a clue: `nim compile --run hello.nims`

```text
HELLO FROM NIM
Hint: used config file '/home/eze/.choosenim/toolchains/nim-2.0.4/config/nim.cfg' [Conf]
Hint: used config file '/home/eze/.choosenim/toolchains/nim-2.0.4/config/config.nims' [Conf]
Hint: used config file '/home/eze/crafts/learnnim/hello.nims' [Conf]
......................................................................
Hint:  [Link]
Hint: mm: orc; threads: on; opt: none (DEBUG BUILD, `-d:release` generates faster code)
37312 lines; 0.172s; 37.73MiB peakmem; proj: /home/eze/crafts/learnnim/hello.nims; out: /home/eze/crafts/learnnim/hello [SuccessX]
Hint: /home/eze/crafts/learnnim/hello [Exec]
HELLO FROM NIM
```

Well that certainly is interesting. The NimScript file is evaluated twice. In addition to being a regular file that can be compiled and ran as usual, it also appears that it is evaluated at the very start of the `nim` cli. 

Alone, this quirk isn't useful. In fact, you probably shouldn't be directly executing NimScript this way. Instead, you would use `nim e hello.nims`:

```
HELLO FROM NIM
```

Much better. 

NimScript can be thought of as the configuration language for `nim`'s cli. It is evaluated before `nim` does anything of consequence. In fact, when `nim` is invoked, it looks for specific NimScript files in specific directories and evaluates them first. If you look back to some of the previous output from invoking `nim`, you'll see lines like: `Hint: used config file '...'`. That's nim evaluating NimScript at those locations. The cool part is that the evaluated NimScript can affect what the `nim` cli ends up doing.

One location in which Nim looks for NimScript is at the root of your project's directory, in a file named `config.nims`. I am not currently in a project directory, but the nim code has a [fallback for when it isn't in a project context](https://github.com/nim-lang/Nim/blob/04f3df4c87e8ba9efc26fa4faed8e3b6cbaa6e93/compiler/nimconf.nim#L278); it just looks in the current directory.

Rename our previous file to `hello.nim`: `mv hello.nims hello.nim`.  
Then in a new file called `config.nims` write the following:
```nim
task run, "Run the hello.nim file": 
  echo "Compiling and running hello.nim"
  setCommand("compile", "hello.nim")
  switch("run", "")
  switch("hints", "off")

task cleanrun, "Remove executables then run":
  echo "Deleting compiled executable hello..."
  rmFile("hello")
  runTask()
```

Now run: `nim cleanrun`:

```
Deleting compiled executable hello...
Compiling and running hello.nim
HELLO FROM NIM
```

Wow! Now we are cooking with gas. In NimScript files, the [`system/nimscript`](https://nim-lang.org/docs/nimscript.html) module is automatically in scope. Among other nifty things in there, there is a Nim template called `task` that lets you define custom nim commands. You can see the list of defined tasks by running `nim help`.

I think having a builtin alternative to a `Makefile` is pretty cool. Interestingly, the configuration for Nim's package manager `nimble` is [actually a NimScript file](https://nim-lang.github.io/nimble/create-packages.html#nimscript-compatibility)! But I'll leave exploring `nimble` for another time.

Hopefully, with this context, the existing [documentation](https://nim-lang.org/docs/nims.html) and [reference](https://nim-lang.org/docs/nimscript.html) starts to make more sense. 
