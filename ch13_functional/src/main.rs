use std::time::Duration;

struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
{
    calculation: T,
    values: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: std::cmp::Eq + std::hash::Hash + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    println!("Chapter 13: Functional Concepts");

    // let simulated_user_specified_value = 30;
    // let simulated_random_number = 7;

    // generate_workout(simulated_user_specified_value, simulated_random_number);

    // Capturing the Environment with Closures
    let x = 4;

    let equal_to_x = |z| z == x; // Closure can access the x variable which is outside it's local scope

    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }

    let y = 4;

    assert!(equal_to_x(y));

    // Processing a series of items with iterators

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2: Vec<_> = v1
        .iter()
        .map(|x: &i32| -> String { (x + 2).to_string() })
        .collect();
    assert_eq!(v2, vec!["3", "4", "5"]);

    let mut v3 = vec![10, 20, 30];
    for v in v3.iter_mut() {
        *v = *v / 10;
    }

    println!("{:#?}", v3);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today do {} pushups", expensive_result.value(intensity));
        println!("Next do {} situps", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!(
                "Today run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }
}
