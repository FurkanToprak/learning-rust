# learning-rust

This is a directory of code I wrote when learning Rust.

## Directory Tree

```
.
├── hello_world.rs
├── quadratic_formula # User input practice.
└── README.md
```

# Summary

## Scripts

| Script                     | Function                |
| -------------------------- | ----------------------- |
| `cargo new <project_name>` | Make new cargo project  |
| `cargo build`              | Build project           |
| `cargo build --release`    | Build optimized project |
| `cargo run`                | Run project             |
| `cargo check`              | Compile check           |

## Data Types

Rust is statically typed. Most of the time Rust can infer type at compile time. If possibility of multiple types, Rust demands type annotation

```
let guess: u32 = some_object.foo();
```

### Scalar Types

- integers

| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

#### Integer Literals

| Number literals  | Example       |
| ---------------- | ------------- |
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | b'A'          |

- floating-point numbers

Found as `f32` and `f64`

- boolean

Found as `bool`

- characters

Found as `char`

### Compound Types
* tuple

Used as 
```
let tup: (type1, type2, type3, ....) = (value1, value2, value3, ...);
```
* array

```
// Explicitly declaring
let a = [1, 2, 3, 4, 5];
// Explicitly declare with type annotation
let b: [type; n_length] = [value_1, value_2, ..., value_n];
// make array of length n populated with value
let c = [value; length];
```
Index
```
let x = array[index];
```

## Functions
