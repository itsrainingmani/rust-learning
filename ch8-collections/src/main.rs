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
}
