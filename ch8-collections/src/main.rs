fn main() {
    // Collections can contain multiple values
    // Unlike the built in array and tuple types,
    // The data these collections point to is stored on the heap
    // The amount of the data does not need to be known at compile time
    // Data can grow or shrink as the program runs
    println!("Chapter 8 - Common Collections!");

    // Vector - Homogenous
    // Allows you to store more than one value in a single data structure
    // that stores all the values next to each other in memory

    let _v: Vec<i32> = Vec::new();
    let a = [""; 5];
    println!("{:#?}", a);

    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);
    println!("{:#?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    // let first = &v[0];

    // v.push(0);

    // println!("The first element is {}", first);

    let v1 = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // De-referencing the ref to an element and incrementing
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    let mut s = String::new();

    let data = "initial contents";

    let s1 = data.to_string();
    let s2 = "initial contents".to_string();

    let s3 = String::from(data);

    // UTF-8 encoded Strings
    let hello = String::from("السلام عليكم");

    s.push_str(&hello);
    s.push_str(" ");
    s.push_str(data);

    println!("{}", s);

    let mut s = String::from("lo");
    s.push('l'); // Add a char to a string using push

    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // This statement takes ownership of s1, appends a copy of the contents of s2
    // and then returns ownership of the result
    // s1 won't be valid after the addition since ownership has passed to s3
    let s3 = s1 + &s2;
    // println!("{}, {}", s3, s1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // Doesn't take ownership of it's parameters
    println!("{}", s);

    let len = String::from("Здравствуйте").len();
    println!("{}", len);

    let hello = "Здравствуйте";
    // let answer = &hello[0]; //Won't compile since it's easy to try to access an invalid or unexpected value

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}
