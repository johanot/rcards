use std::prelude::v1::{Vec, IntoIterator, Iterator};
use std::fmt;
use std::fmt::{Formatter, Error};
use std::default::Default;
use gfx_device_gl::Texture;
use std::rc::Rc;
use crate::graphic::GraphicsEnv;
use piston_window::texture::ImageSize;
use piston_window::G2dTexture;
use std::ops::{DerefMut, Deref};
use core::slice;

pub struct Deck(Vec<Card>);
pub struct Table(Vec<Deck>);

pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub table: Table,
    pub last_round: bool,
    pub graphics_env: Option<GraphicsEnv>,
}

pub struct Player
{
    pub name: String,
    pub hand: Deck
}

pub struct Card {
    pub suit: Suit,
    pub value: u8,
}

pub enum Suit {
    CLUBS,
    SPADES,
    DIAMONDS,
    HEARTS
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "name: {}, hand: {}", &self.name, &self.hand)
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in &self.0 {
            write!(f, "{}, ", &c)?;
        }
        Ok(())
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "deck: {}", &self.deck)?;
        for p in &self.players {
            write!(f, "player: {}", &p)?;
        }
        writeln!(f, "table: \n{}", &self.table)?;
        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Game{
            players: vec![],
            deck: Deck::build(),
            table: Table(vec![]),
            last_round: false,
            graphics_env: None::<GraphicsEnv>,
        }
    }
}


impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for p in &self.0 {
            writeln!(f, "- pile: {}", &p)?;
        }
        Ok(())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token = match &self.suit {
            Suit::CLUBS => "C",
            Suit::SPADES => "S",
            Suit::DIAMONDS => "D",
            Suit::HEARTS => "H",
        };
        write!(f, "{}:{}", token, self.value)
    }
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player{
            name: name.to_string(),
            hand: Deck::empty()
        }
    }
}

impl Deck {
    fn empty() -> Self {
        Deck(vec![])
    }

    pub fn singleton(card: Card) -> Self {
        Deck(vec![card])
    }

    fn build() -> Self {
        let mut deck = Vec::new();
        for value in 1..14 {
            deck.push(Card{
                suit: Suit::CLUBS,
                value
            })
        }
        for value in 1..14 {
            deck.push(Card{
                suit: Suit::SPADES,
                value
            })
        }
        for value in 1..14 {
            deck.push(Card{
                suit: Suit::DIAMONDS,
                value
            })
        }
        for value in 1..14 {
            deck.push(Card{
                suit: Suit::HEARTS,
                value
            })
        }

        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
        Deck(deck)
    }

    pub fn draw(&mut self, count: usize) -> Option<Vec<Card>> {
        match self.0.len() {
            l if count <= l => {
                let mut res = Vec::new();
                for _ in 0..count {
                    res.push(self.0.remove(0));
                }
                Some(res)
            },
            _ => None
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.has_cards(1)
    }

    pub fn has_cards(&self, count: usize) -> bool {
        self.0.len() >= count
    }

    fn append(&mut self, cards: &mut Vec<Card>) {
        self.0.append(cards);
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> IntoIterator for &'a Table {
    type Item = &'a Deck;
    type IntoIter = slice::Iter<'a, Deck>;

    fn into_iter(self) -> slice::Iter<'a, Deck> {
        let d: &'a Vec<Deck> = &self.0;
        d.into_iter()
    }
}

impl<'a> IntoIterator for &'a Deck {
    type Item = &'a Card;
    type IntoIter = slice::Iter<'a, Card>;

    fn into_iter(self) -> slice::Iter<'a, Card> {
        let d: &'a Vec<Card> = &self.0;
        d.into_iter()
    }
}

impl Table {
    pub fn new_pile(&mut self, cards: Vec<Card>) {
        self.0.push(Deck(cards));
    }
}

impl Player {
    pub fn deal(&mut self, cards: &mut Vec<Card>) {
        self.hand.append(cards);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::default::Default;

    #[test]
    fn test_deck_build() {
        let mut d = Deck::build();
        let draw = d.draw(4).unwrap();
    }

    #[test]
    fn test_game_creation() {
        let mut p1 = Player{
            name: "player1".to_string(),
            hand: Deck(vec![])
        };
        let mut p2 = Player{
            name: "player2".to_string(),
            hand: Deck(vec![])
        };
        let mut deck = Deck::build();

        let mut game = Game{
            players: vec![p1, p2],
            deck,
            table: Table(vec![]),
            ..Default::default()
        };

        println!();
        println!("Before setup:");
        println!();
        println!("{}", &game);
        game.setup();
        println!("After setup:");
        println!();
        println!("{}", &game);
    }

}
