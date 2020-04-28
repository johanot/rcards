use crate::types::{Game, Card, Suit, Player};
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

pub struct GraphicsEnv {
    textures: HashMap<TextureKind, Rc<Texture>>,
    gl: GlGraphics,
}

#[derive(Eq, PartialEq, Hash)]
pub enum TextureKind {
    BACK,
    CARDS
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
    pub fn prepare(&mut self, size: &Size, scene: &mut Scene<Texture>) {
        let (x, y) = (size.width, size.height);

        let back = {
            self.graphics_env.as_ref().unwrap().get_texture(TextureKind::BACK)
        };
        let front = {
            self.graphics_env.as_ref().unwrap().get_texture(TextureKind::CARDS)
        };
        let players = {
            &self.players
        };
        let table = {
            &self.table
        };
        let deck = {
            &self.deck
        };

        let mut i = 0;
        for c in players.get(0).unwrap().hand.into_iter() {
            let mut sprite = Sprite::from_texture(back.clone()); //c.to_sprite(front.clone());
            sprite.set_position((i as f64 *200 as f64)+(x-90.0), 270.0);
            i = i + 1;
            scene.add_child(sprite);
        }

        let mut i = 0;
        for c in players.get(1).unwrap().hand.into_iter() {
            let mut sprite = c.to_sprite(front.clone());
            sprite.set_position((i as f64 *200 as f64)+(x-90.0), 670.0);
            i = i + 1;
            scene.add_child(sprite);
        }

/*
        let mut i = 0;
        for p in table.into_iter() {
            for c in p.into_iter() {
                let mut sprite = c.to_sprite(front.clone());
                sprite.set_position((i as f64 *250 as f64)+(x-350.0), y);
                i = i + 1;
                scene.add_child(sprite);
            }
        }
*/
/*
        let mut xx = 300.0;
        let mut yy = 300.0;
        for c in deck.into_iter() {
            let mut sprite = Sprite::from_texture(back.clone());
            sprite.set_position(xx, yy);
            xx = xx - 1.5;
            yy = yy - 2.0;
            scene.add_child(sprite);
        }
*/
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

    pub fn update(&mut self, args: &UpdateArgs) {
        /*if self.graphics_env.is_some() {
            let ge = self.graphics_env.unwrap();

            for p in &self.players {
                p.draw(self.graphics_env.as_mut().unwrap());

        }*/
    }
}


impl Player {
    fn draw(&self, env: &mut GraphicsEnv) {


    }
}



trait ToSprite {
    fn to_sprite(&self, texture: Rc<Texture>) -> Sprite<Texture>;
}

impl ToSprite for Card {
    fn to_sprite(&self, texture: Rc<Texture>) -> Sprite<Texture> {
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

        Sprite::from_texture_rect(texture, rectangle_by_corners(corners.0,
                                                                    corners.1,
                                                                    corners.2,
                                                                    corners.3))
    }
}

fn load_texture(path: &Path) -> Rc<Texture> {
    Rc::new(Texture::from_path(
        &path,
        &TextureSettings::new()
    ).unwrap())
}
