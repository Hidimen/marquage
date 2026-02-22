# Marquage
`Marquage` is a simple and easy-to-read markup language. This library provides powerful parsing and generating ability.
## What Marquage
Marquage is an efficient and easy-to-read markup language. Its syntaxes are similar to JSON, but provide powerful expression ability, including native-supported comment, strings without quotes, etc.
Its interpreter is built in Rust, so it has high-performance and is safe.

## Why Marquage
There already has many markup language, such as `JSON`, `YAML` and `TOML`. `Marquage` is not designed to replace them, but **provide a totally new way to store data, manage your config files and transfer data.**

## How to use
```rust
use marquage_lib::{from_str};
use marquage_derive::{Parse, Generate};

#[derive(Parse, Generate, Debug)]
struct Person{
  name: String,
  age: u8,
  email: String,
  married: bool,
}

let data = r###"
name = "Alice";
age = 20;
email = "alice@example.com";
married = false;
"###;
println!("{:?}", from_str::<Person>(data).unwrap());
```
## Contribute
We are looking forward to your contributions. Issues and pull requests are welcomed.