use std::prelude::v1::{Vec, IntoIterator, Iterator};
use std::fmt;
use std::fmt::{Formatter, Error};
use std::default::Default;
use gfx_device_gl::Texture;
use std::rc::Rc;
use crate::graphic::{GraphicsEnv, SpriteRef};
use piston_window::texture::ImageSize;
use piston_window::G2dTexture;
use std::ops::{DerefMut, Deref};
use core::slice;
use uuid::{Uuid, UuidVersion};
use std::collections::HashMap;
use std::sync::{RwLock, Arc, RwLockReadGuard, RwLockWriteGuard};
use std::option::Option;
use sprite::Sprite;


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct DeckRef(Uuid);

#[derive(Debug)]
pub struct Table(Vec<DeckRef>);

#[derive(Debug)]
pub struct Deck {
    id: DeckRef,
    cards: Vec<Card>,
}

pub struct Game {
    pub players: Vec<Player>,
    pub deck: DeckRef,
    pub table: Table,
    pub last_round: bool,
    pub graphics_env: Option<GraphicsEnv>,
    pub player_turn: Option<u8>,
}

#[derive(Debug)]
pub struct Player {
    pub id: u8,
    pub name: String,
    pub hand: DeckRef,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: Suit,
    pub value: u8,
    pub sprite: Option<SpriteRef>,
}

#[derive(Debug, Clone)]
pub enum Suit {
    CLUBS,
    SPADES,
    DIAMONDS,
    HEARTS
}

lazy_static! {
    static ref DECKS: RwLock<HashMap<DeckRef, Deck>> = RwLock::new(HashMap::new());
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "name: {}, hand: {}", &self.name, &self.hand)
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in &self.cards {
            write!(f, "{}, ", &c)?;
        }
        Ok(())
    }
}

impl fmt::Display for DeckRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DECKS.read().unwrap().get(&self).unwrap().fmt(f)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "deck: {}", &self.deck)?;
        for p in &self.players {
            write!(f, "player: {}", &p)?;
        }
        //writeln!(f, "table: \n{}", &self.table)?;
        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Game{
            players: vec!(),
            deck: Deck::build(),
            table: Table(vec!()),
            last_round: false,
            graphics_env: None::<GraphicsEnv>,
            player_turn: None
        }
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

impl DeckRef {
    pub fn is_empty(&self) -> bool {
        DECKS.read().unwrap().get(&self).unwrap().is_empty()
    }

    pub fn has_cards(&self, count: usize) -> bool {
        DECKS.read().unwrap().get(&self).unwrap().has_cards(count)
    }

    fn append(&mut self, cards: &mut Vec<Card>) {
        DECKS.write().unwrap().get_mut(&self).unwrap().append(cards)
    }

    pub fn draw(&mut self, count: usize) -> Option<Vec<Card>> {
        DECKS.write().unwrap().get_mut(&self).unwrap().draw(count)
    }

    fn len(&self) -> usize {
        DECKS.read().unwrap().get(&self).unwrap().len()
    }

    pub fn iter(&mut self) -> DeckRefIter {
        DeckRefIter {
            guard: DECKS.write().unwrap(),
            deck_ref: self.to_owned()
        }
    }
}

impl Player {
    pub fn new(id: u8, name: &str) -> Player {
        Player{
            id,
            name: name.to_string(),
            hand: Deck::empty(),
        }
    }
}

impl Deck {
    pub fn empty() -> DeckRef {
        Self::new(vec![])
    }

    pub fn new(cards: Vec<Card>) -> DeckRef {
        let deck_ref = DeckRef(Uuid::new(UuidVersion::Random).unwrap());
        DECKS.write().unwrap().insert(deck_ref, Deck {
            id: deck_ref,
            cards
        });
        deck_ref
    }

    pub fn singleton(card: Card) -> DeckRef {
        Self::new(vec![card])
    }

    fn build() -> DeckRef {
        let mut cards = Vec::new();
        for value in 1..14 {
            cards.push(Card::new(Suit::CLUBS, value));
        }
        for value in 1..14 {
            cards.push(Card::new(Suit::SPADES, value));
        }
        for value in 1..14 {
            cards.push(Card::new(Suit::DIAMONDS, value));
        }
        for value in 1..14 {
            cards.push(Card::new(Suit::HEARTS, value));
        }

        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        Deck::new(cards)
    }

    pub fn draw(&mut self, count: usize) -> Option<Vec<Card>> {
        match self.cards.len() {
            l if count <= l => {
                let mut res = Vec::new();
                for _ in 0..count {
                    res.push(self.cards.remove(0));
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
        self.cards.len() >= count
    }

    fn append(&mut self, cards: &mut Vec<Card>) {
        self.cards.append(cards);
    }

    fn len(&self) -> usize {
        self.cards.len()
    }
}

pub struct DeckRefIter {
    guard: RwLockWriteGuard<'static, HashMap<DeckRef, Deck>>,
    deck_ref: DeckRef,
}

impl<'a> IntoIterator for &'a mut DeckRefIter {
    type Item = &'a mut Card;
    type IntoIter = ::std::slice::IterMut<'a, Card>;

    fn into_iter(self) -> ::std::slice::IterMut<'a, Card> {
        self.guard.get_mut(&self.deck_ref).unwrap().cards.iter_mut()
    }
}


impl<'a> IntoIterator for &'a mut Table {
    type Item = &'a mut DeckRef;
    type IntoIter = slice::IterMut<'a, DeckRef>;

    fn into_iter(self) -> slice::IterMut<'a, DeckRef> {
        self.0.iter_mut()
    }
}

impl<'a> IntoIterator for &'a Deck {
    type Item = &'a Card;
    type IntoIter = slice::Iter<'a, Card>;

    fn into_iter(self) -> slice::Iter<'a, Card> {
        let d: &'a Vec<Card> = &self.cards;
        d.into_iter()
    }
}

/*
impl<'a> IntoIterator for &'a DeckRef {
    type Item = &'a Card;
    type IntoIter = slice::Iter<'a, Card>;

    fn into_iter(self) -> slice::Iter<'a, Card> {
        let deck: Option<&Deck> = DECKS.read().unwrap().get(&self.0);
        let d: &'a Vec<Card> = &deck.unwrap().cards;
        d.into_iter()
    }
}
*/

impl Table {
    pub fn new_pile(&mut self, cards: Vec<Card>) {
        self.0.push(Deck::new(cards));
    }
}

impl Player {
    pub fn deal(&mut self, cards: &mut Vec<Card>) {
        self.hand.append(cards);
    }
}

impl Card {
    pub fn new(suit: Suit, value: u8) -> Card {
        Card{
            suit,
            value,
            sprite: None
        }
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
            id: 1,
            name: "player1".to_string(),
            hand: Deck(vec![])
        };
        let mut p2 = Player{
            id: 2,
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
