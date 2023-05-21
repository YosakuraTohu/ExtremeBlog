use yew::prelude::*;

use crate::components::{navigation::Navigation, sidebar::Sidebar};

#[function_component]
pub fn App() -> Html {
    html! {
        <>
        <Navigation/>
        <section class="section">
            <Sidebar/>
            <main class="main-root">
                <img class="main-headimg" src="static/logo.png"/>
                <p>{ "This is a simple example." }</p>
                <p>{ "More information on:" }</p>
                <p>{ "https://github.com/YosakuraTohu/ExtremeBlog" }</p>
            </main>
        </section>
        </>
    }
}
