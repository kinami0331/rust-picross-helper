use game_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub struct GameCore {
    game: game_core::GameCore,
}

#[wasm_bindgen]
impl GameCore {
    pub fn from_json(json_str: String) -> GameCore {
        log("[rust module] GameCore::from_json(...)");
        GameCore {
            game: game_core::GameCore::from_json(&json_str),
        }
    }

    pub fn get_info(&self) -> String {
        log("[rust module] game.get_info()");
        self.game.get_info()
    }

    pub fn get_debug_info(&self) -> String {
        log("[rust module] game.get_debug_info()");
        format!("{:?}", self.game)
    }
}
