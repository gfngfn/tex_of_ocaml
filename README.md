
# `tex_of_ocaml`

Note: This project is a kind of joke and probably not of practical use.


## Summary

This software consists of the following:

* A compiler for untyped lambda-terms (which have OCaml-like syntax) to *(Knuthian) TeX code fully-expandable by `\edef`* (written in Rust)
* A virtual machine based on the *SECD machine* (written in TeX itself)


## How to use

```console
$ SRC=examples/example2.txt
$ TARGET_DIR=_generated/
$ TARGET_NAME=example2.tex

$ cat $SRC
(fun x -> x) 42

$ mkdir -p $TARGET_DIR

$ cp src_tex/secd.sty $TARGET_DIR

$ cargo run $SRC -o $TARGET_DIR/$TARGET_NAME

$ cd $TARGET_DIR

$ latexmk $TARGET_NAME
```
