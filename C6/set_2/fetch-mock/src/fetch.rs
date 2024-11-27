use std::{thread, time::Duration};

pub fn fetch_data(id: u32) -> Result<String, String> {
    thread::sleep(Duration::from_secs(3));
    if id < 99 {
        Ok(format!("USER: {}", id))
    } else {
        Err(format!("user with the id `{}` does not exist", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_users() {
        assert_eq!(fetch_data(88), Ok("USER: 88".into()));
    }

    #[test]
    fn invalid_user() {
        assert_eq!(
            fetch_data(113),
            Err("user with the id `113` does not exist".into())
        );
    }
}
