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

# run all tests
cargo test
# run specific test
cargo test -- --exact my_module::test_inc_1
# all unit-tests are built into a single file

# Integration tests
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── module1.rs
│   └── module2.rs
└── tests/
    ├── integration-tests_1.rs
    └── integration-tests_2.rs

# each test .rs file is a separate crate which is built into a executable

cargo new test_project --lib

cd test_project
cargo test

# dependency: nextest - gives more functionalities
cargo install cargo-nextest --locked
cargo nextest run

# Run:
$PROJECT_ROOT/target/release/rust-demo1

# inside debug shell
print arr
#shows: (int[3])  ([0] = 1, [1] = 2, [2] = 3)

# cargo install cargo-expand
# cargo expand - tool to debug macros
# wrapper around: cargo rustc --profile=check -- -Zunpretty=expanded

```

## Library

```toml
# main.rs -> lib.rs

[lib]
crate-type = ["lib"]

# lib - can be any type of library, compatible with any other rust program
# rlib - static rust library, *.rlib
# dylib - rust only dynamic library .dll, .so, .dylib
# cdylib - C compatible dynamic library .dll, .so, .dylib
# staticlib - static *.a, *.lib C compatible
# procmacro - library with procedural macros

# lib - is the most popular
# crate-type=["lib"]

# in src/lib.rs
# pub fn sum2(a: i32, b: i32) -> i32 {
#     a + b
# }

# Cargo.toml deps. support

[dependencies]
crate = { git = "https://github.com/aa/qqq.git", branch = "main" }

[dependencies]
crate = { path = "/abc/deq" }

[dependencies]
crate = { version = "1.0", registry = "repository" }
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

## STD Traits

- `Debug` -> implements `println!("{:?}")`
- `Clone` -> implements clone()
- `Copy` -> implements `=` which executes clone instead of change of ownership
- `Hash` - implements hash function for the type, calls hash for each field, can be implemented manually
- `PartialEq`, `Eq` - equals logic
- `PartialOrd`, `Ord` - ordinal logic used on sort ops
- `#[derive(Default)]` - sets default value for all fields, `let options: SomeOptions = Default::default();`
- `Deref`

  ```java
  let x = Box::new(5);
  let y: &i32 = &x; // works because Box<T> implements Deref<Target = T>
  // is same as
  let y: &i32 = x.deref();
  ```

- `Iterator`
- `From`, `Into`
- `AsRef<T>`
- `AsMut`
- `Borrow` - should follow Eq, Ord and Hash of the wrapped value
- `BorrowMut`
- `ToOwned`
- `Drop` - destructor
- `Sized` - mark trait, tells that the size is known on compile time
- `Sync`
- `Send`

## Difference between `Eq` and `PartialEq`

- `PartialEq` forces to implement `==` and `!=`, e.g. f32: PartialEq but not Eq because of NAN == NAN -> false
- `Eq` extends `PartialEq` and satisfies contract: reflexive, transitive and symmetric

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

- Additional executables are placed in `src/bin`
  - `cargo run --bin fibonacci_sum`
  - `cargo run --bin fibonacci_prod`
  - `cargo run --bin rust-demo1`
  - `cargo build --bin fibonacci_sum`

- What is crate?
  - An abstract unit that can be either lib or executable
- What is a package?
  - A full rust project which can have multiple executables `src/main.rs` and `src/bin/*.rs` and libs
- What is a module?
  - a file/folder with files which exports functions, structs, traits and impls

## References

1. <https://stas-norochevskiy.gitbook.io/roll-in-to-rust>
1. Dependency repository: <https://crates.io/>
1. Rust tips and tricks: <https://github.com/awesome-rust-com/awesome-rust>
1. Rust's versioning approach <https://semver.org/>
