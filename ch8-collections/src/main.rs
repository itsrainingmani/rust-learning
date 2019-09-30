use std::collections::HashMap;

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

    // Iterating through a UTF-8 string's characters and printing them
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Creating an empty HashMap and inserting values into it
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];

    // Zipping together the iters of two vectors and constructing a HashMap from the zip
    // Type annotation HashMap<_, _> is needed since collect can produce many different data structures
    // and we need to specify one. Rust can infer the types of the key and values so we use underscores
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:#?}", teams);

    // For types that implement the Copy trait like i32, the values are copied into the hashmap
    // For owned values like String, the values will be moved and the hashmap will be the owner
    // of those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{:#?}, {:#?}", field_name, field_value);

    let team_name = teams.get(0);
    if let Some(s) = team_name {
        print!("{}", s);
        if let Some(i) = scores.get(&s) {
            println!(": {}", i);
        }
    }

    for (key, value) in &scores {
        println!("{}: {}", *key, *value);
    }

    // Updating a HashMap

    // Overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores); // {"Blue": 25} the og value of 10 is overwritten

    // Only inserting a value if a key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
