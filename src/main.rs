use rand::{Rng, StdRng};
use std::io;

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

fn is_straight(hand: &Vec<Card>) -> bool {
    let mut reply: bool = false;
    if (hand[0].rank + 1 == hand[1].rank)
        && (hand[1].rank + 1 == hand[2].rank)
        && (hand[2].rank + 1 == hand[3].rank)
        && (hand[3].rank + 1 == hand[4].rank)
    {
        reply = true;
    }
    if (hand[0].rank == 1 && hand[4].rank == 13)
        && (hand[1].rank == 10)
        && hand[2].rank == 11
        && hand[3].rank == 12
    {
        reply = true;
    }
    reply
}

fn is_fullhouse(_hand: &Vec<Card>) -> bool {
    false
}

fn is_royal_flush(_hand: &Vec<Card>) -> bool {
    false
}

fn check_hand(hand: &Vec<Card>, bet: i32) -> i32 {
    let mut win = -1;
    if is_two_pairs(hand) {
        win = bet * 2;
        println!("Two pairs! Win {}", win);
    } else if is_three_of_kind(hand) {
        win = bet * 3;
        println!("Three of a kind! Win {}", win);
    } else if is_straight(hand) {
        win = bet * 4;
        println!("Straight! Win {}", win);
    } else if is_flush(hand) {
        win = bet * 5;
        println!("Flush! Win {}", win);
    } else if is_fullhouse(hand) {
        win = bet * 8;
        println!("Full house! Win {}", win);
    } else if is_four_of_kind(hand) {
        win = bet * 25;
        println!("Four of a kind! Win {}", win);
    } else if is_royal_flush(hand) {
        win = bet * 50;
        println!("Royal flush! Win {}", win);
    } else {
        println!("No win, sorry!");
    }
    win
}

fn main() {
    let mut bank = 20;
    while bank > 0 {
        println!("Bank: {}", bank);
        let mut deck = Deck::init();
        let hand = deal_cards(&mut deck, 5);
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
        bank = bank + check_hand(&new_hand, 1);
    }
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
        let cards = vec![
            Card::init(3, 1),
            Card::init(4, 2),
            Card::init(5, 3),
            Card::init(6, 4),
            Card::init(7, 1),
        ];
        assert_eq!(is_straight(&cards), true);
        let cards = vec![
            Card::init(1, 1),
            Card::init(10, 2),
            Card::init(11, 3),
            Card::init(12, 4),
            Card::init(13, 1),
        ];
        assert_eq!(is_straight(&cards), true);
    }
}
