extern crate uuid;
extern crate base64;

use uuid::Uuid;
use base64::encode;

pub fn uuid_to_base64(uuid: &Uuid) -> String {
    encode(uuid.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_uuid_to_base64() {
        let uuid = Uuid::new_v4();  // Generating a new UUID using the correct method
        let encoded = uuid_to_base64(&uuid);
        println!("UUID: {}", uuid);
        println!("Base64: {}", encoded);
        // Add assertions as needed
    }
}
