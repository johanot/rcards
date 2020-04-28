#[macro_use]
extern crate lazy_static;

extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;


mod types;
mod graphic;
mod kasino;

use std::rc::Rc;

use piston_window::*;
use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While,
};
use sdl2_window::Sdl2Window;
use graphics::rectangle::{square, rectangle_by_corners};
use crate::types::{Game, Player, Deck};
use crate::graphic::{GraphicsEnv, TextureKind};
use opengl_graphics::GlGraphics;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let (width, height) = (300, 300);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("kasino", (width, height))
            .exit_on_esc(true)
            .graphics_api(opengl)
            .vsync(false)
            .build()
            .unwrap();


/*
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let id;
    let mut scene = Scene::new();
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };
    let tex = Rc::new(Texture::from_path(
        &mut texture_context,
        assets.join("cards_deck-half.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap());



    let mut sprite = Sprite::from_texture_rect(tex.clone(), rectangle_by_corners(ace_of_spades.0,
                                                                                 ace_of_spades.1,
                                                                                 ace_of_spades.2,
                                                                                 ace_of_spades.3));
    sprite.set_position(window.draw_size().width as f64, window.draw_size().height as f64);

    id = scene.add_child(sprite);

    let seq = Action(MoveTo(1.0, 500.0, 400.0));
*/

/*
    // Run a sequence of animations.
    let seq = Sequence(vec![
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.5, 0.5)))),
        Action(Ease(EaseFunction::BounceOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
        Action(Ease(EaseFunction::ElasticOut, Box::new(MoveBy(2.0, 0.0, -100.0)))),
        Action(Ease(EaseFunction::BackInOut, Box::new(MoveBy(1.0, 0.0, -100.0)))),
        Wait(0.5),
        Action(Ease(EaseFunction::ExponentialInOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
        Action(Blink(1.0, 5)),
        While(Box::new(WaitForever), vec![
            Action(Ease(EaseFunction::QuadraticIn, Box::new(FadeOut(1.0)))),
            Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeIn(1.0)))),
        ]),
    ]);
*/

    //scene.run(id, &seq);

/*
    // This animation and the one above can run in parallel.
    let rotate = Action(Ease(EaseFunction::ExponentialInOut,
                             Box::new(RotateTo(2.0, 360.0))));
                             */
    //scene.run(id, &rotate);


    // create scene
    let mut scene: Scene<opengl_graphics::Texture> = Scene::new();

    // create game
    let graphics_env = GraphicsEnv::new(GlGraphics::new(opengl));
    let mut game = Game::new(graphics_env,
                             vec!(Player::new("player1"), Player::new("player2")));
    game.start();

    game.prepare(&window.size(), &mut scene);

    // init rendering
    let mut settings = EventSettings::new();
    settings.max_fps = 30;
    let mut events = Events::new(settings);
    let mut mouse_pos = [0.0,0.0];
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Mouse(button)) = e.press_args() {
            for s in scene.children() {
                let mx = mouse_pos[0];
                let my = mouse_pos[1];
                let [x, y, w, h] = s.bounding_box();
                if mx >= x && mx <= x+w && my >= y && my <= y+h {
                    println!("Mouse: {:?} '{} {}', Sprite: {}", button, mouse_pos[0], mouse_pos[1], &s.id());
                }
            }
        }
        e.mouse_cursor(|pos| {
            mouse_pos = pos;
        });

        if let Some(args) = e.render_args() {
            game.render(&mut scene, &args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
    }

    /*while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g, _| {

        });
        if let Some(_) = e.press_args() {
            scene.toggle(id, &seq);
            scene.toggle(id, &rotate);
        }
    }*/
}
