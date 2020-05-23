use crate::types::{Game, Card, Suit, Player, Deck};
use sprite::{Sprite, Scene};
use sdl2_window::Sdl2Window;
use std::rc::Rc;
use std::path::Path;
use std::convert::Into;
use std::collections::HashMap;
use graphics::rectangle::rectangle_by_corners;
use opengl_graphics::{GlGraphics, Texture};
use std::borrow::ToOwned;
use piston::input::{UpdateArgs, RenderArgs};
use piston_window::TextureSettings;
use std::iter::IntoIterator;
use piston::window::Size;
use uuid::Uuid;
use std::cell::RefCell;
use std::sync::{Arc, RwLock};


lazy_static! {
    static ref SPRITES: RwLock<HashMap<SpriteRef, SpriteInfo>> = RwLock::new(HashMap::new());
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SpriteRef(Uuid);

impl SpriteRef {
    pub fn new(id: Uuid, sprite_info: SpriteInfo) -> Self {
        let sprite_ref = SpriteRef(id);
        SPRITES.write().unwrap().insert(sprite_ref, sprite_info);
        sprite_ref
    }

    /*pub fn get_info<'l>(&self) -> &'l SpriteInfo {
        SPRITES.read().unwrap().get(&self).unwrap()
    }*/
}

impl std::convert::From<&Uuid> for SpriteRef {
    fn from(id: &Uuid) -> Self {
        SpriteRef(id.to_owned())
    }
}

pub struct GraphicsEnv {
    textures: HashMap<TextureKind, Rc<Texture>>,
    gl: GlGraphics,
}

#[derive(Eq, PartialEq, Hash)]
pub enum TextureKind {
    BACK,
    CARDS
}

#[derive(Debug, Clone)]
pub struct SpriteInfo {
    card: Card,
}

impl GraphicsEnv {
    pub fn new(gl: GlGraphics) -> Self {
        // load textures
        let mut textures = HashMap::new();
        let back = load_texture(&Path::new("assets/back-sized.png"));
        let front = load_texture(&Path::new("assets/cards_deck-half.png"));

        textures.insert(TextureKind::BACK, back.clone());
        textures.insert(TextureKind::CARDS, front.clone());

        GraphicsEnv {
            gl, textures,
        }
    }

    pub fn get_texture(&self, kind: TextureKind) -> Rc<Texture> {
        self.textures.get(&kind).unwrap().to_owned()
    }
}

impl Game {
    pub fn prepare(&mut self, scene: &mut Scene<Texture>) {
        let back = {
            self.graphics_env.as_ref().unwrap().get_texture(TextureKind::BACK)
        };
        let mut ge = {
            self.graphics_env.as_mut().unwrap()
        };

        for p in &mut self.players {
            let mut i = 0;
            for c in &mut p.hand.iter() {
                let mut sprite = Sprite::from_texture(back.clone());
                c.sprite = Some(SpriteRef::new(sprite.id(), SpriteInfo{
                    card: c.to_owned()
                }));
                scene.add_child(sprite);
                i = i + 1;
            }
        }

        let mut i = 0;
        for p in &mut self.table.into_iter() {
            for c in &mut p.iter() {
                let mut sprite = Sprite::from_texture(back.clone());
                c.sprite = Some(SpriteRef::new(sprite.id(), SpriteInfo{
                    card: c.to_owned()
                }));
                scene.add_child(sprite);
                i = i + 1;
            }
        }
    }

    pub fn update(&mut self, size: &Size, scene: &mut Scene<Texture>) {
        let (x, y) = (size.width, size.height);

        let ge = {
            self.graphics_env.as_ref().unwrap()
        };
        let players = {
            &mut self.players
        };
        let table = {
            &mut self.table
        };
        let deck = {
            &self.deck
        };

        let mut pnum = 0;
        for p in players {
            let pos = (pnum as f64) *800.0 + 270.0;
            let mut i = 0;
            for c in &mut p.hand.iter() {
                if self.player_turn.is_some() && self.player_turn.unwrap() == pnum {
                    c.front(scene, ge.get_texture(TextureKind::CARDS));
                }
                {
                    let sprite = scene.child_mut(c.sprite.unwrap().0).unwrap();
                    i = i + 1;
                    sprite.set_position((i as f64 * 200 as f64) + (x - 200.0), pos);
                }
            }
            pnum = pnum+1;
        }


        let mut i = 0;
        for p in &mut table.into_iter() {
            for c in &mut p.iter() {
                c.front(scene, ge.get_texture(TextureKind::CARDS));
                let sprite = scene.child_mut(c.sprite.unwrap().0).unwrap();
                i = i + 1;
                sprite.set_position((i as f64 *200 as f64)+(x-200.0), 400.0+270.0);
            }
        }
    }

    pub fn render(&mut self, scene: &mut Scene<Texture>, args: &RenderArgs) {
        let gl = &mut self.graphics_env.as_mut().unwrap().gl;

        gl.draw(args.viewport(), |c, g| {
            use graphics::*;

            // Clear the screen.
            clear([0.3, 0.6, 0.3, 0.0], g);

            let transform = c
                .transform;

            scene.draw(transform, g);
        });
    }
}

impl Card {
    fn front(&mut self, scene: &mut Scene<Texture>, texture: Rc<Texture>) {
        if let Some(sprite_ref) = self.sprite {
            let offset = 15.0;
            let width = 180.0;
            let height = 270.0;

            let row = match self.suit {
                Suit::SPADES => 1.0,
                Suit::HEARTS => 2.0,
                Suit::DIAMONDS => 3.0,
                Suit::CLUBS => 4.0,
            };
            let col = self.value as f64;
            let corners = (offset*col+width*(col-1.0), offset*row+height*(row-1.0), offset*col+width*col, offset*row+height*row);
            let sprite = scene.child_mut(sprite_ref.0).unwrap();
            sprite.set_texture(texture);
            sprite.set_src_rect(rectangle_by_corners(corners.0,
                                                     corners.1,
                                                     corners.2,
                                                     corners.3))
        }
    }
    fn back(&mut self, scene: &mut Scene<Texture>, texture: Rc<Texture>) {
        if let Some(sprite_ref) = self.sprite {
            let sprite = scene.child_mut(sprite_ref.0).unwrap();
            sprite.set_texture(texture);
        }
    }
}

fn load_texture(path: &Path) -> Rc<Texture> {
    Rc::new(Texture::from_path(
        &path,
        &TextureSettings::new()
    ).unwrap())
}
