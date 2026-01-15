# Rust demo

Minimal rust project

```sh
# Build dev/debug
cargo build
#Build release
cargo build --release

# create helloworld
cargo new test_rust

cargo new hello_world --bin
#  --bin - executable (src/main.rs)
#  --lib - library (src/lib.rs)
# Cargo.toml - config file for the project
# TOML (Tom's Obvious, Minimal Language)

cargo build
./target/debug/rust-demo1

cargo build --release
./target/release/rust-demo1

# run
cargo run

# crate = module

# show dependency tree
cargo tree

# Run:
$PROJECT_ROOT/target/release/rust-demo1

# inside debug shell
print arr
#shows: (int[3])  ([0] = 1, [1] = 2, [2] = 3)

# cargo install cargo-expand
# cargo expand - tool to debug macros
# wrapper around: cargo rustc --profile=check -- -Zunpretty=expanded

```

## Padding example

```rust
struct MyStruct {
    a: i32,
    b: i64,
}
```

- as you can see, it takes 12 bytes
- rust applies padding for `a` to make it `size % 8 == 0` in order to minimize amount of instructions with the cost of memory
- `println!("Size = {}", std::mem::size_of::<MyStruct>()); // Size = 16`

    ```rust
    let arr = [
        MyStruct { a: 1, b: 2 },
        MyStruct { a: 3, b: 4 },
        MyStruct { a: 5, b: 6 },
    ];
    println!("arr[0].a: {:p}", &arr[0].a); // arr[0].a: 0x7ffdc124d970
    println!("arr[0].b: {:p}", &arr[0].b); // arr[0].b: 0x7ffdc124d968
    println!("arr[1].a: {:p}", &arr[1].a); // arr[1].a: 0x7ffdc124d980
    println!("arr[1].b: {:p}", &arr[1].b); // arr[1].b: 0x7ffdc124d978
    println!("arr[2].a: {:p}", &arr[2].a); // arr[2].a: 0x7ffdc124d990
    println!("arr[2].b: {:p}", &arr[2].b); // arr[2].b: 0x7ffdc124d988
    ```

- <img src="./_forReadme/image.png" />

  - image taken from <https://stas-norochevskiy.gitbook.io/roll-in-to-rust>

- Unlike C lang, Rust can change order of fields in the struct. To keep order use annotation `#[repr(C)]`

## Code editor + debugger

Codium with these plugins:

<img src="./_forReadme/image1.png" />

## Q&A

- Generics: why do 2 ways of defining generics exist, `generic parameter (<T>)` and `associated types (type T;)`?
  - `Generic parameters (<T>)` - the caller choses T `parser.parse::<String>();`
  - `Associated types (type T;)` - The implementor chooses `Output`

    ```java
    impl Parser for JsonParser {
        type Output = String;
    }
    ```

- `uuid = { version = "1.17.0", features = ["v4"] }`
  - cargo will use latest `major.(minor.patch)` - 1.18.1, to force 1.17.0 write: `uuid = { version = "=1.17.0", features = ["v4"] }`

## References

1. <https://stas-norochevskiy.gitbook.io/roll-in-to-rust>
1. Dependency repository: <https://crates.io/>
1. Rust tips and tricks: <https://github.com/awesome-rust-com/awesome-rust>
1. Rust's versioning approach <https://semver.org/>
