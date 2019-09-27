const MAX_POINTS: u32 = 100_000;

fn some_func() {
    println!("Some Function");
}

// Function signatures require that each parameter is type annotated
fn another_func(x: i32) {
    println!("The value of the parameter is: {}", x);
}

fn func_2(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn main() {
    //For example, in cases where you’re using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances.
    //With smaller data structures, creating new instances and writing in a more functional programming style may be easier to think through,
    //so lower performance might be a worthwhile penalty for gaining that clarity.

    // Mutating a variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // const
    println!("The max number of points achievable is: {}", MAX_POINTS);

    // Variable shadowing
    let y = 5; // binds y to value of 5;
    let y = y + 1; // shadows y by taking the original value and adding 1;
    let y = y * 2; // shadows y by taking the previous value and mult by 2;
    println!("The value of y is: {}", y);

    // Shadowing vs mut
    // We're effectively creating a new variable when we use let
    // so we can change the type or perform some transformations
    // but re-use the name
    // We are not allowed to mutate a variable's type only it's value

    // Data Types
    // Rust is a statically typed language - must know types of all variables at compile time
    let _answer: u32 = "42".parse().expect("Not a number!");

    // floats
    let _x1 = 2.0; // defaults to f64
    let _y1: f32 = 3.0;

    // Mathematical ops
    let _sum = 5 + 10;
    let _diff = 95.4 - 123.3;
    let _product = 4 * 23;
    let _quot = 56.7 / 32.2;
    let _rem = 43 % 5;

    // boolean - one byte
    let _t = true;
    let _f: bool = false;

    // char - 4 bytes, single quotes
    let _c = 'z';
    let _z = 'Z';
    let _cat = '😻';

    // Compound Data Types - Tuples & Arrays
    let tup = (500, 6.4, 1); // Tuples are heterogenous and are fixed in length once initialized
    let (x, y, z) = tup; // destructuring the tuple
    println!("The value of (x, y, z) is: ({}, {}, {})", x, y, z);

    // Array
    // * Homogenous collection of values
    // * Stack allocated - contiguous in memory
    // * Fixed length like tuples
    let _a = [1, 2, 3, 4, 5]; // Type = [i32; 5] -> [value type; size]

    let _a = [3; 5]; // [3, 3, 3, 3, 3]

    let _index = 10;
    // let _element = _a[_index]; // This produces a panic rather than allowing you to access invalid memory location
    // println!("The value of element {} is: {}", index, element);

    some_func();
    another_func(35);
    func_2(1243, 2421435);

    let block_stmt = {
        let something = 20;
        something + 20 // IMPORTANT: Expressions do not include ending semicolons
    };
    println!("The value of block-stmt is: {}", block_stmt);

    let x = five();
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", plus_one(7));

    let number = 3;

    if number != 0 {
        println!("Number was not 0");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    for element in [10, 20, 30, 40, 50].iter() {
        println!("{}", element);
    }

    // (start..end) is a Range type that generates numbers in sequence from start to end
    for element in (1..5).rev() {
        println!("{}", element);
    }

    // Extra Credit assignments
    // 1. Convert Temp between fahrenheit and celsius
    println!("0 deg Celsius is {} deg Fahrenheit:", convert_to_fahrenheit(0.0));
    println!("212 deg Fahrenheit is {} Celsius:", convert_to_celsius(212.0));

    // 2. Generate nth Fibonacci number
    println!("The 16th fibonacci number is: {}", fibonacci(16));

    // 3. The Twelve Days of Christmas
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelve"];
    let carol = [
        "partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve Lords a leaping"
    ];

    for (i, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas, my true love sent to me", day);
        for num in (0..i+1).rev() {
            if i + 1 > 1 && num == 0 {
                print!("And a ");
            } else if i == 0 {
                print!("A ");
            }
            println!("{}", carol[num]);
        }
        println!("");
    }
}

fn fibonacci(n: u32) -> u32 {
    let mut f1;
    let mut f2 = 0;
    let mut f_n = 1;
    match n <= 1 {
        true => n,
        false => {
            for _ in 1..n {
                f1 = f2;
                f2 = f_n;
                f_n = f1 + f2;
            }
            f_n
        }
    }
}

fn convert_to_fahrenheit(temp: f32) -> f32 {
    (temp * 9.0/5.0) + 32.0
}

fn convert_to_celsius(temp: f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0
}

// Returns a value of type i32
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
