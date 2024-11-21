struct Text<'a> {
    content: &'a str,
    count: u32,
}

impl Text<'_> {
    fn increment(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let mut text = Text {
        content: "hello",
        count: 0,
    };

    text.increment();
    text.increment();
    text.increment();

    println!("content: {}\ncount: {}", text.content, text.count);
}
