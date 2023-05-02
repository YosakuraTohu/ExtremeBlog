use yew::prelude::*;

use crate::components::navigation::Navigation;

#[function_component]
pub fn App() -> Html {
    html! {
        <>
        <Navigation/>
        <main class="main-root">
            <img class="main-headimg" src="static/logo.png"/>
            <p>{ "This is a simple example." }</p>
            <p>{ "More information on:" }</p>
            <p>{ "https://github.com/YosakuraTohu/ExtremeBlog" }</p>
        </main>
        </>
    }
}
