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

- tuple

Used as

```
let tup: (type1, type2, type3, ....) = (value1, value2, value3, ...);
```

- array

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

```
fn main() {
    foo(val_1, val_2, ..., val_n);
}

fn foo(param_1: type_1, param_2: type_2, ..., param_n: type_n) {
    // Statement
}
```

#### Note

Function definition are statements, which do not return values. Rust does not allow statements to be attached to non-existant values.

Therefore in rust, you can not do the following:
```
fn main() {
    let x = (let y = z);
}
```

### Returning in functions and declaring function's return value
```
fn increment(value) -> type {
    value + 1
}
```

### Control flow
```
if boolean_1 {
    statement_1;
} else if boolean_2 {
    statement_2;
} else {
    statement_n;
}
```
### Ternary
```
let number = if condition { value_1 } else { value_2 };
```
Note that `value_1` and `value_2` must have the same type.

## Loops
```
// Runs until Ctrl+C
let mut value = 0;
loop {
    println("This is an infinitr loop");
    if condition {
        break value;
    }
}

// While loop
while condition {
    statement();
}

// For loop
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}

// For loop with numerical iterator
for number in (a..b).rev() {
    println!(number);
}
```