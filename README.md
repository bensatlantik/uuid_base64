## uuid_base64

# UUID Base64 Converter

This library provides a simple function to convert a UUID to a Base64-encoded string. This can help shorten the UUID for display or storage purposes.

## Installation

Make sure you have Rust installed. Then, create a new project or clone this repository and add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
uuid = { version = "1.0", features = ["v4"] }
base64 = "0.13"
```
## Usage
Here's an example of how to use the uuid_to_base64 function:

```rust
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
        let uuid = Uuid::new_v4();
        let encoded = uuid_to_base64(&uuid);
        println!("UUID: {}", uuid);
        println!("Base64: {}", encoded);
        // Add assertions as needed
    }
}
```

## Testing

cargo test

## License
This project is licensed under the MIT License.


