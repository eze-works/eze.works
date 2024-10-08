{
    "title": "Self-describing command line arguments",
    "date": "2024-09-30",
    "labels": ["command-line"]
}
+++

> [!NOTE]
> _Update (2024-10-2):_
>
> This post no longer reflects my opinions on command line parsing.
>
> 1. A deep sub-command tree is not a problem with the syntax. It's fixable by redesigning the CLI in question.
> 1. With the right API, there is no ambiguity when parsing `--option` followed by a `value`.
>    If the user asks for an option called `opt`, and you find `--opt`, you can check if the next thing is a positional argument.
>    If so, that's the value to return.
>    If you find `--opt=value`, return the value without looking ahead.
> 1. I still do not like fused/combined options
>
> Depsite my new perspective, I'll keep this post up. Changing one's mind is an important aspect of learning.

## The problem 

The popular style of command line invocations make for terrible user _and_ developer experiences. 

_Positional arguments are indistiguishible from sub-commands_

```bash
git add remote origin https://git.blah/blah
```

... oh wait, that's wrong. It must be:

```bash
git add origin remote https://git.blah/blah
```

... wrong again. It's actually:

```bash
git remote add origin https://git.blah/blah
```

I can't be the only one that has tripped over this numerous times.

_Deeply nested sub-commands are not discoverable_

You use this command whenever you want to authenticate to Google APIs:

```bash
gcloud auth application-default login
```

Imagine you have never run this command on your current machine.
It is not present in your shell history, so you try to navigate the help to find it:

```bash
gcloud 
# Ugh ... a huge wall of help text to scoll past.
# You see the `auth` subcommand. Promising...
gcloud auth help
# Woops, wrong order
gcloud help auth
# Lots more output
gcloud help auth application-default
# Aha!, found the incantation in one of the examples
gcloud auth application-default login
```

This isn't discoverable.
I have to rely on my shell history to meaningfully interact with the  `gcloud` cli.

_Parsing options and positional arguments is ambiguous_

You can't tokenize a command line invocation without extra information:

```bash
tar --create --gzip --file archive.tar.gz  file.to.archive
```

When you hit `archive.tar.gz`, is that a positional argument? or a value for the `--file` flag?

More annoying is the prevalence of "fused"-style options.

Instead of this:

```bash
pacman --query --info firefox
```

Many CLI libraries let you write this:

```bash
pacman -Qi firefox
```

Now, without the specification of the `pacman` program, is that:

1. A `-Q` flag, followed by an `-i` flag, followed by a positional argument?
2. Or is it a `-Q` flag followed by an `-i` option with a value of `firefox`?


## An new hope

Parsing command line arguments should not be this difficult.
All these problems can be solved in one swoop by tweaking the syntax of command line invocations:

The new rules:
1. Options and their values are always written as one shell "word" separated by `=`.
   (e.g. `--level=info`, `-f=archive.tar`)
1. Fused-style arguments are not allowed.
1. Sub-commands are prefixed with `@`
   - This removes the ambiguity between subcommands and positional arguments.
1. You may only have one sub-command and it must be the first argument.
   - "But what about global options??!!", you ask:
     I think environment variables are a better fit.
     I would argue that this `GIT_FORMAT=json git @log` is clearer than `git --format json @log`

Examples:

```bash
git @remote-add origin http://git.blah/blah
gcloud @auth-application-default-login
tar @create --gzip --file=archive.tar.gz file.to.archive
pacman @q --info=firefox 
```

Naturally, I have implemented a [library that recognizes this syntax](https://github.com/eze-works/bind-args).
