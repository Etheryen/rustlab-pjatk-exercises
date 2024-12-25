use std::{cell::RefCell, rc::Rc};

fn one() {
    let num = RefCell::new(5);
    *num.borrow_mut() = 10;

    println!("{}", num.borrow()); // => 10
}

fn two() {
    let text = Rc::new(RefCell::new("Hello"));

    let ref1 = Rc::clone(&text);
    let ref2 = Rc::clone(&text);

    *ref1.borrow_mut() = "Goodbye";

    println!("{}", ref2.borrow()); // => "Goodbye"
}

fn three() {
    let visits = Rc::new(RefCell::new(0));

    let ref1 = Rc::clone(&visits);
    let ref2 = Rc::clone(&visits);
    let ref3 = Rc::clone(&visits);

    *ref1.borrow_mut() += 1;
    *ref2.borrow_mut() += 1;
    *ref3.borrow_mut() += 1;

    println!("{}", visits.borrow()); // => 3
}

fn main() {
    one();
    two();
    three();
}
