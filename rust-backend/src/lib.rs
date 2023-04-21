extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <h1 class="font-bold text-center">{ "Hello World!" }</h1>
    }
}

#[wasm_bindgen]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
