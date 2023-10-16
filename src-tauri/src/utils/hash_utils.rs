use chrono::Utc;
use ring::digest;

fn get_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn generate() -> String {
    let time = rand::random::<f32>() * get_timestamp() as f32;

    let digest = digest::digest(&digest::SHA256, time.to_string().as_bytes());

    let result: String = digest
        .as_ref()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();

    result.chars().take(24).collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::hash_utils::generate;

    #[test]
    fn should_create_a_new_hash() {
        assert_eq!(generate().len(), 24);
    }
}
