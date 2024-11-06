use database::Database;

mod database;

fn main() {
    let mut db: Database<String, String> = Database::new();

    println!("Color: {:?}", db.get("color")); // => None

    db.insert("color", "red");
    println!("Color: {:?}", db.get("color")); // => Some("red")

    db.delete("color");
    println!("Color: {:?}", db.get("color")); // => None

    db.insert("1", "value");
    db.insert("2", "john");
    db.save_to_file("dbfile.json").unwrap();

    let mut new_db: Database<u8, String> = Database::new();
    new_db.load_from_file("dbfile.json").unwrap();

    println!("New db value: {:?}", new_db.get(1)); // => Some("value")
    println!("New db user: {:?}", new_db.get(2)); // => Some("john")
}
