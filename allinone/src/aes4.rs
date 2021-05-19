
extern crate crypto;
extern crate rand;

use crypto::{ symmetriccipher, buffer, aes, blockmodes };
// use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

use rand::{ Rng, OsRng };

pub fn aes_cbc_mode(){
    let message = "Hello World!";

    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];

    // In a real program, the key and iv may be determined
    // using some other mechanism. If a password is to be used
    // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
    // supported by Rust-Crypto!) would be a good choice to derive
    // a password. For the purposes of this example, the key and
    // iv are just random values.
    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);

    let encrypted_data = encrypt(message.as_bytes(), &key, &iv).ok().unwrap();
    let decrypted_data = decrypt(&encrypted_data[..], &key, &iv).ok().unwrap();

    assert!(message.as_bytes() == &decrypted_data[..]);
}


