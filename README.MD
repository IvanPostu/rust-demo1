# Rust demo

Minimal rust project

```sh
rustup default stable
# or rustup toolchain install nightly

# setup sqlx cli tool
cargo install sqlx-cli

# run main
cargo run --bin sequence_10
# run other
cargo run --bin test_axum
kill -SIGINT $(ps aux | grep test_axum | awk 'NR==1 {print $2}')

cargo run --bin test_sigint_sigterm
kill -SIGINT $(ps aux | grep test_sigint_sigterm | awk 'NR==1 {print $2}')
kill -SIGTERM $(ps aux | grep test_sigint_sigterm | awk 'NR==1 {print $2}')

cargo run --bin test_axum2
cargo test --bin test_axum2

cargo run --bin test_sqlx

cargo test --bin test_containers

cargo run --bin sqlx_macro
# validate sqlx macros
cargo sqlx prepare --database-url postgres://postgres:1111@localhost/mydb

cargo run --bin axum_sqlx

cargo run --bin openapi_test
cargo run --bin openapi_test2

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

## Setup postgres container

```sh
docker run --name my_pg_container \
  -p 5432:5432 \
  -e POSTGRES_PASSWORD=1111 \
  -d postgres:17-alpine

docker exec -it my_pg_container bash

psql -U postgres

CREATE DATABASE mydb;

\c mydb;

\list # show all dbs

# ┌────────────┐         ┌────────────────┐
# │  accounts  │         │  transactions  │
# ├────────────┤         ├────────────────┤
# │ id         │───┐     │ id             │
# │ owner_name │   │     │ amount         │
# │ balance    │   ├────*│ src_account_id │
# └────────────┘   └────*│ dst_account_id │
#                        │ tx_timestamp   │
#                        └────────────────┘

\dt # show current tables
```

```sql
CREATE SEQUENCE accounts_seq START WITH 1000;

CREATE TABLE accounts ( -- mydb.public.accounts 
    id BIGINT PRIMARY KEY DEFAULT nextval('accounts_seq'),
    owner_name VARCHAR(255) NOT NULL UNIQUE,
    balance NUMERIC(10, 2)  NOT NULL DEFAULT 0.00 CHECK (balance >= 0)
);

CREATE SEQUENCE transactions_seq START WITH 1000;

CREATE TABLE transactions ( -- mydb.public.transactions 
    id BIGINT PRIMARY KEY DEFAULT nextval('transactions_seq'),
    amount NUMERIC(10, 2) DEFAULT 0.00,
    src_account_id BIGINT NOT NULL,
    dst_account_id BIGINT NOT NULL,
    tx_timestamp TIMESTAMP NOT NULL,
    FOREIGN KEY (src_account_id) REFERENCES accounts (id),
    FOREIGN KEY (src_account_id) REFERENCES accounts (id)
);

INSERT INTO accounts(id, owner_name, balance) VALUES
(1, 'John Doe',    1000.00),
(2, 'Ivan Ivanov', 2000.00);

INSERT INTO transactions(amount, src_account_id, dst_account_id, tx_timestamp)
VALUES
(10.00, 1, 2, TO_TIMESTAMP('2025-12-11 14:00:00', 'YYYY-MM-DD HH24:MI:SS')),
(20.00, 2, 1, TO_TIMESTAMP('2025-12-12 15:00:00', 'YYYY-MM-DD HH24:MI:SS'));
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

## VecDeque

- <img src="./_forReadme/image copy.png" />

## Newtype pattern

- Rust has orphan rule, if trait and struct are parts of other crate then we can't define new methods as a part of that trait
- Solution is Newtype pattern (`struct NewType(ForeignType)`)

## Atomic wrappers

| Primitive / Pointer | Atomic equivalent |
|---------------------|-------------------|
| bool | std::sync::atomic::AtomicBool |
| u8 | std::sync::atomic::AtomicU8 |
| u16 | std::sync::atomic::AtomicU16 |
| u32 | std::sync::atomic::AtomicU32 |
| u64 | std::sync::atomic::AtomicU64 |
| i8 | std::sync::atomic::AtomicI8 |
| i16 | std::sync::atomic::AtomicI16 |
| i32 | std::sync::atomic::AtomicI32 |
| i64 | std::sync::atomic::AtomicI64 |
| usize | std::sync::atomic::AtomicUsize |
| isize | std::sync::atomic::AtomicIsize |
| *mut T | std::sync::atomic::AtomicPtr<T> |

## Serialization

- Rust doesn't have a std serialize logic
- there is `serde` crate which exposes Serialize/Deserialize traits which are considered by all serialization dependencies
- <img src="./_forReadme/image copy 2.png" />

## Logging

- `tracing` - facade (plusses: supports async & can be used fordeps. that use `log` facade)
  - `tracing-subscriber` - implementation
- `log` - facade
  - `env_logger`
  - `log4rs`

## Blocking vs. Non-blocking operation

- Blocking
  - `t1` makes a system call requesting a blocking I/O operation, for example, reading bytes from an open network connection
  - `t1` “falls asleep” and is marked in the thread table as waiting to be awakened once the I/O operation completes
  - `At` some point, the kernel performs the actual read from the device and places the data into a buffer for the program to read
  - `t1`'s entry in the thread table is updated. The thread is now ready to continue execution
  - The scheduler assigns a time quantum to `t1`
  - `t1` wakes up and reads the data that is already available in the buffer
- Non-blocking
  - Receive multiple connections from clients, and each connection is represented by a network socket.
    - Instead of immediately reading input data from each socket, we collect the socket descriptors and store them in an array
  - All descriptors are passed together to the appropriate system call (for example, `epoll_wait`)
    - Depending on the specific operating system, the non-blocking API differs: on Linux it may be `select`, `poll`, `epoll`, or `io_uring`; on Windows — `IOCP`; on BSD/macOS — `kqueue`, etc.)
  - The `epoll_wait` system call blocks the calling thread and, when it wakes up, returns a list of descriptors for which data is ready to be read.
  - We iterate over the list of ready descriptors (returned by `epoll_wait`) and, for each one, first read all available bytes and then immediately write them back to the socket. After that, we close the socket
  - Next, we call `epoll_wait` again for the remaining descriptors, and repeat this process until all of them have been processed
- <img src="./_forReadme/image copy 3.png" />

## Crates for backend development

- `Axum` - lightweight and fast web framework for building HTTP servers
- `SQLx` - interact with relation DBs
- `Tower` - library for building client side / server side logic
- `Utopia` - library for OpenAPI / Swagger
- `Metrics` - library for metrics

## Usage of `epool` in Linux

```c
// see linux_epool_example.c
```

## JS's old approach - callbacks

- <img src="./_forReadme/image copy 4.png" />

## Actor approach

- Do not call each other directly, they use queue for communication
- <img src="./_forReadme/image copy 5.png" />
- Erlang/OTP uses this approach
- WhatsApp uses it for backend

## Fibers - a.k.a. coroutines, green threads, user space threads

A fiber scheduler typically has two pools of operating system threads at its disposal:

- threads for executing fiber computations
- a thread dedicated to performing input/output operations (an I/O thread)

With such a scheduler, we can implement our own I/O API specifically for fibers, which works as follows:

- An I/O operation is invoked in fiber X.
- The scheduler removes this fiber from execution on the OS thread and places it into a waiting queue. It then takes another fiber that is ready to run from the queue and resumes its execution on the OS thread.
- The I/O request is sent to a dedicated I/O thread, which continuously processes I/O requests using `epoll` or another appropriate API.
- When the dedicated I/O thread completes the I/O operation for fiber X, it stores the result for that fiber and marks the fiber as ready to continue execution.
- At some point, the fiber scheduler resumes execution of fiber X on an OS thread. By that time, the result of the I/O operation is already available to the fiber.
- <img src="./_forReadme/image copy 6.png" />

## Async rust

- Rust language and standard library expose API that follows fibers approach. `async runtime` (which includes `executor` as a main element) can be implemented differently
- async function in rust returns fiber(future) object
- to get the result, the fiber(future) should be executed inside `async runtime`/ executor

## Executor interaction with Future

1. A Future is polled
1. It is not ready → returns Poll::Pending
1. The Future registers interest in some event (I/O, timer, channel)
1. The Future (or the underlying system) stores the Waker from the Context
1. When the event occurs → waker.wake() is called
1. The executor re-polls the Future
<img src="./_forReadme/image copy 7.png" />

## Shutdown hook

- without explicit shutdown hook tokio stops requests that are ongoing
- <img src="./_forReadme/image copy 8.png" />
- with explicit shutdown hook, server waits until existing requests are processed
- <img src="./_forReadme/image copy 9.png" />
  - new requests are rejected

## Sqlx Migrations

- `cargo install sqlx-cli`, requires openssl & pkg-config, see shell.nix
- `cargo sqlx migrate add accounts -r --sequential`
- `cargo sqlx migrate add transactions -r --sequential`
- `psql > CREATE DATABASE my_migration;`
- `cargo sqlx migrate run  --database-url postgres://postgres:1111@localhost/my_migration`

  ```sh
    # revert
    cargo sqlx migrate revert \
        --target-version 1 \
        --database-url postgres://postgres:1111@localhost/my_migration
  ```

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
- What is the difference between `Send` and `Sync`?
  - `Send`: a value can be moved to another thread
  - `Sync`: a value can be shared between threads by reference
- Sync mechanisms
  - `Mutex` - exclusive lock
  - `RwLock` - read/write lock
  - `Condvar` - t1 sleeps, t2 awakes t1
  - `Barrier` - sync N threads on specific section of code
- What is thread safe version of `Rc`?
  - `Arc` - Atomic reference counter
- What is poisoned mutex?
  - if t1 acquired mutex and finished with panic, then if t2 tries to acquire that mutex gets `PoisonError`
- Enumerate `std::sync::atomic::Ordering`
  - Relaxed: “just don’t tear”
  - Acquire: “see what was published”
  - Release: “publish what I did”
  - AcqRel: “update with sync”
  - SeqCst: “everyone agrees on order”
- Difference between `channel` and `sync_channel`
  - `channel` doesn't have size
  - `sync_channel` - has size, if is exceeded, producer waits in blocked state until there is space for element
- How to add a dependency with explicit feature?
  - `cargo add serde --features "derive"`
- Difference between `concurrency` and `parallelism`?
  - `parallelism` - a concept describing the physical execution of instructions on two different processors or cores
  - `concurrency` - a concept describing the logical execution of instructions across multiple processors or cores
    - “logical” is the key word here — concurrency can happen even on a single core via interleaving, such problems as `data race`are still present
- What is middleware's order?
  - reversed order of definition, i.e. last `.layer` becomes first
- What is the benefit if we create middleware/request handler by implementing `Service` trait from `tower` dependency?
  - more flexibility, because it can have state
- What does `CorsLayer` middleware do?
  - adds `Access-Control-Allow-Origin` header
- Enumerate services and middlewares
  - `ServeDir` - serves all files inside the directory
  - `Redirect`
  - `CompressionLayer`
  - `RequestDecompressionLayer`
  - `NormalizePathLayer`
  - `RequestBodyLimitLayer`
  - `TimeoutLayer`
  - `TraceLayer`
  - `InFlightRequestLayer`
- Why it is important to handle SIGTERM?
  - When k8s kills a pod it triggers SIGTERM, waits 30s and if it is still running triggers SIGKILL
- Difference between macros `query_scalar!`, `query!`, `query_as!` and functions `query_scalar`, `query`, `query_as`?
  - macros validates on compile time e.g. `cargo sqlx prepare --database-url postgres://postgres:1111@localhost/mydb` sql's correctness
    - types are known on compile time
    - is problematic when prod/dev tables differ
  - function is good when query is built dynamically
  - performance is same

## References

1. <https://stas-norochevskiy.gitbook.io/roll-in-to-rust>
1. Dependency repository: <https://crates.io/>
1. Rust tips and tricks: <https://github.com/awesome-rust-com/awesome-rust>
1. Rust's versioning approach <https://semver.org/>
1. Rust Atomics and Locks - Mara Bos
1. Tokio official docs. <https://tokio.rs/tokio/tutorial>
1. <https://swagger.io/specification/>
