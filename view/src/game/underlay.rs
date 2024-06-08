
use super::View;

mod celestial_objects;
mod icons;
mod segments;
mod selected;

pub fn draw(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw underlay");
    celestial_objects::draw(view);
    segments::draw(view);
    icons::draw(view);
    selected::update(view);
}