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
    println!("small_float={aaa}", aaa = small_float); // print with named placeholder
    println!("b1={}, b2={}", b1, b2); // print with args, similar to C sprintf
    println!("big_float={big_float}"); // print with explicit variable inside the string
    println!("downcast={}", big_float as f32);
    println!("upcast={}", small_float as f64);

    // printf works for type that defines std::fmt::Display, similar to java toString or C++ ostream << operator
    // {}  - std::fmt::Display
    // {:?} - std::fmt::Debug

    {
        let _b = 2;
    }
    // scopes are similar to java, variables lifetime is related to the scope

    let scope_result: i32 = {
        let a = 1;
        let b = 1;
        let c = 1;
        a + b + c
    }; // ; is required
    println!("scope_result={}", scope_result);

    // by default scope returns Unit type i.e. ()
    let _unit2: () = {
        let _b = 2;
    };
    let _unit3: () = {
        let _b = 2;
        ()
    };

    {
        // immutable references
        let example_of_i32: i32 = 90;
        let ref_to_example_of_i32: &i32 = &example_of_i32;
        println!("example_of_i32={}", example_of_i32);
        println!("ref_to_example_of_i32={}", ref_to_example_of_i32);
    }

    {
        // mutable references
        let mut a: i32 = 5;
        let ref_a: &mut i32 = &mut a;
        println!("Value in a is {}", ref_a); // Value in a is 99
        *ref_a = 99;
        println!("Value in a is {}", ref_a); // Value in a is 99
    }

    // rust reference is handled by compiler, C's reference is a real data

    // immutable array
    {
        let arr: [i32; 3] = [1, 2, 3]; // can't be expanded, known on compile time
        println!("Array is {arr:?}");
    }

    // mutable array
    {
        let mut arr = [1, 2, 3];
        arr[1] = 55;
        println!("Array is {arr:?}");
    }
    // by default array in the function scope is stored on stack

    // vector usage
    // vector has pointer to the first element in heap, len and capacity
    // initial size is not standardized
    {
        let mut my_vec: Vec<i32> = Vec::with_capacity(10);
        my_vec.push(1);
        my_vec = Vec::new();
        my_vec.push(1);
        my_vec.push(2);
        my_vec.push(3);

        let third: i32 = my_vec[2];
        println!("3-rd element: {}", third);
    }

    {
        let my_vec = vec![1, 2, 3];
        let third: i32 = my_vec[2];
        println!("3-rd element: {}", third);
    }

    // slice is a real data which is composed of: address to the first element and length
    {
        let arr = [0, 1, 2, 3, 4];
        let slice: &[i32] = &arr[2..=4];
        println!("{}", slice.len()); // 3
        println!("{}", slice[2]); //  4
    }

    {
        let mut v: Vec<i32> = Vec::with_capacity(5);
        v.push(0);
        v.push(1);
        v.push(2);
        v.push(3);

        let slice: &mut [i32] = &mut v[1..3];
        slice[0] = 9;

        println!("v[1]: {}", v[1]); // 9
    }

    {
        let c: char = '😄';
        println!("Java's 2 byte char can't store rust's 4 byte char:{}", c);
    }

    // String type and &str (string's slice (address, length))
    {
        let s1: &str = "some text";
        let s2 = "some text";
        //. type &str, similar to C's const char*
        // for this example let s2 = "some text"; string lives in binary’s read-only section
        // but because it is slice, it can reference string in heap, stack or binary's section

        println!("s1={s1}");
        println!("s2={s2}");
    }

    // String is a wrapper for Vec<u8>
    // String::from copies &str to a new String instance
    // String::new creates empty string (can be populated)
    // String's buffer lives in heap
    {
        let slice: &str = "text";
        let s = String::from(slice);
        println!("s={s}");
    }

    {
        println!("Please enter some text and hit Enter button");

        let mut buf = String::new();
        // let _ = std::io::stdin().read_line(&mut buf);
        populate_str(&mut buf);
        println!("You have entered: {buf}");
        println!(
            "without first character: {}",
            copy_string_ignoring_first_character(&buf)
        );
        let a_slice_2: &str = buf.as_str();
        println!("a_slice_2={a_slice_2}")
    }

    // format usage
    {
        let s: String = format!("{} in the power of the 2 is {}", 3, 9);
        println!("{s}");
    }

    // parenthesis for condition is ambiguous, generates warn
    // {} for then else is required even for one statement
    {
        let a = -5;
        let mod_a: i32 = if a < 0 { -a } else { a };
        println!("{mod_a}"); // 5
    }

    // loop
    {
        let mut n = 5;
        while n > 0 {
            println!("{n}");
            n -= 1;
        }

        // do-while emulation
        while {
            println!("do-while emulation");
            false // condition
        } {}

        // while true
        let r = loop {
            println!("infinite loop emulation");
            break 12;
        };
        println!("r={r}");

        let arr = [10, 20, 30, 40, 50];

        for element in arr {
            println!("the value is: {}", element);
        }
        for i in 0..arr.len() {
            // [0..arr.len()-1]
            println!("Index: {}, Value: {}", i, arr[i]);
        }
        // rangeClosed()
        for i in 0..=arr.len() {
            // [0..arr.len()]
            println!("Index: {}", i);
        }
    }

    println!("end")
}

fn populate_str(s: &mut String) {
    s.push_str("😄hello 1");
    s.push('1');
    s.push('2');
}

fn copy_string_ignoring_first_character(s: &String) -> String {
    let start = s.char_indices().nth(1).map_or(s.len(), |(i, _)| i);
    let slice: &str = &s[start..];
    return slice.to_string();
}
