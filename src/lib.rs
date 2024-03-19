use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
//use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

mod browser;
mod engine;
mod player;
mod weapon;
mod enemy;

use browser::{now, request_animation_frame};
use engine::Game;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let game_ref = Rc::new(RefCell::new(Game::new()));
    game_ref.borrow_mut().init();

    //--
    let mut keystate = browser::KeyState::new();
    let mut keyevent_receiver = browser::prepare_input()?;

    *g.borrow_mut() = Some(Closure::new(move || {
        let game = Rc::clone(&game_ref);
        
        // INPUT
        browser::process_input(&mut keystate, &mut keyevent_receiver);
        
        // UPDATE
        let mut game = game.borrow_mut();
        game.update(&keystate);
        
        // DRAW
        game.draw();

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
