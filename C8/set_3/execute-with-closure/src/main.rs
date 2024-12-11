fn execute_with_closure<F, A, R>(num: A, closure: F) -> R
where
    F: Fn(A) -> R,
{
    closure(num)
}

fn main() {
    let num = 4;
    let result = execute_with_closure(3, move |arg| arg * num);

    println!("result: {}", result); // => 12
}
