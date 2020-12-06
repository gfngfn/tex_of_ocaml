
# `tex_of_ocaml`

Note: This project is a kind of joke and probably not of practical use.


## Summary

This software consists of the following:

* `src/`: A compiler for call-by-value untyped lambda-terms (which have OCaml-like syntax) to *(Knuthian) TeX code fully-expandable by `\edef`* (written in Rust)
* `src_tex/secd.sty`: A virtual machine based on the *SECD machine* (written in TeX itself)


## How to use

```console
$ SRC=examples/example2.txt
$ TARGET_DIR=_generated/
$ TARGET_NAME=example2.tex

# Prepare the target directory:
$ mkdir -p $TARGET_DIR
$ cp src_tex/secd.sty $TARGET_DIR

# Compile a source file to target code:
$ cat $SRC
(fun x -> add x 1) 42
$ cargo run $SRC -o $TARGET_DIR/$TARGET_NAME

# Evaluate target code and produce a PDF file:
$ cd $TARGET_DIR
$ latexmk $TARGET_NAME
```


## Syntax

```
e ::= '(' e ')' | x | 'fun' x '->' e | e e | p | n | s
x ::= (variables)
p ::= 'add' | 'sub' | 'mult' | 'append'
n ::= (non-negative decimal integer literals)
s ::= (double-quoted string literals)
```


## Supported primitives

* `add : nat -> nat -> nat`
* `sub : nat -> nat -> nat`
* `mult : nat -> nat -> nat`
* `append : string -> string -> string`
