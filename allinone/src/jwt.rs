use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize, // Expiration time as a UNIX timestamp (seconds since the UNIX epoch)
}

fn main() {
    // Your secret key
    let secret = "your_secret_key";

    // Set the expiration time (e.g., 1 hour from the current time)
    let expiration_time = SystemTime::now()
        .checked_add(std::time::Duration::from_secs(3600))
        .expect("Overflow");

    // Convert expiration time to a UNIX timestamp
    let exp = expiration_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Create JWT claims
    let claims = Claims {
        sub: "user123".into(),
        exp: exp as usize,
    };

    // Encode JWT token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    println!("JWT Token: {}", token);

    // Decode and verify the JWT token
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            println!("Decoded Token: {:?}", token_data.claims);
        }
        Err(err) => {
            println!("Error decoding token: {:?}", err);
        }
    }
}
