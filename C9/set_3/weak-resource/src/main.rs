use std::rc::Rc;

fn main() {
    let resource = Rc::new("Hello".to_string());

    let weak = Rc::downgrade(&resource);

    match weak.upgrade() {
        Some(data) => println!("data: {}", data),
        None => println!("no data found"),
    };
}
