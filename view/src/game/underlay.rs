
use super::View;

mod celestial_objects;
mod icons;
mod segments;
mod selected;

pub fn draw(view: &mut View) -> bool {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw underlay");
    celestial_objects::draw(view);
    segments::draw(view);
    let is_mouse_over_any_icon = icons::draw(view);
    selected::update(view, is_mouse_over_any_icon);
    is_mouse_over_any_icon
}