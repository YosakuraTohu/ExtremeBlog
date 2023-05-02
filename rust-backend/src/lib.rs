mod app;
mod components;

extern crate wasm_bindgen;
use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<bool, bool> {
    let document = gloo::utils::document();
    let container = document.get_element_by_id("container").unwrap();
    yew::Renderer::<App>::with_root(container).render();

    Ok(true)
}
