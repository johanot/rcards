use crate::types::{Game, Player, Deck};
use crate::graphic::GraphicsEnv;
use std::default::Default;
use piston::input::UpdateEvent;
use piston_window::texture::ImageSize;
use std::collections::HashMap;
use uuid::Uuid;

impl Game {

    pub fn new(graphics_env: GraphicsEnv, players: Vec<Player>) -> Game {
        Game{
            players,
            graphics_env: Some(graphics_env),
            ..Default::default()
        }
    }

    pub fn start(&mut self) {
        self.setup();
    }

    fn setup(&mut self) {
        self.deal_each_player(2);
        self.deal_table(2);
        self.deal_each_player(2);
        self.deal_table(2);
    }

    fn end_of_round(&mut self) {
        if !self.last_round {
            self.deal_each_player(2);
            self.deal_each_player(2);
            if self.deck.is_empty() {
                self.last_round = true;
            }
        } else {
            self.end_of_game();
        }
    }

    fn end_of_game(&self) {
        //TODO: count points etc.
    }

    fn deal_each_player(&mut self, count: usize) -> Result<(), KasinoError> {
        if self.deck.has_cards(count*self.players.len()) {
            for p in &mut self.players {
                p.deal(&mut self.deck.draw(count).ok_or(KasinoError::DeckOrPileEmpty)?);
            }
            Ok(())
        } else {
            Err(KasinoError::DeckOrPileEmpty)
        }
    }

    fn deal_table(&mut self, count: usize) -> Result<(), KasinoError> {
        if self.deck.has_cards(count) {
            for _ in 0..count {
                self.table.new_pile(self.deck.draw(1).ok_or(KasinoError::DeckOrPileEmpty)?);
            }
            Ok(())
        } else {
            Err(KasinoError::DeckOrPileEmpty)
        }
    }
}

pub enum KasinoError {
    DeckOrPileEmpty
}
