use crate::game::View;

mod approach;
mod apsis;
mod burn;
mod encounter;
mod fire_torpedo;
mod guidance;
mod intercept;
mod orbitable;
mod point;
mod vessel;

pub fn update(view: &mut View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update selected");
    approach::update(view);
    apsis::update(view);
    point::update(view);
    burn::update(view);
    encounter::update(view);
    guidance::update(view);
    orbitable::update(view);
    intercept::update(view);
    fire_torpedo::update(view);
    vessel::update(view);
}