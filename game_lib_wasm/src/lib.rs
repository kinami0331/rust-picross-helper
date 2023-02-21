use game_core::GameCore;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // let game = GameCore::new(15, 15);
    // alert(&game.get_info());
    // log(&game.get_info());
}
