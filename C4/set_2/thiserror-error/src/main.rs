use thiserror::Error;

#[derive(Debug, Error)]
enum FileProcessingError {
    #[error("file counldn't be found, it likely doesn't exist")]
    FileNotFound,
    #[error("couldn't parse file, check your syntax")]
    ParseError,
    #[error("file with name `{0}` already exists")]
    AlreadyExists(String),
    #[error("permissions `{file_perms:?}` are insufficient")]
    Permissions { file_perms: (u8, u8, u8) },
}

fn main() {
    println!("{}", FileProcessingError::FileNotFound);
    println!("{}", FileProcessingError::ParseError);
    println!("{}", FileProcessingError::AlreadyExists("stats".into()));
    println!(
        "{}",
        FileProcessingError::Permissions {
            file_perms: (5, 4, 4)
        }
    );
}
