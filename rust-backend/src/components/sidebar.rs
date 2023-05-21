use yew::prelude::*;
use yewdux::prelude::*;

use crate::{store::GlobalState, utils::css_toggle};

#[derive(Clone, PartialEq, Properties)]
struct SidebarItemProps {
    name: String,
    url: String,
}

#[function_component]
fn SidebarItem(props: &SidebarItemProps) -> Html {
    let SidebarItemProps { name, url } = props;

    html! {
        <a href={ url.clone() } class="components-sidebar-item">{ name }</a>
    }
}

#[function_component]
pub fn Sidebar() -> Html {
    let (state, _) = use_store::<GlobalState>();

    html! {
        <aside class={classes!("components-sidebar", css_toggle(state.sidebar_toggle.clone(), "components-sidebar-toggle"))}>
          <SidebarItem name="测试" url="#" />
        </aside>
    }
}
