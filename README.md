# Tiny Code Interpreter in Rust
I am trying to build an code interpreter in Rust. It is an small attempt to do this. I am trying to understand how an interpreter works.

## Features
- `Tokenizer` to tokenize the element
- `Parser` is to pull all the tokens from `Tokenizer` and validate the rule
- `Interpreter` is to supply each line to `Parser` and get nodes and execute the nodes.

## Demo
### Input file as code.txt
with similar code text like below
```txt
let String name = "john";
printer("my name is {{ name }}, Ok!");
```

### Output
according to input
```console
my name is john, Ok!
```
