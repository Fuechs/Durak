use rand::prelude::*;

#[allow(dead_code)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades
}

impl Suit {
    fn as_str(&self) -> &str {
        match self {
            Suit::Clubs     => "Clubs",
            Suit::Diamonds  => "Diamonds",
            Suit::Hearts    => "Hearts",
            Suit::Spades    => "Spades"
        } 
    }

    fn as_symbol(&self) -> &str {
        match self {
            Suit::Clubs     => "♣",
            Suit::Diamonds  => "♦",
            Suit::Hearts    => "♥",
            Suit::Spades    => "♠"
        }
    }
}

struct Card {
    value: u8,
    suit: Suit
}

impl Card {
    fn new(value: u8, suit: Suit) -> Self {
        Self { value, suit }
    }

    fn print(&self) {
        println!("Card: {0} of {1}", self.value, self.suit.as_str());
    }
}

struct Deck {
    cards: Vec<Card>
}

impl Deck {
    fn new() -> Self {
        let mut vec = Vec::new();
        
        for i in 1..11 {
            vec.push(Card::new(i, Suit::Clubs));
            vec.push(Card::new(i, Suit::Diamonds));
            vec.push(Card::new(i, Suit::Hearts));
            vec.push(Card::new(i, Suit::Spades));
        }

        Self { cards: vec }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng)
    }
}

#[allow(dead_code)]
struct Hand {
    deck: Deck,
    cards: Vec<Card>
}

#[allow(dead_code)]
impl Hand {
    fn new(deck: Deck) -> Self {
        Self {deck: deck, cards: vec![]}
    }

    fn draw(&mut self) {
        while self.cards.len() < 6 {
            self.cards.push(self.deck.cards.pop().expect("Deck is empty"))
        }
    }

    fn play(&mut self, index: usize) -> Card {
        self.cards.remove(index)
    }

    fn print(&self) {
        print!("Hand:");
        for i in &self.cards {
            print!(" {0} {1} |", i.value, i.suit.as_symbol());
        }
        println!();
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    
    let mut hand = Hand::new(deck);
    hand.draw();
    hand.print();
    hand.play(5);
    hand.print();
    hand.draw();
    hand.print();
}