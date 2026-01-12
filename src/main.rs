mod mod1;
mod mod2;

const CONST_EXAMPLE: &str = "CONST_EXAMPLE"; // can't be created via String::from("value") because on compile time value should be known;
static STATIC_EXAMPLE: i32 = 123;

const PI: f32 = 3.14;
const TAU: f32 = double(PI);

const STATIC_LIFETIME_STR: &'static str = "STATIC_LIFETIME_STR";
const IMPLICIT_STATIC_LIFETIME_STR: &str = "IMPLICIT_STATIC_LIFETIME_STR";

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

    {
        println!("safe_divide(2.2, 0.0)={}", safe_divide(2.2, 0.0));
        println!("safe_divide(2.2, 0.1)={}", safe_divide(2.2, 0.1));
        println!("safe_divide2(2.2, 0.0)={}", safe_divide2(2.2, 0.0));
        println!("safe_divide2(2.2, 0.1)={}", safe_divide2(2.2, 0.1));
        println!("fibonacci_nth_element(7)={}", fibonacci_nth_element(7));
    }

    {
        fn gen_num() -> i32 {
            // variable v is of never type
            let _v = return 5;
        }
        let r = gen_num();
        println!("r={r}");

        println!("TAU={TAU}");
        let q = double(r as f32);
        println!("q={q}");
    }

    {
        println!("sum_with_previous={}", sum_with_previous(1)); // 1
        println!("sum_with_previous={}", sum_with_previous(2)); // 3
        println!("sum_with_previous={}", sum_with_previous(7)); // 9
        println!("sum_with_previous={}", sum_with_previous(-6)); // 1
    }

    // tuples
    {
        let employee: (&str, i32, bool) = ("John Doe", 1980, true);
        println!(
            "Name: {}, birth year: {}, active: {}",
            employee.0, employee.1, employee.2
        );
        let (name, birth_year, is_active) = employee;
        println!(
            "Name: {}, birth year: {}, active: {}",
            name, birth_year, is_active
        );
    }

    {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let (odds, evens) = split_to_odd_and_even(&numbers);
        println!("Odd numbers:  {odds:?}");
        println!("Even numbers: {evens:?}");
    }

    // ownership
    {
        let s1 = String::from("some string");
        let s2 = s1; // calls destructor for s1
                     // s1 is unaccessible, attempt to access it will cause compile time error

        // borrow-checker - mechanism of rust that guarantees that freed variable can't be accessed

        // ownership is modified in the next cases:
        // assignment
        // call a function with argument
        // return from a function
        // closure

        println!("s2={}", s2);
    }

    {
        fn len_of_string(s: String) -> (String, usize) {
            let length = s.len();
            (s, length)
        }

        let s = String::from("aaa");
        let (s, len) = len_of_string(s);
        println!("Len of {s} is {len}");
    }

    // rust has function pub fn drop<T>(_x: T) {}
    // it does nothing, just takes ownership and automatically deletes/frees variable

    {
        // ownership change doesn't work for primitives, primitives are copied
        let i32: i32 = 1;
        #[allow(dropping_copy_types)]
        drop(i32);
        println!("i32={}", i32);
    }

    // borrowing, caller still holds ownership even on call
    {
        // normally instead of s: &String is used string literal s: &str
        fn len_of_string(s: &String) -> (&String, usize) {
            let length = s.len();
            (s, length)
        }

        let s1 = String::from("aaa");
        let (s, len) = len_of_string(&s1);
        println!("Len of {s} is {len}, s1={s1}");
    }

    // The Rule (The Borrowing Rule)
    // For any value in Rust at any point in time, you can have either:
    // Exactly one mutable reference (&mut), or
    // Any number of immutable (readonly) references (&)
    // …but never both at the same time.
    {
        let mut s = String::from("x");

        let r1 = &mut s;
        // let r2 = &s; // compile time error

        println!("r1 = {r1}");
    }

    {
        let arr = [String::from("1"), String::from("2"), String::from("3")];

        // if it is not reference, then println!("{arr:?}"); will fail with borrow error
        for n in &arr {
            println!("{n}");
        }

        println!("{arr:?}");
    }

    // lifetime - a mechanism that guarantees that a reference to an abject doesn't live longer than the object
    {
        // let s1 = String::from("aaa");
        // let longest;
        {
            // compile time error
            // fn take_longest(x: &str, y: &str) -> &str {
            //     if x.len() > y.len() {
            //         x
            //     } else {
            //         y
            //     }
            // }

            // let s2 = String::from("bbbb");
            // longest = take_longest(s1.as_str(), s2.as_str());
        }

        {
            // explicit lifetime, it says x and y should belong to the same lifetime (scope)
            fn _take_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
                if x.len() > y.len() {
                    x
                } else {
                    y
                }
            }

            // let s1 = String::from("aaa");
            // let longest;
            // {
            //     let s2 = String::from("bbbb");
            //     longest = take_longest(s1.as_str(), s2.as_str()); // s2 does not live long enough
            //                                                       // compile time error due to different lifetime of s1 and s2

            //     println!("The longest string is {}", longest);
            // }
        }

        // Lifetime is a protection provided by compiler, sometimes in order to avoid it we can use .clone() method

        // Macros - a mechanism that permits to generate code before or during compilation
        // Declarative macros - works during compilation, manipulates with AST tree
        // Procedural macros - macros that works before compilation
    }

    {
        println!("IMPLICIT_STATIC_LIFETIME_STR={IMPLICIT_STATIC_LIFETIME_STR}");
        println!("STATIC_LIFETIME_STR={STATIC_LIFETIME_STR}");
    }

    {
        let res = sum_numbers!(1, 2);
        // on compilation will be replaced with:
        // let res = 1 + 2;
        println!("Sum is: {}", res);
        println!("result={}", sum_numbers!(if 5 > 4 { 1 } else { -1 }, 9));
    }

    // rust supports vararg only through macros
    {
        let res1 = vararg_enulation_sum!(1, 2, 3, 4, 5);
        println!("Sum is: {}", res1); // Sum is: 15

        let res2 = vararg_enulation_sum_simplified!(1, 2, 3, 4, 5);
        println!("Sum is: {}", res2); // Sum is: 15

        let res3 = vararg_enulation_sum_simplified!();
        println!("Sum is: {}", res3); // Sum is: 15

        // any kind of parenthesis
        let _ = vararg_enulation_sum!(1, 2);
        let _ = vararg_enulation_sum![1, 2];
        let _ = vararg_enulation_sum! {1, 2};
        ()
    }

    {
        make_empty_func!(function_1);
        make_empty_func! {function_2} // if macros generates func or struct and we use {} parenthesis, ; is optional

        function_1();
        function_2();
    }

    {
        let v = custom_vec2![1, 2, 3];
        println!("{v:?}");
    }

    // pointers
    {
        let mut v: i32 = 5;
        let const_ptr1: *const i32 = &v as *const i32;
        let mut_prt1: *mut i32 = &mut v as *mut i32;

        let const_ptr2: *const i32 = &raw const v;
        let mut_prt2: *mut i32 = &raw mut v;

        let const_ptr3: *const i32 = std::ptr::addr_of!(v);
        let mut_prt3: *mut i32 = std::ptr::addr_of_mut!(v);

        let ptr: *const i32 = (&v) as *const i32; // convert address to pointer

        unsafe {
            println!("{}", *const_ptr1); // get value that pointer references
            println!("{}", *mut_prt1); // get value that pointer references
            println!("{}", *const_ptr2); // get value that pointer references
            println!("{}", *mut_prt2); // get value that pointer references
            println!("{}", *const_ptr3); // get value that pointer references
            println!("{}", *mut_prt3); // get value that pointer references
            println!("{}", *ptr); // get value that pointer references
        }
    }

    // bypassing borrowing rule: at most one mutable reference or multiple readonly
    // mutable reference/address -> pointer -> mutable reference
    {
        fn inc(a: &mut i32) {
            *a = *a + 1;
        }

        let mut a = 5;
        unsafe {
            let r1: &mut i32 = &mut a;
            let ptr: *mut i32 = r1 as *mut i32;
            let r2: &mut i32 = ptr.as_mut().unwrap();
            inc(r1);
            inc(r2);
        }
        println!("{a}"); // 7
    }

    // structs
    // struct name - PascalCase
    // struct field - snake case

    {
        struct Person {
            first_name: String,
            last_name: String,
        }

        fn get_full_name(p: &Person) -> String {
            format!("{} {}", p.first_name, p.last_name)
        }

        let first_name = String::from("John");
        let person = Person {
            first_name,
            last_name: String::from("Doe"),
        };

        let full_name = get_full_name(&person);
        println!("{}", full_name); // "John Doe"

        let mut p = Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        p.first_name = "Theodor".to_string();
    }

    {
        struct Person {
            first_name: String,
            last_name: String,
        }

        impl Person {
            fn new(first: &str, last: &str) -> Person {
                Person {
                    first_name: first.to_string(),
                    last_name: last.to_string(),
                }
            }
            fn set_empty_full_name(&mut self) {
                self.first_name = String::new();
                self.last_name = String::new();
            }
            fn get_full_name(&self) -> String {
                format!("{} {}", self.first_name, self.last_name)
            }
        }

        let mut _p = Person::new("John", "Doe");
        let p1 = Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };

        let mut p2 = Person {
            first_name: "Robert".to_string(),
            ..p1
        };

        println!("{} {}", p2.first_name, p2.last_name); // Robert Doe
        println!("Fullname method result: {}", p2.get_full_name()); // Robert Doe
        p2.set_empty_full_name();
        println!("Set_empty + Fullname method result: {}", p2.get_full_name());
    }

    // tuple structs
    // briefly instead of field_name is used index
    // we can add methods
    {
        struct RGB(u8, u8, u8);

        impl RGB {
            fn as_u32(&self) -> u32 {
                ((self.0 as u32) << 16) + ((self.1 as u32) << 8) + (self.2 as u32)
            }
        }

        {
            let mut color: RGB = RGB(255, 0, 0);
            println!("Red channel: {}", color.0);

            color.1 = 255;

            let RGB(r, g, b) = color;

            println!("R={r}, G={g}, B={b}"); // R=255, G=255, B=0

            println!("As number: {}", color.as_u32());
        }
    }

    // singleton structure
    {
        struct Universe;

        impl Universe {
            fn includes(&self, p: &Planet) -> bool {
                println!("{}", p.name); // true
                true
            }
        }

        struct Planet {
            name: String,
        }

        let universe = Universe;

        let earth = Planet {
            name: "Earth".to_string(),
        };
        println!("{} {}", universe.includes(&earth), earth.name); // true
    }

    // struct lifetime
    {
        #[derive(Debug)]
        struct _NameComponents<'a> {
            first_name: &'a str,
            last_name: &'a str,
        }

        // let components;
        {
            let full_name = "John Doe".to_string();

            let _space_position = full_name.find(" ").unwrap();

            // Error: `full_name` does not live long enough
            // components = NameComponents {
            //     first_name: &full_name[0..space_position],
            //     last_name: &full_name[space_position + 1..],
            // };
        }
        // println!("{components:?}");
    }

    // How the struct is represented in memory?
    {
        struct MyStruct {
            a: i32,
            b: i64,
        }

        println!("Size = {}", std::mem::size_of::<MyStruct>()); // Size = 16

        let s = MyStruct { a: 1, b: 2 };
        println!("a: {}", (&s.a as *const i32) as usize); // a: 140731421349072
        println!("b: {}", (&s.b as *const i64) as usize); // b: 140731421349064

        println!("MyStruct at {:p}", &s);

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
    }

    {
        mod a {
            pub fn get_num() -> i32 {
                1
            }
        }

        mod b {
            pub fn get_num() -> i32 {
                get_5()
            }
            // by default is private
            fn get_5() -> i32 {
                5
            }
        }

        println!("{}", a::get_num());
        println!("{}", b::get_num());

        // module as file
        println!("{}", mod1::get_num_5());

        // module as folder
        println!("{}", mod2::do_work(2));
    }

    // The difference between C++ and rust is that C++ generates *.o file for each .h/.cpp source file while rust merges all modules in one .o file

    // Trait - i.e. interface in java or virtual class in c++
    {
        trait CanIntroduce {
            fn introduce(&self) -> String;
        }

        struct Person {
            name: String,
        }

        impl CanIntroduce for Person {
            fn introduce(&self) -> String {
                format!("Hello, I'm {}", self.name)
            }
        }

        let person = Person {
            name: String::from("John"),
        };

        println!("{}", person.introduce()); // Hello, I'm John

        // Polymorphism
        // static dispatching - impl Трэйт
        // dynamic dispatching - dyn Трэйт

        struct Dog {
            #[allow(dead_code)]
            name: String,
        }

        impl CanIntroduce for Dog {
            fn introduce(&self) -> String {
                String::from("Waf-waf")
            }
        }

        fn print_introduction(v: &impl CanIntroduce) {
            println!("Value says: {}", v.introduce());
        }

        let person = Person {
            name: String::from("John"),
        };
        let dog = Dog {
            name: String::from("Bark"),
        };

        print_introduction(&person); // Value says: Hello, I'm John
        print_introduction(&dog); // Value says: Waf-waf

        // for static polymorphism compiler generates a version of method for each type
        // print_introduction_$Person(&person);
        // print_introduction_$Dog(&dog);

        // v keeps 2 addresses, first to the actual object and the second to vtable for specific type
        // vtable - table or virtual calls
        // it basically does: vtable.lookup("introduce").call()
        // dyn Trait - is composed of reference to the object and reference to vtable is named trait object
        fn print_introduction_dyn(v: &dyn CanIntroduce) {
            println!("Value says: {}", v.introduce());
        }
        print_introduction_dyn(&person); // Value says: Hello, I'm John
        print_introduction_dyn(&dog); // Value says: Waf-waf

        // static trait can be passed via value and address
        // dynamic trait can be passed via address and pointer
    }

    // implementing traits for foreign/stranger types
    {
        trait CanIntroduce {
            fn introduce(&self) -> String;
        }

        struct Person1 {
            name: String,
        }

        struct Dog1 {
            #[allow(dead_code)]
            name: String,
        }

        impl CanIntroduce for Dog1 {
            fn introduce(&self) -> String {
                String::from("Waf-waf")
            }
        }

        impl CanIntroduce for Person1 {
            fn introduce(&self) -> String {
                format!("Hello, I'm {}", self.name)
            }
        }

        impl CanIntroduce for &str {
            fn introduce(&self) -> String {
                String::from("I am string slice")
            }
        }

        impl CanIntroduce for i32 {
            fn introduce(&self) -> String {
                String::from("I am integer")
            }
        }

        fn print_introduction(v: impl CanIntroduce) {
            println!("Value says: {}", v.introduce());
        }

        print_introduction("a"); // Value says: I am string slice
        print_introduction(5); // Value says: I am integer

        // Orphan rule - restriction, we can't implement foreign trait for foreign type
        // either type or trait should be our

        // return Trait
        fn _make_someone(is_person: bool) -> impl CanIntroduce {
            if is_person {
                return Person1 {
                    name: String::from("John"),
                };
            }
            return Person1 {
                name: String::from("John"),
            };

            // can't return 2 different types
            // else {
            //     Dog { name: String::from("Bark") }
            // }
        }

        // dyn can return different types
        fn make_someone_dyn(is_person: bool) -> Box<dyn CanIntroduce> {
            if is_person {
                // we can't return &dyn Trait because it will be stored inside the stack and it will be erased as soon as method exits
                Box::new(Person1 {
                    name: String::from("John"),
                })
            } else {
                Box::new(Dog1 {
                    name: String::from("Bark"),
                })
            }
        }

        let person = make_someone_dyn(true);
        let dog = make_someone_dyn(false);

        fn print_introduction_dyn(v: &dyn CanIntroduce) {
            println!("Value says: {}", v.introduce());
        }

        print_introduction_dyn(person.as_ref());
        print_introduction_dyn(dog.as_ref());

        // Box an wrapper that keeps address in the heap
    }

    // default methods
    {
        trait CanIntroduce {
            fn say_name(&self) -> String;
            fn introduce(&self) -> String {
                format!("Hello, I am {}", self.say_name())
            }
        }

        struct Person {
            name: String,
        }

        impl CanIntroduce for Person {
            fn say_name(&self) -> String {
                self.name.clone()
            }
        }

        let person = Person {
            name: String::from("John"),
        };
        println!("{}", person.introduce()); // Hello, I am John
    }

    // trait inheritance
    {
        trait HasName {
            fn say_name(&self) -> String;
        }

        trait CanIntroduce: HasName {
            fn introduce(&self) -> String;
        }

        struct Person {
            name: String,
        }

        impl CanIntroduce for Person {
            fn introduce(&self) -> String {
                format!("Hello, I am {}", self.say_name())
            }
        }
        // required, or else compile time error!
        impl HasName for Person {
            fn say_name(&self) -> String {
                self.name.clone()
            }
        }

        let person = Person {
            name: String::from("John"),
        };
        println!("{}", person.introduce()); // Hello, I am John

        // combine types/traits
        fn _print_worker_introduction(_v: &(impl CanIntroduce + HasName)) {}
    }

    // Self - means object of type that implements the trait
    {
        trait HasDefaultConstructor {
            fn make_default() -> Self;
        }

        struct Person {
            name: String,
        }

        impl HasDefaultConstructor for Person {
            fn make_default() -> Self {
                // -> Person | Self is also valid
                Person {
                    name: "Anonymous".to_string(),
                }
            }
        }

        let p = Person::make_default();
        println!("Default name: {}", p.name);
    }

    // unsafe trait
    {
        struct _MyStruct {
            a: i32,
            b: i64,
        }

        unsafe trait _MyTrait {
            fn do_something_dangerous();
        }

        unsafe impl _MyTrait for _MyStruct {
            fn do_something_dangerous() {}
        }
    }

    {
        struct Point2D {
            x: i32,
            y: i32,
        }

        // PartialEq - is analogue of java's equals method
        impl PartialEq for Point2D {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        let p1 = Point2D { x: 1, y: 1 };
        let p2 = Point2D { x: 1, y: 1 };
        println!("p1 = p2: {}", p1 == p2);

        #[derive(Debug, Clone, PartialEq)] // checks equality for each field
        struct Point2D2 {
            x: i32,
            y: i32,
        }

        let p1 = Point2D2 { x: 1, y: 1 };
        let p2 = Point2D2 { x: 1, y: 1 };
        println!("p1 = p2: {}", p1 == p2);

        let p3 = Point2D2 { x: 0, y: 0 };
        let p4 = Point2D2 { x: 1, y: 1 };
        println!("p3 = p4: {}", p3 == p4);

        // #[annotation(args)]
        // annotation is a mechanism that allow rust to use a specific pre-processor that generates code
        // e.g. Hash, Debug, Default

        let p5 = p4.clone();
        println!("p4 = p5: {}", p4 == p5);
        println!("p4={:?}, p5={:?}", p4, p5);

        // clone can be implemented manually
        impl Clone for Point2D {
            fn clone(&self) -> Point2D {
                Point2D {
                    x: self.x,
                    y: self.y,
                }
            }
        }
        // the difference is that #[derive(Clone)] requires all fields to be cloneable
    }

    // trait Copy
    {
        #[derive(Debug, Clone, Copy)]
        struct Point2D {
            #[allow(dead_code)]
            x: i32,
            #[allow(dead_code)]
            y: i32,
        }

        let p1 = Point2D { x: 1, y: 1 };
        let p2 = p1; // calls p1.clone()

        println!("p1={:?}, p2={:?}", p1, p2);
        // p1=Point2D { x: 1, y: 1 }, p2=Point2D { x: 1, y: 1 }
    }

    // destructuring
    {
        // tuple
        let employee: (&str, i32, bool) = ("John Doe", 1980, true);
        let (name, birth_year, is_active_employee) = employee;
        println!("name={name}, birth_year={birth_year}, is_active_employee={is_active_employee}");
    }

    {
        // nested tuples
        let tup: (i32, char, bool, (i32, i32, i32)) = (1, 'z', true, (7, 7, 7));
        let (num, c, _, t) = tup;
        println!("num={}, char={}, triplet={:?}", num, c, t);
    }
    {
        // destructure nested tuples
        let tup: (i32, char, bool, (i32, i32, i32)) = (1, 'z', true, (7, 8, 9));
        let (num, c, _, (d1, d2, d3)) = tup;
        println!("num={num}, char={c}, d1={d1}, d2= {d2}, d3={d3}");
    }

    // destructuring arrays
    {
        let arr: [i32; 3] = [1, 2, 3];
        let [a1, a2, a3] = arr;
        println!("a1={a1}, a2={a2}, a3={a3}");
    }
    {
        // ignore remaining
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let [a_1, _, a_3, ..] = arr;
        println!("a1={}, a3={}", a_1, a_3);
    }

    {
        // tail as sub-array
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let [a_1, _, a_3, rest @ ..] = arr;
        println!("a1={}, a3={}, rest={:?}", a_1, a_3, rest);
    }

    {
        // destructure struct
        struct Person {
            name: String,
            age: u32,
        }

        let p = Person {
            name: String::from("John"),
            age: 25,
        };
        let Person { name, age } = p;
        println!("Name={}, Age={}", name, age);
    }
    {
        // destructure only needed fields
        struct Person {
            name: String,
            _age: u32,
        }

        let p = Person {
            name: String::from("John"),
            _age: 25,
        };
        let Person { name, .. } = p;
        println!("Name={name}");
    }

    // matching
    {
        let a = 1;
        match a {
            0 => println!("The number is 0"),
            1 => println!("The number is 1"),
            _ => (),
        }
        match a {
            0 => println!("The number is 0"),
            2 => println!("The number is 2"),
            _ => println!("The number is not 0 or 2"),
        }
        match a {
            0 => println!("The number is 0"),
            2 => println!("The number is 2"),
            x => println!("The number {x}"),
        }
        match a {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => println!("The number is less than 10"),
            _ => println!("The number is equal to or greater than 10"),
        }
        match a {
            0..=9 =>
            // от 0 до 9 inclusive
            {
                println!("The number is less than 10")
            }
            10..100 =>
            // от 10 до 100 exclusive
            {
                println!("The number is in range [10,99]")
            }
            _ => println!("The number is equal to or greater than"),
        }
        match 22 + 22 {
            x @ (0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9) => println!("{x} is less than 10"),
            x @ 10..100 => println!("{x} is in range [10,99]"),
            x => println!("{x} is equal to or greater than"),
        }

        let b = -5;
        let absolute = match b {
            ..0 => -b,
            _ => b,
        };
        println!("{absolute}");

        // matching for strings should use slice
        let name = String::from("Robert Smith");
        let is_anonymous = match name.as_str() {
            "Anonymous" | "John Doe" => true,
            _ => false,
        };
        println!("{is_anonymous}");

        // match statement for vector's slice
        let v = vec![1, 2, 3, 4, 5];
        let s = match v.as_slice() {
            [] => 0,
            [a, b, c, ..] => a + b + c,
            _ => -1,
        };

        println!("{}", s); // 6

        #[derive(Debug)]
        struct Person {
            name: String,
            age: u32,
        }

        let p = Person {
            name: String::from("John"),
            age: 17,
        };
        match p {
            Person {
                ref name,
                age: 1..18,
            } => println!("Person {name} is not adult"),
            Person { ref name, age: 18 } => println!("Person {name} just turned 18"),
            Person { ref name, .. } => println!("Person {name} is adult"),
        }
        println!("rest={:?}", p);

        match p {
            Person { name, age } if age < 18 => println!("Person {name} is not adult"),
            Person { name, age } if age == 18 => println!("Person {name} just turned 18"),
            Person { name, .. } => println!("Person {name} is adult"),
        }

        let mut p = Person {
            name: String::from("Anonymous"),
            age: 25,
        };
        match p {
            Person { ref mut name, .. } if name == "Anonymous" => {
                *name = "John Doe".to_string();
            }
            Person { .. } => (),
        }
        println!("{p:?}"); // Person { name: "John Doe", age: 17 }
    }

    // Anonymous functions (Lambda functions)
    // functional programming sh*t
    {
        let inc: fn(i32) -> i32 = |x: i32| x + 1;
        let a = 1;
        let b = inc(a);
        println!("{b}"); // 2

        let _inc_1 = |x: i32| x + 1;
        let _inc_2 = |x: i32| x + 1;
    }

    {
        // HighOrderFunction - function that takes a function argument or return function

        fn transform(a: i32, f: fn(i32) -> i32) -> i32 {
            f(a)
        }

        let inc: fn(i32) -> i32 = |x: i32| x + 1;
        let a = 9;
        let b = transform(a, inc);
        println!("{b}"); // 10
    }
    {
        fn create_inc() -> fn(i32) -> i32 {
            |x: i32| x + 1
        }

        let inc = create_inc();
        let a = 1;
        let b = inc(a);
        println!("{b}"); // 2
    }

    // Anonymous function is compiled and moved to code segment space and on runtime the variable is a pointer to that function
    {
        fn func_inc(x: i32) -> i32 {
            x + 1
        }

        let inc: fn(i32) -> i32 = func_inc;
        let a = inc(7);
        println!("{a}"); // 8
    }

    {
        // compile time error: expected fn pointer, found closure
        // fn make_inc_with_step(step: i32) -> fn(i32) -> i32 {
        //     |x| x + step
        // }
    }
    // the diff between closure and pure function is that:
    // pure function's result only depends on it's args
    // closure is a complex structure that grabs variables from the current context
    {
        fn make_inc_with_step(step: i32) -> impl Fn(i32) -> i32 {
            // implement trait Fn
            move |x| x + step // move tels to the compiler to grab variables in this context
        }

        let inc_with_5 = make_inc_with_step(5);
        let a = inc_with_5(2);
        println!("{a}"); // 7
    }

    // Closure types:
    // Fn - can only read vars from the context, thread safe
    // MutFn - same as Fn but can change by mutable reference, thread unsafe
    // FnOnce - takes ownership by value and destroys them after execution

    {
        let salutation = "Hello".to_string();

        let greet = |name: &str| make_greeting(&salutation, name);

        println!("{}", greet("John")); // Hello John

        print_string(salutation); // OK, data is still usable

        fn make_greeting(salutation: &str, name: &str) -> String {
            format!("{} {}", &salutation, name)
        }

        fn print_string(s: String) {
            println!("{s}")
        }
    }

    {
        let salutation = "Hello".to_string();

        let greet = |name: &str| make_greeting(salutation, name);

        println!("{}", greet("John")); // Hello John

        print_string("aaa".to_string());
        // print_string(salutation); // Error: use of moved value: `salutation`

        fn make_greeting(salutation: String, name: &str) -> String {
            format!("{} {}", &salutation, name)
        }

        fn print_string(s: String) {
            println!("{s}")
        }
    }

    {
        fn make_greeting(salutation: &String, name: &str) -> String {
            format!("{} {}", &salutation, name)
        }
        fn make_greet_closure() -> impl Fn(&str) -> String {
            let salutation = "Hello".to_string();
            move |name: &str| make_greeting(&salutation, name)
        }

        println!("{}", make_greet_closure()(&"test"));
    }

    {
        let mut step = 1;

        // impl FnMut(i32)->i32
        let mut growing_inc = |x: i32| {
            let step_ref = &mut step;
            let res = x + *step_ref;
            *step_ref += 1;
            res
        };
        println!("{}", growing_inc(1)); // 2
        println!("{}", growing_inc(1)); // 3
        println!("{}", growing_inc(1)); // 4
    }

    {
        let mut step = 1;

        // impl FnMut(i32)->i32
        let mut growing_inc = |x: i32| {
            let res = x + step;
            step += x;
            res
        };
        println!("{}", growing_inc(1)); // 2
        println!("{}", growing_inc(1)); // 3
        println!("{}", growing_inc(1)); // 4
        println!("{}", step); // 4
        step += 1;
        println!("{}", step); // 5
    }

    {
        fn print_and_destroy(s: String) {
            println!("{s}");
        }

        let text = "text".to_string();
        let print_and_destory_text = || print_and_destroy(text);
        print_and_destory_text();
        // print_and_destroy_text(); can't be called
        // println!("{}", text); // text is destroyed
    }

    {
        fn _make_inc_with_step(step: i32) -> impl Fn(i32) -> i32 {
            move |x| x + step
        }

        // let inc_with_5: impl Fn(i32) -> i32 = make_inc_with_step(5);
        // can't specify the type because it will be generated on compilation

        // unstable (nightly) bullsh*t
        // type MyFn = impl Fn(i32) -> i32;
        // let inc_with_5: MyFn = make_inc_with_step(5);
    }

    {
        // rustc --version - 1.82.0
        fn make_inc(is_decrement: bool) -> impl Fn(i32) -> i32 {
            if is_decrement {
                move |x| x - 1
            } else {
                move |x| x + 1
            }
        }

        let inc = make_inc(true);
        let dec = make_inc(false);

        println!("{}", inc(5)); // 4
        println!("{}", dec(5)); // 6
    }

    {
        fn make_inc(is_decrement: bool) -> Box<dyn Fn(i32) -> i32> {
            if is_decrement {
                Box::new(move |x| x - 1)
            } else {
                Box::new(move |x| x + 1)
            }
        }
        let dec: Box<dyn Fn(i32) -> i32> = make_inc(true);
        let a = 2;
        let b = dec.as_ref()(a);
        let c = dec(a);
        println!("{b} {c}"); // 1
    }

    println!("end")
}

#[macro_export]
macro_rules! custom_vec2 {
    () => { Vec::new() };
    ( $( $x:expr),* ) => {
        {
            let mut _temp = Vec::new();
            $( _temp.push($x); )*
            _temp
        }
    }
}

#[macro_export]
macro_rules! make_empty_func {
    ($func_name:ident) => {
        fn $func_name() {}
    };
}

#[macro_export]
macro_rules! vararg_enulation_sum_simplified {
    ( $( $rest:expr ),* ) => { 0 $( + $rest )* }
}

#[macro_export]
macro_rules! vararg_enulation_sum {
    () => { 0 };
    (  $first:literal $(, $rest:literal )* ) => {
        $first $( + $rest )*
    };
}

// instead of expr we can use: expr, stmt, ty, path, pat, item, block, meta, ident, tt, literal, vis
#[macro_export]
macro_rules! sum_numbers {
    ( $x:expr, $y:expr ) => {
        $x + $y
    };
}

// slice argument is for flexibility, allows using it with array and vector
fn split_to_odd_and_even(numbers: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut odds = Vec::new();
    let mut evens = Vec::new();
    for n in numbers {
        if n % 2 != 0 {
            odds.push(*n);
        } else {
            evens.push(*n);
        }
    }
    (odds, evens)
}

// static variable in the method behaves exactly like in C lang
fn sum_with_previous(x: i32) -> i32 {
    static mut PREV: i32 = 0;
    unsafe {
        let result = PREV + x;
        PREV = x;
        result
    }
}

// const fn can be executed on compile time
const fn double(num: f32) -> f32 {
    num * 2.0
}

fn fibonacci_nth_element(index: usize) -> u32 {
    if index == 0 {
        return 0;
    }
    if index == 1 {
        return 1;
    }

    fn next_fibonacci(x0: u32, x1: u32, next_index: usize, desired_index: usize) -> u32 {
        let x2 = x0 + x1;
        if next_index == desired_index {
            x2
        } else {
            next_fibonacci(x1, x2, next_index + 1, desired_index)
        }
    }

    next_fibonacci(0, 1, 2, index)
}

fn safe_divide(a: f32, b: f32) -> f32 {
    if b != 0.0 {
        a / b
    } else {
        0.0
    }
}

fn safe_divide2(a: f32, b: f32) -> f32 {
    if b != 0.0 {
        return a / b;
    }
    0.0
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
