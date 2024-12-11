use std::future::Future;

use futures::executor::block_on;

async fn async_filter<T, P, F>(elements: Vec<T>, predicate: P) -> Vec<T>
where
    P: Fn(&T) -> F,
    F: Future<Output = bool>,
{
    // Zrobione imperatywnie, bo .filter() nie może utworzyć
    // async closure, poza tym async closures są unstable

    let mut filtered = Vec::new();

    for el in elements {
        if predicate(&el).await {
            filtered.push(el);
        }
    }

    filtered
}

fn main() {
    let elements = (0_u32..10).collect::<Vec<_>>();
    let predicate = |arg: &u32| {
        let arg = *arg;
        async move { arg % 2 == 0 }
    };

    let filtered = block_on(async_filter(elements, predicate));

    println!("{:?}", filtered);
}
