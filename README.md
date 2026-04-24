# Tiny Code Interpreter in Rust
A small experimental interpreter written in Rust. It is an small attempt to do this. I am trying to understand how an interpreter works. It has many limitations even in my existing code, but i am experimenting and exploring system programming

## Features
- `Tokenizer` to convert input text into meaningful tokens (keywords, identifiers, literals, etc.)
- `Parser` is to pull all the tokens from `Tokenizer`, Consumes tokens and builds structured expressions
- `Interpreter` is to supply each line to `Parser` and Executes parsed expressions and maintains variable state

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

## How to run
```console
cargo run
```
Make sure your input file is named `code.txt` and placed in the project root.

## Limitations
- Supports only String type
- Only one {{variable}} per string is supported
- Minimal error handling (uses panic in some cases)
- No complex expressions or control flow yet

## Learning Goals
This project is part of my exploration into:
- how interpreters work internally
- parsing and tokenization
- moving from high-level development to system-level thinking
