use std::{
    env::args,
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
    process::exit,
};

fn file_read(source_file: &str) -> String {
    let mut reader = BufReader::new(File::open(source_file).expect("error opening file"));

    let mut content = String::new();

    reader
        .read_to_string(&mut content)
        .expect("error reading from file");

    content
}

fn file_write(content: &str, source_file: &str, maybe_out_file: Option<&String>) {
    let write_file = match maybe_out_file {
        Some(name) => match File::open(name) {
            Ok(file) => file,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                File::create(name).expect("error creating out_file")
            }
            _ => panic!("error reading out_file"),
        },
        None => File::create(source_file).expect("error opening file"),
    };

    let mut writer = BufWriter::new(write_file);

    for line in content.lines() {
        writeln!(&mut writer, "{}", line).expect("error writing to file");
    }
}

fn main() {
    let args = args().collect::<Vec<_>>();
    if args.len() != 4 && args.len() != 5 {
        println!("invalid arguments: pass in 1:file 2:text_from 3:text_to [4:out_file]");
        exit(1);
    }

    let (source_file, from, to, maybe_out_file) = (&args[1], &args[2], &args[3], args.get(4));

    let mut content = file_read(source_file);

    println!("\nbefore:\n{}", content);
    content = content.replace(from, to);
    println!("after:\n{}", content);

    file_write(&content, source_file, maybe_out_file);
}
