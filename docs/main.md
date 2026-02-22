# Marquage: A simple and easy-to-read markup language

## What Marquage
Marquage is an efficient and easy-to-read markup language. Its syntaxes are similar to JSON, but provide powerful expression ability, including native-supported comment, strings without quotes, etc.
Its interpreter is built in Rust, so it has high-performance and is safe.

## Why Marquage
There already has many markup language, such as `JSON`, `YAML` and `TOML`. `Marquage` is not designed to replace them, but **provide a totally new way to store data, manage your config files and transfer data.**

## Tutorial
### Syntax
#### Data type
There nine different data types:
|Type|Literal|
|:----:|:----------:|
|Void|void|
|RawString|string|
|QuotedString|"string"|
|UnsignedIntegerNumber|1|
|SignedIntegerNumber|-1|
|FloatNumber|0.1|
|Boolean|true, false|
|Array|[]|
|Object|{}|

**Note**: Interpreter can not distinguish whether a positive integer number has a sign or not. We will fix it in the future.

#### Basic Syntax
In Marquage, **every entry must have key and value**. That's means a standard `Marquage` file is composed of different entries. We call entry block.

For example:
```marquage
block = "I am a block"; # This is a valid block
```

Like C-styled programme language, a semicolon is a must behind an entry. If not, interpreter will complain about it. **Specially**, there is no need to put a semicolon behind an object, for it has clear boundary to identify.

For example:
For example:
```marquage
block = "I am a block without semicolon"
                                        ^
                                        Missing semicolon

object = {
  hello = world;
}
 ^
 No semicolon here
```

Here comes array. In `Marquage`, an array could hold different types, like `["string", 1, 1.0, true, void]`. **Specially**, a following comma is allowed: `["string",]`. Also an object can be an element in an array.

For example: 
```marquage
array = [
  {
    hello = world;
  }
];
```

