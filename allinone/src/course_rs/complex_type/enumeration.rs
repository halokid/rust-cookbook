

#[derive(Debug)]
enum PokerSuit {
  Clubs,
  Spades,
  Diamonds,
  Hearts,
}

struct PokerCard {
  suit:   PokerSuit,
  value:  u8,
}

pub fn comm() {
  let heart = PokerSuit::Hearts;
  let diamond = PokerSuit::Diamonds;

  print_suit(heart);
  print_suit(diamond);

  // --------------------------------------
  let c1 = PokerCard {
    suit: PokerSuit::Clubs,
    value: 1,
  };

  let c2 = PokerCard {
    suit: PokerSuit::Diamonds,
    value: 12,
  };
}

fn print_suit(card: PokerSuit) {
  println!("{:?}", card);
  dbg!(&card);
}




