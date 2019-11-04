use std::slice;

// FFI
extern "C" {
    // The "C" part defines which Application Binary Interface, the external function uses:
    // The ABI defines how to call the function at the assembly level. The "C" ABI is the most
    // comon and follows the C programming language's ABI
    fn abs(input: i32) -> i32;
}

// Calling Rust Functions from Other Languages
#[no_mangle] // Tells the Rust compiler to not mangle the name of this function
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Global - the name is in Screaming Snake Case
static HELLO_WORLD: &str = "Hello, world!"; // has the lifetime of 'static

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        // Accessing and modifyiung a mutable static variable is unsafe
        COUNTER += inc;
    }
}

fn main() {
    println!("Chapter 19. Unsafe Rust");

    let mut num = 5;

    // This can be written in safe Rust. However to derefence these pointers you'll need an unsafe block
    let r1 = &num as *const i32; // Immutable raw pointer
    let r2 = &mut num as *mut i32; // Mutable raw pointer

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = new_split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("Name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn new_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
    // (&mut slice[..mid], &mut slice[mid..])
}

unsafe fn dangerous() {}
