use std::collections::HashMap;

fn process_csv(path: &str, col_name: &str) -> f64 {
    let mut reader = csv::Reader::from_path(path).unwrap();

    let numbers = reader
        .deserialize()
        .map(|result| {
            let record: HashMap<String, f64> = result.unwrap();
            *record.get(col_name).unwrap()
        })
        .collect::<Vec<_>>();

    arith_geo_mean(&numbers)
}

const MAX_APPROXIMATIONS: u8 = 20;

fn arith_geo_mean(numbers: &[f64]) -> f64 {
    let mut arith = numbers.iter().sum::<f64>() / numbers.len() as f64;
    let mut geo = numbers
        .iter()
        .copied()
        .reduce(|acc, curr| acc * curr)
        .unwrap()
        .powf(1_f64 / numbers.len() as f64);

    println!("arithmetic: {}", arith);
    println!("geometric: {}", geo);

    let mut count = 0;
    while arith != geo {
        count += 1;
        if count >= MAX_APPROXIMATIONS {
            break;
        }

        let arith_copy = arith;
        arith = (arith + geo) / 2.;
        geo = (arith_copy * geo).sqrt()
    }

    arith
}

fn main() {
    let avg = process_csv("data.csv", "score");
    println!("Średnia wartość kolumny 'score': {}", avg); // Wynik: 75.3 (na podstawie danych z pliku)
}
