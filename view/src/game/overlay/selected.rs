use crate::game::View;

mod approach;
mod apsis;
mod burn;
mod burn_point;
mod encounter;
mod fire_torpedo;
mod guidance;
mod guidance_point;
mod intercept;
mod orbit_point;
mod orbitable;
mod vessel;

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update selected");
    approach::update(view);
    apsis::update(view);
    burn_point::update(view);
    guidance_point::update(view);
    orbit_point::update(view);
    burn::update(view);
    encounter::update(view);
    guidance::update(view);
    orbitable::update(view);
    intercept::update(view);
    fire_torpedo::update(view);
    vessel::update(view);
}
