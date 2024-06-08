use super::View;

mod camera;
mod keyboard;

pub fn update(view: &mut View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update misc");
    camera::update(view);
    keyboard::update(view);
}