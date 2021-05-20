/*
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use allinone::types::string;


pub fn comm() {
  // create an alias for convenience
  type Aes128Cbc = Cbc<Aes128, Pkcs7>;

  let key = hex!("000102030405060708090a0b0c0d0e0f");
  let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
  let plaintext = b"Hello world!";
  // let plaintext = "Hello world!";
  let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();

// buffer must have enough space for message+padding
  let mut buffer = [0u8; 32];
// copy message to the buffer
  let pos = plaintext.len();
  buffer[..pos].copy_from_slice(plaintext);
  let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

  // let b: Vec<u8> = ciphertext.iter().clone().collect();
  // todo: &[u8] 转为 Vec[u8]
  let b = ciphertext.to_vec();

  // todo: 直接输出 &[u8]
  println!("ciphertext --------------- {:?}", ciphertext);
  // todo: &[u8] 转为 string
  let s = String::from_utf8_lossy(ciphertext);


  // todo: Vec[u8] 转为 string
  // let sx = String::from_utf8(b);
  // println!("sx ------------------- {}", sx.unwrap());

  println!("ciphertext to string --------------- {:?}", s.to_string());
  assert_eq!(ciphertext, hex!("1b7a4c403124ae2fb52bedc534d82fa8"));

// re-create cipher mode instance
  let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
  let mut buf = ciphertext.to_vec();
  let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();

  println!("decrypted_ciphertext --------------- {:?}", decrypted_ciphertext);
  assert_eq!(decrypted_ciphertext, plaintext);

  // -----------------------------------------------------------
  let mut msg = String::from("hello");
  unsafe {
    let mut data = &mut msg.as_bytes_mut();
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let mut buffer = [0u8; 32];
    // copy message to the buffer
    let pos = msg.len();
    buffer[..pos].copy_from_slice(msg.as_bytes());
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();
    println!("ciphertest 1 &[u8] --------------- {:?}", ciphertext);

    let ciph_vecu8 = ciphertext.to_vec();
    println!("ciphertest 2 Vec[u8] --------------- {:?}", ciph_vecu8);

    let ciph_str = String::from_utf8(ciph_vecu8).unwrap();
    println!("ciphertest 3 Vec[u8] to string --------------- {:?}", ciph_str);
  }
}


 */
