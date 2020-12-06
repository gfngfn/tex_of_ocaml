
# `tex_of_ocaml`

Note: This project is a kind of joke and probably not of practical use.


## Summary

This software consists of the following:

* `src/`: A compiler for call-by-value untyped lambda-terms (which have OCaml-like syntax) to *(Knuthian) TeX code fully-expandable by `\edef`* (written in Rust)
* `src_tex/secd.sty`: A virtual machine based on the *SECD machine* (written in TeX itself)


## Example usage

Invoke:

```console
$ SRC=examples/example3.txt
$ TARGET_DIR=_generated/
$ TARGET_NAME=example3.tex

# Prepare the target directory:
$ mkdir -p $TARGET_DIR
$ cp src_tex/secd.sty $TARGET_DIR

# Compile a source file to target code:
$ cat $SRC
(fun x -> fun f -> f (arabic (add 1 x))) 42 (append "foo")
$ cargo run $SRC -o $TARGET_DIR/$TARGET_NAME

# Evaluate target code and produce a PDF file:
$ cd $TARGET_DIR
$ latexmk $TARGET_NAME
```

Then you can see “foo43” on the generated PDF.


## Syntax

```
e ::=
    '(' e ')'
  | x                         (variables)
  | 'fun' x '->' e            (lambda abstractions)
  | e e                       (applications)
  | 'let' x '=' e 'in' e      (let-bindings)
  | 'if' e 'then' e 'else' e  (conditionals)
  | p                         (primitive operations)
  | b                         (Boolean literals)
  | n                         (integer literals)
  | s                         (string literals)
x ::= (identifiers)
p ::= 'add' | 'sub' | 'mult' | 'append' | 'arabic' | 'iszero'
b ::= 'true' | 'false'
n ::= (non-negative decimal integer literals)
s ::= (double-quoted string literals)
```


## Supported primitives

* `add : nat -> nat -> nat`
  - integer addition
* `sub : nat -> nat -> nat`
  - integer subtraction where negative results are truncated to zero
* `mult : nat -> nat -> nat`
  - integer multiplication
* `append : string -> string -> string`
  - string concatenation
* `arabic : nat -> string`
  - converts integers to strings
* `iszero : nat -> bool`
  - checks whether the given integer is zero
