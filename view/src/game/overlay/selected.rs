use crate::game::View;

mod approach;
mod apsis;
mod burn;
mod turn;
mod encounter;
mod fire_torpedo;
mod guidance;
mod intercept;
mod orbitable;
mod burn_point;
mod turn_point;
mod guidance_point;
mod orbit_point;
mod vessel;

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update selected");
    approach::update(view);
    apsis::update(view);
    burn_point::update(view);
    turn_point::update(view);
    guidance_point::update(view);
    orbit_point::update(view);
    burn::update(view);
    turn::update(view);
    encounter::update(view);
    guidance::update(view);
    orbitable::update(view);
    intercept::update(view);
    fire_torpedo::update(view);
    vessel::update(view);
}
