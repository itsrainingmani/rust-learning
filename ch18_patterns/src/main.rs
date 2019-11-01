fn main() {
    println!("Chapter 18. Patterns");

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "23".parse();

    if let Some(color) = favorite_color {
        println!("Using your fave color {} as the background", color);
    } else if is_tuesday {
        println!("Tuesday is Green Day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let (x, y, _) = (1, 2, 3);
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

    // let Some(x) = favorite_color;
    // Patterns come in two forms - refutable and irrefutable.
    // Patterns that will match for any possible value passed are irrefutable

    // Matching Literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        // Starts a new scope
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // the y here shadows the y outside the match scope
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {:?}", x, y);

    // Multiple Patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"), // matches against either one or two
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching ranges of values with ...
    let x = 5;

    match x {
        1..=5 => println!("one through five"), // ... range patterns are deprecated
        _ => println!("anything else"),
    }

    let c = 'c';

    match c {
        'a'..='j' => println!("Early ascii letter"),
        'k'..='z' => println!("Late ascii letter"),
        _ => println!("Something else"),
    }

    // Destructuring Structs
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // let msg = Message::ChangeColor(0, 160, 255);

    // match msg {
    //     Message::Quit => println!("The Quit variant has no data to destructure"),
    //     Message::Move { x, y } => println!("Move in the x direction {} and y direction {}", x, y),
    //     Message::Write(txt) => println!("Text message: {}", txt),
    //     Message::ChangeColor(r, g, b) => {
    //         println!("Change color to red {}, green {}, blue {}", r, g, b)
    //     }
    // }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, value {}",
            h, s, v
        ),
        _ => (),
    }

    // Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    foo(3, 4);

    // Ignoring parts of a value with a nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("can't overwrite an existing customized value"),
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => println!("Some nums: {}, {}, {}", first, third, fifth),
    }

    let _x = 5;
    // let y = 10;

    // Ignoring remaining parts of a value with ..

    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {}", x), // the .. pattern ignores any parts of a value that we haven't explicitly matched
                                                     // this is quicker than saying y: _, z: _
    }

    match numbers {
        (first, .., fourth, fifth) => println!("Some nums: {}, {}, {}", first, fourth, fifth),
    }

    // Extra conditionals with match guards
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {:?}", x, y);

    // @ Bindings
    // The at operator (@) lets us create a variable that holds a value at the same time
    // we're testing that value to see whether it matches a pattern
    enum SimpleMessage {
        Hello { id: i32 },
    }

    let msg = SimpleMessage::Hello { id: 5 };

    match msg {
        // SimpleMessage::Hello {
        //     id: id_variable @ 3..=7,
        // } => println!("Found an id in range: {}", id_variable),
        SimpleMessage::Hello { id: id_variable } if id_variable > 3 && id_variable <= 7 => {
            println!("Found an id in range: {}", id_variable)
        }
        SimpleMessage::Hello { id: 10..=12 } => println!("Found an id in another range"),
        SimpleMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y param: {}", y);
}

struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
