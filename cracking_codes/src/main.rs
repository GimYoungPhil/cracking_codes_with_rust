// use chapter_00;
// use chapter_01;
// use chapter_02::caesar::Cipher;

fn main() {
    // let message = "This is my secret message.";

    // let cipher = Cipher::with_key(13);

    // let encoding = cipher.encrypt_message(message);
    // println!("{}", encoding);

    // let decoding = cipher.decrypt_message(&encoding);
    // println!("{}", decoding);

    // let coin = Coin::Pnney;
    // println!("{}", value_in_cents(coin));

    let card = Card::from_i8(Suit::Spade, 13);
    println!("{:?}", card);

    let card1 = Card::from_i8(Suit::Diamond, 23);
    println!("{:?}", card1);
}

// enum Coin {
//     Pnney = 1,
//     Niccle = 5,
//     Dime = 10,
//     Quarter = 25,
// }

// fn value_in_cents(coin: Coin) -> i32 {
//     coin as i32
// }

// type MyNumber = 1;

type SuitNumber = u8;

#[derive(Debug)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(Debug)]
struct Card(Suit, SuitNumber);

impl Card {
    fn from_i8(suit: Suit, number: SuitNumber) -> Card {
        match number {
            1..=13 => Card(suit, number),
            _ => (),
        }
    }
}
