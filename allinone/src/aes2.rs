/*
extern crate hex;
extern crate openssl;
extern crate otpauth;
extern crate time;

use openssl::aes::{AesKey, aes_ige};
use openssl::symm::Mode;
use otpauth::totp::TOTP;
use std::vec::Vec;

pub struct Crypto {
    key: Vec<u8>,
    iv: Vec<u8>,
    time_key: Vec<u8>,
}

pub trait CryptoService {
    fn new(key: Vec<u8>, iv: Vec<u8>, time_key: Vec<u8>) -> Self;
    fn encrypt(&self, msg: &Vec<u8>) -> Vec<u8>;
    fn decrypt(&self, encrypted: &Vec<u8>) -> Vec<u8>;
    fn encrypt_time_based(&self, msg: &Vec<u8>) -> Vec<u8>;
    fn decrypt_time_based(&self, encrypted: &Vec<u8>) -> Vec<u8>;
    fn encrypt_internal(&self, msg: &Vec<u8>, key: &Vec<u8>) -> Vec<u8>;
    fn decrypt_internal(&self, encrypted: &Vec<u8>, key: &Vec<u8>) -> Vec<u8>;
    fn create_key(&self) -> Vec<u8>;
    fn create_token(&self) -> Vec<u8>;
    fn xor(&self, one: Vec<u8>, two: Vec<u8>) -> Vec<u8>;
}

impl CryptoService for Crypto {
    fn new(key: Vec<u8>, iv: Vec<u8>, time_key: Vec<u8>) -> Self {
        if key.len() != 16 && key.len() != 24 && key.len() != 32 {
            panic!("Error creating AES key (must be 128, 192, or 256 bits)");
        }
        if iv.len() % 2 != 0 || iv.len() < 4 {
            panic!("Error creating IV (must be at least 32 bits and multiple of 16)");
        }
        if time_key.len() < 4 {
            panic!("Error creating TOTP key (must be at least 32 bits)");
        }
        Crypto {
            key,
            iv,
            time_key,
        }
    }

    fn encrypt(&self, msg: &Vec<u8>) -> Vec<u8> {
        let vec_encrypt = CryptoService::encrypt_internal(self, msg, &self.key);
        vec_encrypt
    }

    fn decrypt(&self, encrypted: &Vec<u8>) -> Vec<u8> {
        let msg = CryptoService::decrypt_internal(self, encrypted, &self.key);
        msg
    }

    fn encrypt_time_based(&self, msg: &Vec<u8>) -> Vec<u8> {
        let new_key = CryptoService::create_key(self);
        let encrypted = CryptoService::encrypt_internal(self, msg, &new_key);
        encrypted
    }

    fn decrypt_time_based(&self, encrypted: &Vec<u8>) -> Vec<u8> {
        let new_key = CryptoService::create_key(self);
        let msg = CryptoService::decrypt_internal(self, encrypted, &new_key);
        msg
    }

    fn encrypt_internal(&self, msg: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
        if msg.len() % 16 != 0 {
            panic!("Message vec size must be multiple of 16 current {}", msg.len());
        }
        let encrypt_key = AesKey::new_encrypt(key).unwrap();
        let mut vec_encrypt = vec![0; msg.len()];
        let mut vec_iv = self.iv.to_vec();
        aes_ige(&msg, &mut vec_encrypt, &encrypt_key, &mut vec_iv, Mode::Encrypt);
        vec_encrypt
    }

    fn decrypt_internal(&self, encrypted: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
        if encrypted.len() % 16 != 0 {
            panic!("Encrypted vec size must be multiple of 16 current {}", encrypted.len());
        }
        let decrypt_key = AesKey::new_decrypt(&key).unwrap();
        let mut msg = vec![0; encrypted.len()];
        let mut vec_iv = self.iv.to_vec();
        aes_ige(&encrypted, &mut msg, &decrypt_key, &mut vec_iv, Mode::Decrypt);
        msg
    }

    fn create_key(&self) -> Vec<u8> {
        let token = CryptoService::create_token(self);
        let new_key = CryptoService::xor(self,self.key.to_vec(), token.to_vec());
        new_key
    }

    fn create_token(&self) -> Vec<u8> {
        let key = String::from_utf8_lossy(&self.time_key);
        let auth = TOTP::new(key);
        let now = time::now().to_timespec().sec as usize;
        let code = auth.generate(30, now);
        let string = code.to_string();
        let bytes = string.as_bytes();
        Vec::from(bytes)
    }

    fn xor(&self, one: Vec<u8>, two: Vec<u8>) -> Vec<u8> {
        if one.len() >= two.len() {
            let mut one_copy = one;
            for i in 0..two.len() {
                one_copy[i] ^= two[i];
            }
            one_copy
        } else {
            let mut two_copy = two;
            for i in 0..one.len() {
                two_copy[i] ^= one[i];
            }
            two_copy
        }
    }
}


 fn create_cypto() -> Crypto {
        let key = "12345678901234567890123456789012";
        let vec_key = Vec::from(key.as_bytes());
        let iv = "21098765432109876543210987654321";
        let vec_iv = Vec::from(iv.as_bytes());
        let time_key = "00010203040506070809";
        let vec_time_key = Vec::from(time_key.as_bytes());
        let crypto = Crypto::new(vec_key, vec_iv, vec_time_key);
        println!("Crypto created");
        crypto
 }

 pub fn comm() {
   let msg_bytes = "Time based tests rust. Secret.!!".as_bytes();
   let msg = Vec::from(msg_bytes);

   let crypto = create_cypto();

   let encrypted = CryptoService::encrypt_time_based(&crypto, &msg);
   let original = CryptoService::decrypt_time_based(&crypto, &encrypted);

   println!("Original {}", String::from_utf8_lossy(&original));
 }


 */

