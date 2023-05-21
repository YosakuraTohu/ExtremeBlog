pub fn css_toggle<T>(state: bool, sigin: T) -> Option<T> {
    if state {
        Some(sigin)
    } else {
        None
    }
}
