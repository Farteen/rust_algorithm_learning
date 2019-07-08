use std::rc::Rc;
use crate::basic::test_box_deref_trait::List::{Cons, Nil};

#[test]
fn test_deref_point() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[test]
fn test_box_deref_trait() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

#[test]
fn test_drop_trait() {
    let c = CustomSmartPointer { data: String::from("my stuff")};
    let d = CustomSmartPointer { data: String::from("other stuff")};
    println!("CustomSmartPointers created.");
    drop(c);
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[test]
fn test_ref_cell() {
    let a = Rc::new(Cons(10, Rc::new(Nil)));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}