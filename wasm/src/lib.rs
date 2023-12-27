mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;
use fgcd_parse;
use web_sys;
use wasm_bindgen_futures;
use js_sys;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub async fn fetch_bytes(url: &str) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    let window = web_sys::window().unwrap();
    let request = web_sys::Request::new_with_str(url).unwrap();
    let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(response.is_instance_of::<web_sys::Response>());
    let response: web_sys::Response = response.dyn_into().unwrap();

    let buf_promise = response.array_buffer().unwrap();
    let buf = wasm_bindgen_futures::JsFuture::from(buf_promise).await?;
    assert!(buf.is_instance_of::<js_sys::ArrayBuffer>());

    let typebuf: js_sys::Uint8Array = js_sys::Uint8Array::new(&buf);
    let mut bytes = vec![0; typebuf.length() as usize];
    typebuf.copy_to(&mut bytes[..]);
    Ok(bytes)
}

#[wasm_bindgen]
pub async fn read_game(game_name: String) -> Result<JsValue, JsValue> {
    set_panic_hook();

    let bytes = fetch_bytes(&format!("data/games/{game_name}/Game.bin")).await?;
    let game = fgcd_parse::binary::game::read_game_bytes(&bytes).unwrap();
    Ok(serde_wasm_bindgen::to_value(&game)?)
}

/*#[wasm_bindgen]
pub async fn read_character(game_name: String, character_name: String) -> Result<JsValue, JsValue> {
    set_panic_hook();

    let bytes = fetch_bytes("Game.bin").await?;
    let game = fgcd_parse::binary::game::character::read_character_bytes(&bytes).unwrap();
    Ok(serde_wasm_bindgen::to_value(&game)?)
}*/
