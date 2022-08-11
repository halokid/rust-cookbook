

#[derive(Debug)]
enum PokerCard {
  Clubs(u8),
  Spades(u8),
  Diamonds(u8),
  Hearts(u8),
}

enum Message {
  Quit,
  Move{ x: i32, y: i32},
  Write(String),
  ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
  x:  i32,
  y:  i32,
}
struct WriteMessage(String);
struct ChangeColor(i32, i32, i32);

pub fn comm() {
  let c1 = PokerCard::Spades(5);
  let c2 = PokerCard::Diamonds(13);

  // ---------------------------------------
  let m1 = Message::Quit;
  let m2 = Message::Move {x: 1, y: 2};
  let m3 = Message::ChangeColor(255, 255, 0);
}




