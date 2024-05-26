use nalgebra_glm::{vec2, DVec2};

/// <https://stackoverflow.com/a/46007540>
/// <https://blog.chatfield.io/simple-method-for-distance-to-ellipse/>
/// This is a GENIUS very efficient and accurate solution
pub fn solve_for_closest_point_on_ellipse(a: f64, b: f64, p: DVec2) -> DVec2 {
    // let px = f64::abs(p[0]);
    // let py = f64::abs(p[1]);
    let px = p.x;
    let py = p.y;

    let mut tx = 0.707 * px;
    let mut ty = 0.707 * py;

    for _ in 0..3 {
        let x = a*tx;
        let y = b*ty;

        let ex = (a.powi(2) - b.powi(2)) * tx.powi(3) / a;
        let ey = (b.powi(2) - a.powi(2)) * ty.powi(3) / b;

        let qx = px - ex;
        let qy = py - ey;
        let q = f64::sqrt(qx.powi(2) + qy.powi(2));

        let rx = x - ex;
        let ry = y - ey;
        let r = f64::sqrt(rx.powi(2) + ry.powi(2));

        tx = (qx * r / q + ex) / a;
        ty = (qy * r / q + ey) / b;

        let t = f64::sqrt(ty.powi(2) + tx.powi(2));

        tx /= t;
        ty /= t;
    }

    vec2(a*tx, b*ty)
}
