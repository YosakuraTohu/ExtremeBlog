extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <main class="flex flex-col px-2 justify-center items-center h-screen w-screen bg-gradient-to-r from-sky-200 to-blue-100 font-semibold leading-6 text-sm shadow">
            <img class="w-auto mb-2" src="static/logo.png"/>
            <p>{ "This is a simple example." }</p>
            <p>{ "More information on:" }</p>
            <p>{ "https://github.com/YosakuraTohu/ExtremeBlog" }</p>
        </main>
    }
}

#[wasm_bindgen]
pub fn main() -> Result<bool, bool> {
    let document = gloo::utils::document();
    let container = document.get_element_by_id("container").unwrap();
    yew::Renderer::<App>::with_root(container).render();

    Ok(true)
}
