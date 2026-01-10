const CONST_EXAMPLE: &str = "CONST_EXAMPLE"; // can't be created via String::from("value") because on compile time value should be known;
static STATIC_EXAMPLE: i32 = 123;

fn main() {
    let mut _i8: i8 = 1;
    let mut _i16: i16 = 1;
    let mut _i32: i32 = 1;
    let mut _i64: i64 = 1;
    let mut _i128: i128 = 1;

    _i8 = -5i8;
    _i16 = -5i16;
    _i32 = -5i32;
    _i64 = -5i64;
    _i128 = -5i128;

    let mut _u8: u8 = 1;
    let mut _u16: u16 = 1;
    let mut _u32: u32 = 1;
    let mut _u64: u64 = 1;
    let mut _u128: u128 = 1;

    _u8 = 5u8;
    _u16 = 5u16;
    _u32 = 5u32;
    _u64 = 5u64;
    _u128 = 5u128;

    let local_string_value: String = String::from("value");
    let mut long_example: i64 = 999999999997;
    long_example += 1;
    let r#if: i32 = 1;
    let _unused_but_because_of_underscore_prefix_compiler_does_not_warn = 5;
    let _ = 10; // discarded variable, can't be used in code
    let small_float: f32 = 1.1; // follows IEEE-754, i.e. supports -+Infinity and NaN
    let big_float: f64 = 1.1;
    let b1: bool = true;
    let b2 = false;
    let _unit: () = (); // Unit type: functional programming version of `void`

    // rust has `never type`, to remember. Is defined using `!`

    println!("CONST_EXAMPLE={}", CONST_EXAMPLE);
    println!("local_string_value={}", local_string_value);
    println!("long_example={}", long_example);
    println!("r#if={}", r#if);
    println!("STATIC_EXAMPLE={}", STATIC_EXAMPLE);
    println!("small_float={}", small_float);
    println!("b1={}, b2={}", b1, b2);
    println!("big_float={}", big_float);
    println!("downcast={}", big_float as f32);
    println!("upcast={}", small_float as f64);
}
