use futures::sink::Feed;

// fn three_vowels(word: &String) -> bool {
fn three_vowels(word: &str) -> bool {
  let mut vowle_count = 0;
  for c in word.chars() {
    match c {
      'a' | 'e' | 'i' | 'o' | 'u' => {
        vowle_count += 1;
        if vowle_count >= 3 {
          return true;
        }
      }

      _ => vowle_count = 0
    }
  }
  false
}

pub fn comm() {
  let ferris = "Ferris".to_string();
  let curious = "Curious".to_string();
  println!("{} -->>> {}", ferris, three_vowels(&ferris));
  println!("{} -->>> {}", curious, three_vowels(&curious));


  println!("{} -->>> {}", ferris, three_vowels("Ferris"));
  println!("{} -->>> {}", curious, three_vowels("Curious"));
}



