use rand::{Rng, StdRng};
use std::io;
use std::iter::FromIterator;

#[derive(Copy, Clone, Debug)]
enum Suit {
    Diamond,
    Heart,
    Club,
    Spade,
}

#[derive(Clone, Debug)]
struct Card {
    suit: u8,
    rank: u8,
    symbol: String,
    suit_symbol: String,
}

impl Card {
    fn init(rank: u8, suit: u8) -> Card {
        let symbol = match rank {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            _ => rank.to_string(),
        };
        let suit_symbol = match suit {
            1 => "♡".to_string(),
            2 => "♢".to_string(),
            3 => "♣".to_string(),
            4 => "♠".to_string(),
            _ => panic!(),
        };
        Card {
            suit,
            rank,
            symbol,
            suit_symbol,
        }
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn init() -> Deck {
        let mut vc = Vec::new();
        let mut rank = 1;
        let mut suit = 1;
        //let mut suit = Suit::Heart;
        while suit < 5 {
            while rank < 14 {
                //let card = Card::init(rank: u8, suit: u8)
                vc.push(Card::init(rank, suit));
                rank = rank + 1;
            }
            rank = 1;
            suit = suit + 1;
        }

        let mut rng = StdRng::new().unwrap();
        rng.shuffle(&mut vc);
        Deck { cards: vc }
    }
}

fn deal_cards(deck: &mut Deck, nbr: usize) -> Vec<Card> {
    let mut hand: Vec<Card> = Vec::new();
    let mut i = 1;
    while i <= nbr {
        let c = Some(deck.cards.pop());
        match c {
            Some(c) => hand.push(c.unwrap()),
            None => panic!(),
        }

        i = i + 1;
    }
    hand
}

fn print_hand(hand: &Vec<Card>) {
    for card in hand.iter() {
        print!("{}{} ", card.suit_symbol, card.symbol);
    }
    println!("");
}

fn is_two_pairs(hand: &Vec<Card>) -> bool {
    if is_four_of_kind(hand) {
        return false;
    }
    if (hand[0].rank == hand[1].rank) && (hand[2].rank == hand[3].rank) {
        return true;
    } else if (hand[0].rank == hand[1].rank) && (hand[3].rank == hand[4].rank) {
        return true;
    } else if (hand[1].rank == hand[2].rank) && (hand[3].rank == hand[4].rank) {
        return true;
    }
    false
}

fn is_three_of_kind(hand: &Vec<Card>) -> bool {
    if (hand[0].rank == hand[2].rank)
        || (hand[1].rank == hand[3].rank)
        || (hand[2].rank == hand[4].rank)
    {
        return true;
    }
    false
}

fn is_four_of_kind(hand: &Vec<Card>) -> bool {
    if (hand[0].rank == hand[3].rank) || (hand[1].rank == hand[4].rank) {
        return true;
    }
    false
}

fn is_flush(hand: &Vec<Card>) -> bool {
    (hand[0].suit == hand[1].suit)
        && (hand[0].suit == hand[2].suit)
        && (hand[0].suit == hand[3].suit)
        && (hand[0].suit == hand[4].suit)
}

fn check_hand(hand: &Vec<Card>) -> String {
    let mut reply = String::from("no win");
    if is_flush(hand) {
        reply = String::from("flush!");
    } else if is_two_pairs(hand) {
        reply = String::from("two pairs");
    } else if is_three_of_kind(hand) {
        reply = String::from("three of kind");
    }
    reply
}

fn check_full_house(hand: &Vec<Card>) {}

fn main() {
    let mut deck = Deck::init();
    let mut hand = deal_cards(&mut deck, 5);
    print_hand(&hand);
    println!("Input cards you want to keep.");
    let mut selection = String::new();
    io::stdin().read_line(&mut selection).expect("failed.");
    let selection = selection.trim();
    let mut new_hand: Vec<Card> = Vec::new();
    for c in selection.chars() {
        // let mut i: u32 = c.to_digit(10).unwrap();
        match c {
            '1' => new_hand.push(Card::init(hand[0].rank, hand[0].suit)),
            '2' => new_hand.push(Card::init(hand[1].rank, hand[1].suit)),
            '3' => new_hand.push(Card::init(hand[2].rank, hand[2].suit)),
            '4' => new_hand.push(Card::init(hand[3].rank, hand[3].suit)),
            '5' => new_hand.push(Card::init(hand[4].rank, hand[4].suit)),
            _ => panic!(),
        }
    }
    let new_cards = deal_cards(&mut deck, 5 - selection.len());
    for c in new_cards {
        &new_hand.push(c);
    }
    new_hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    print_hand(&new_hand);
    println!("{}", check_hand(&new_hand));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal_cards() {
        let mut deck = Deck::init();
        let hand = deal_cards(&mut deck, 5);
        assert_eq!(5, hand.len());
    }

    #[test]
    fn test_check_hand() {
        let cards = vec![
            Card::init(1, 1),
            Card::init(1, 2),
            Card::init(1, 3),
            Card::init(1, 4),
            Card::init(2, 1),
        ];
        assert_eq!(is_four_of_kind(&cards), true);
        let cards = vec![
            Card::init(1, 1),
            Card::init(2, 2),
            Card::init(2, 3),
            Card::init(2, 4),
            Card::init(2, 1),
        ];
        assert_eq!(is_four_of_kind(&cards), true);
    }
}
