#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub trait Draw {
    fn draw(&self);
}

// Trait Object
// We create a trait object by specifying some sort of pointer,
// such as a & reference or a Box<T> smart pointer, then the dyn
// keyword, and then specifying the relevant trait

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// This works differently from defining a struct that uses a generic
// type parameter with trait bounds. That can only be subsituted with
// one concrete type at a time, whereas trait objects allow for
// multiple concrete types to fill in for the trait object at runtime
// If we will only have homogenous collections, using generics and
// trait bounds is preferable because the definitions will be
// monomorphized at compile time to use the relevant concrete type

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("{:?}", self);
    }
}
