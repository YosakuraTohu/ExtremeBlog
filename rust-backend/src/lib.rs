extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let greeter = use_state(|| "Hello World");
    let color = use_state(|| "text-slate-400");

    let onmouseover = {
        let greeter = greeter.clone();
        let color = color.clone();

        move |_| {
            greeter.set("You mouse are over it.");
            color.set("text-green-600");
        }
    };

    let onmouseout = {
        let greeter = greeter.clone();
        let color = color.clone();

        move |_| {
            greeter.set("Hello World");
            color.set("text-slate-400");
        }
    };

    html! {
        <main class="flex flex-col justify-center items-center h-screen bg-gradient-to-r from-sky-200 to-blue-100">
            <h1 class={classes!("font-bold", "text-3xl", "md:text-6xl", *color)} {onmouseover} {onmouseout}>{ *greeter }</h1>
            <p>{ "This is a simple example." }</p>
            <p>{ "More information on:" }</p>
            <a href="https://github.com/YosakuraTohu/ExtremeBlog">{ "https://github.com/YosakuraTohu/ExtremeBlog" }</a>
        </main>
    }
}

#[wasm_bindgen]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
