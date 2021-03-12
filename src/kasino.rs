use crate::types::{Game, Player, Deck, PlayerInteraction};
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
        self.player_turn = Some(0);
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

    fn current_player<'p>(&'p self) -> Option<&'p Player> {
        self.player_turn.and_then(|number| self.players.get(number as usize))
    }

    pub fn try_to_intent(&self) -> Result<(), IntentError> {
        match self.interactions.len() {
            1 => {
                self.interactions.first().as_ref().ok_or(IntentError::Unknown).and_then(|i| {
                    match i {
                        PlayerInteraction::Click(sprite_ref) => {
                            if self.current_player().unwrap().hand.contains(&sprite_ref.get_info().card) {
                                Err(IntentError::PartialIntent("(d) to drop"))
                            } else {
                                Ok(())
                            }
                        },
                        _ => { Ok(()) }
                    }
                })
            },
            2 => {
                self.interactions.last().as_ref().ok_or(IntentError::Unknown).and_then(|i| {
                    match i {
                        PlayerInteraction::Click(sprite_ref) => {
                            if self.current_player().unwrap().hand.contains(&sprite_ref.get_info().card) {
                                Err(IntentError::PartialIntent("(d) to drop"))
                            } else {
                                Ok(())
                            }
                        },
                        _ => { Ok(()) }
                    }
                }) 
            }
            _ => {
                self.interactions.clear();
                Err(IntentError::IllegalAction("yet unknown action"))
            }


            /*self.interactions.first().as_ref().ok_or(IntentError::Unknown).and_then(|i| {
                if self.table.contains(&i.sprite.get_info().card) {
                    Err(IntentError::PartialIntent("(t) to take"))
                } else {
                    Ok(())
                }
            })*/
        } //else {

            //
        //}
    }
}

pub enum IntentError {
    PartialIntent(&'static str),
    IllegalAction(&'static str),
    Unknown
}

pub enum KasinoError {
    OtherPlayersCards,
    DeckOrPileEmpty
}
