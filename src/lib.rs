mod game;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}, {}!", name, 12));
}

#[wasm_bindgen]
pub fn render() -> JsValue {
    let game: game::Game = game::Game::new();
    return JsValue::from_serde(&game.render()).unwrap();
}

