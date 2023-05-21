use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct GlobalState {
    pub sidebar_toggle: bool,
}
