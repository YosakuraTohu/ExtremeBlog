use yew::prelude::*;
use yewdux::prelude::*;

use crate::{store::GlobalState, utils::css_toggle};

#[derive(Clone, PartialEq, Properties)]
struct NavigationButtonProps {
    name: String,
    focus: bool,
}

#[function_component]
fn NavigationButton(props: &NavigationButtonProps) -> Html {
    let NavigationButtonProps { name, focus } = props;

    html! {
        <a href={ "#" } class={classes!("components-navigation-item-button", css_toggle(*focus, "components-navigation-item-button-focus"))}>{ name }</a>
    }
}

#[function_component]
pub fn Navigation() -> Html {
    let (_, dispatch) = use_store::<GlobalState>();

    let toggle_bar_state = use_state(|| false);
    let toggle_bar = {
        let toggle_bar_state = toggle_bar_state.clone();
        dispatch.reduce(|_state| {
            GlobalState {
                sidebar_toggle: !*toggle_bar_state.clone(),
            }
            .into()
        });
        move |_| toggle_bar_state.set(!*toggle_bar_state)
    };

    html! {
        <nav class="components-navigation">
            <div class="components-navigation-title">
                <span class="components-navigation-title-font">{ "KinareYuki's Blog" }</span>
            </div>
            <div class="components-navigation-menu">
                <button class="components-navigation-menu-button" onclick={toggle_bar}>
                    <span class="i-menu"></span>
                </button>
            </div>
            <div class="components-navigation-item">
                <NavigationButton name="首页" focus=true />
                <NavigationButton name="分类" focus=false />
                <NavigationButton name="标签" focus=false />
                <NavigationButton name="归档" focus=false />
                <NavigationButton name="关于" focus=false />
            </div>
        </nav>
    }
}
