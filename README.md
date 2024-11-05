# (SK) Starkey
A data-science programming langauge made to make the manipulation and handling of data precise, deliberate, and free of error.

## An explanation
> This repository was originally going to be used for `Lamuta` which was a separate language I was working on. I went a few days before deciding it was silly to work one these both at the same time. That said, the code for this was better than Starkey's most recent code, so I just renamed this repository to `Starkey`.
> For a more detailed history, see the `darcylang` repository activity, which was my 'original' repo for Starkey.
> Starkey has previous versions in my private repositories dating back to March of 2024, the oldest of which is `darcylang`, which is public here: `https://github.com/alarie-c/darcylang`

## 10/25/24
> Redid the lexer and parser to use `&'a str` instead of creating random `String` variables for no reason. Everything points to the source file, everything lives for `'a`.

## 10/31/24
> I began to write the parser with the intention of it being a shift-reduce parser... It's not, but it works and is better than my original parser(s) so I'm going to work with it for now and then optimize it later as needed.

## 11/4/24
> Realized the parser I wrote before didn't handle the ambiguity of the grammar very well, and also that I wanted a more elegant solution--thus, I have rewritten it to hopefully follow an LR/LALR parser format. Whether it really does that in practice is another question but it works so far.

> In essence, it has the stack and tree. The stack only ever has a single statement worth of expressions on it. As soon as the parser hits a semicolor (or similar terminating token) it tries to reduce based on the current state. The state is set as it encounters tokens. For example, when it reaches a `var` token, the state is set to `"VAR EXPR"` which is then used to turn an identifier and any other expr into a `VarExpr` which is pushed to the tree.

> The only problem I forsee with this is that everything needs to be parsed into a single thing. Any kind of ambiguity has to be encapsulated in some way. For example, function parameters have to be a single thing, because I'm not about to reduce an ambiguous number of identifiers into what's a function name, parameter, type annotation, return type, etc.

# 11/5/24
> This parser is working really well so far. There are a couple parts of the code that are super duper sketchy that I need to work out and handle a little better, but other than that, everything works as intended and is scalable/modular exactly the way I had planned.