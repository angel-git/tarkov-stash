use chrono::Utc;
use ring::digest;
use std::fmt::Write;

fn get_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn generate() -> String {
    let time = rand::random::<f32>() * get_timestamp() as f32;

    let digest = digest::digest(&digest::SHA256, time.to_string().as_bytes());

    let result: String = digest.as_ref().iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02x}");
        output
    });

    result.chars().take(24).collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::hash_utils::generate;

    #[test]
    fn should_create_a_new_hash() {
        println!("{}", generate().as_str());
        assert_eq!(generate().len(), 24);
    }
}
