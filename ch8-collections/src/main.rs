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
}
