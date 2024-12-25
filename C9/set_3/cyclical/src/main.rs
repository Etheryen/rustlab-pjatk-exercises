use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct A {
    b: Rc<RefCell<B>>,
}

#[derive(Debug)]
struct B {
    a: Weak<RefCell<A>>,
}

fn main() {
    let b = Rc::new(RefCell::new(B { a: Weak::new() }));
    println!("b.a: {:?}", b.borrow().a.upgrade()); // => None
    {
        let a = Rc::new(RefCell::new(A { b: Rc::clone(&b) }));

        b.borrow_mut().a = Rc::downgrade(&a);
        println!("b.a: {:?}", b.borrow().a.upgrade()); // => Some

        println!();

        println!(
            "a - strong: {}, weak: {}",
            Rc::strong_count(&a),
            Rc::weak_count(&a),
        );

        println!(
            "b - strong: {}, weak: {}",
            Rc::strong_count(&b),
            Rc::weak_count(&b),
        );
    }

    println!(
        "b - strong: {}, weak: {}",
        Rc::strong_count(&b),
        Rc::weak_count(&b),
    );
    println!("b.a: {:?}", b.borrow().a.upgrade()); // => None
}
