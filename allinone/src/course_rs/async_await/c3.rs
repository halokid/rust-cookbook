use futures::executor::block_on;

struct Song {
  author: String,
  name: String,
}

async fn learn_song() -> Song {
  Song {
    author: "Jacky".to_string(),
    name: String::from("`Ju Hua Tai`"),
  }
}

async fn sing_song(song: Song) {
  println!("Song for everyone, it's {} song {}", song.author, song.name);
}

async fn dance() {
  println!("when singing, also want dance...");
}

async fn learn_and_sing() {
  let song = learn_song().await;

  sing_song(song).await;
}

pub async fn async_comm() {
  let f1 = learn_and_sing();
  let f2 = dance();

  futures::join!(f1, f2);
}

pub fn comm() {
  let song = block_on(learn_song());
  block_on(sing_song(song));
  block_on(dance());
}
