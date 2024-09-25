{
    "title": "Notes on Nix",
    "date": "2024-09-22",
    "labels": ["nix", "notes"],
    "stage": "draft"
}
+++

Update:
In the end, i gave nix a quick try by installing it on archlinux. I quickly moved on.
I can endure a new language & way of doing things as long as its decently6 documented.
Nix suffers from a deep lack of cohesive documentation. The most annoying example here is the availability of two "official" installers. One by determinate systems , and the other by nixos.
Don't get me started about documentation. There is a nixwiki...I went to the part about flakes (since that's what's currently recommended even if it's "experimental"??)...and i could not understand it...even though i feel like i have good grasp on the underlying concepts of nix (and i love them!)
I'm not saying its no understandable eventually...it's just not worth it to me.
A good tool without good explanation on its use is bad (i'm looking at you hugo).
Idk who is in charge...but until a grownupi steps up, I won't waste my time here.

I found [this extrememly useful repository](https://github.com/KFearsoff/nix-drama-explained) for providing context about what has happenend in nix.
I don't have any conclusions to draw...but the fact that the project is still


[Link](https://edolstra.github.io/pubs/phd-thesis.pdf)

---
NOTES ABOUTE USING NIX:

`nix-shell`: Reference says its for debugging derivation building...but nix.dev clarifies that was before. Now its used as a general way for creating temporary environments
Source: <https://nix.dev/tutorials/first-steps/declarative-shell#a-basic-shell-nix-file>

---
NOTES ABOUTE THE PHD PAPER

# Introduction

>  Deployment problems also seem curiously resistant to automation: the same concrete problems appear time and again
p.11

- Makes the good point that deployment has not really been academically studied. It's been mostly patched together as we went.

 Correct deployment should be a matter of copying a few files. However, in practice, you run into:
- Environment issues: The software being deployed might have explicit and implicit dependencies on the environment in which it runs
  We must somehow _identify_ what the software's requiremnts are, then _realize_ them
- Manageability issues: The software's lifecycle needs to be managed. Upgrades, uninstalls and updates need pieces of information that is not easily gleaned.
  For example, to uninstall, you have to figure out what the dependencies are so you can uninstal them as well...but you need to be careful not to install things depended on by other programs.


# An overview of Nix

- Central to how nix works is the nix store
- The nix store stores software "components".
- The definition of a component is loose...but you can think of it as a "package"
  - As far as nix is concerned a component is just a set of files in a file system
- The nix store stores each component as a directory. Part of the directory path is a cryptographic hash of all the inputs that went into building that component
  - Thus nix prevents _undeclared dependencies_ and _component interference_
    - Undeclared dependencies are not possible because the builder is pure. It starts out with an empty PATH and uses a patched dynamic linker without default lookup paths.
      Thus the only way to specify dependencies is via paths to the nix store!  Woaw.
  - Inputs include the sources of components, the script that performed the build, the arguments and env vars passed to the build script, all build time depedencies
- The paper is titled "purely functional model" because components don't change after they have been built. They are in fact marekd readonly on the file system.
  This is similar to functional languages like haskel where the result of a function call depends solely on its inputs

  [???]: I don't get the part about retained dependencies. What i understand is that apparently nix figures out runtime dependencies by scanning the binaries for  occurences of the cryptographic hash.
  This is kind of interesting. Use pure functional model when building to specify build dependencies. Scan build output for runtime dependencies...

You create a package in three steps:
- You create a nix expression. This is nix-speak for creating a nix function that takes as input other components (that have to be built before) that you need to build your component.
- You create a builder. This is usually a script that does the actual build
- You call your nix expression with its inputs (this is called composing).
  This was big aha moment for me...this is done in a single file that declares what i think are  variable with ALL the packages in the nix package collection, and sets them to the result of calling their nix epression with arguments that are other packages?


Installing a built component is a matter of making it availble for the user to call (e.e.g putting it in the PATH).
This is done using `nix-env` (which apparently i understand i should avoid?).
nix-env is "an abstraction over stor paths"...a sort of "view" to what components are accessible to the user at a given point

These nix-env "views" are called "generations"

Nix doesn't actually execute nix expressions directly. It transforms them into an intermediate level language called "store derivations". These are stored in the nix store.
These store derivations are basically a manifest of what it takes to build a component.

So to build a component, you:
- Compile the nix expression to a store derivation
- "Execute the store derivation". This is called "realizing" the store derivation.
  This "realizes the effect of the derivation in the file system"


THe "closure" of a component means all the other components that are needed for it to run.
The "closure" of a store derivation means all the other derivations needed for the derivation to be "realized".

The first is basically a binary deployment.
The second is basically a source deployment.


# Deployment as Memory Management
  
This section is about drawing parallels between memory management  and software/package management.
The same way assembly treated memoyr as unstructured, but modern languages impose rigor, so do current package management techniques treat the filesystems as unstructured, but nix imposes rigor.


This section is about drawing parallels between memory management  and software/package management.
The same way assembly treated memoyr as unstructured, but modern languages impose rigor, so do current package management techniques treat the filesystems as unstructured, but nix imposes rigor.


>  File systems on the one hand and RAM on the other hand are just two different levels in the storage hierarchy.
> Where programs manipulate memory cells, deployment operations manipulate the file system.
> This analogy reveals that the safeguards against abuse of memory applied by programming languages are absent in deployment, and suggests that we can make deployment safe by trying to impose similar safeguards
page 61

> In this thesis, I will use the following definition of “software component”:
> - A software component is a software artifact that is subject to automatic composition.
>   It can require, and be required by, other components.
> - A software component is a unit of deployment.
page 58

Whether a component is deployed in source or binary form is an implementation detail. Source files on their own are not components, but coupled with metadata and build scripts, they become one.

Note that a component need not be an executabel pieace of softare either. It could be a non-executable part that is depended upon by an executable component. 

He further specifies that components must be physical entities on the file system. There are multipel ways to establish composition between components ad-hoc (e.g. dynamic linking, pre-processor directives etc...) But what they all have in common is that they are grounded in the filesystem.

File System Paths ==> Pointers
Components ==> Values/obejcts 

It is not obvious how to determine the set of paths that a component references. There are four ways for a component to reference another:
- At build time. (e.g. the component `firefox` will need a reference to a compiler to actually get built)
- At runtime (e.g. a component that references another program when running)
- Retained dependenceis: i.e. dynamic linking. These are paths passed in at build time, that are used at runtime. THey are "Retained" because they are kept in the final binary output
- Path "arithmetic"/manipulation: Given a path, its easy to construct another path

Complete and correct deployment implies figuring out (recursively) the set of files that is pointed to by a component. This is similar to garbage collection. You need to figure out a "pointer graph".

The reason why nix component paths have cryptographic hashes is partly  to solve the problem of identifying "what looks like a path" in arbitrary binary programs.

# The Nix Expression Language

The nix expression language was specially made to define components and their compositions.

In the nix expression language, a component is created through a `derivation`, which is a primitive operation.

> The most important primop (primitive operation), indeed the raison d’être for the Nix expression language,
> is the primop derivation. It translates a set of attributes describing a build action to a store
> derivation, which can then be built. The translation process is performed by the function
> instantiate,
p 88

# The Extensional model

Nix has its own archive format to serialize file system objects called NAR (nix archive format).
Though tar is ubiquitous, it stores more information that nix needs (e.g. timestamps), which might cause two objects that are equal to be serialized non-equally
