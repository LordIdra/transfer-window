#version 330 core

precision mediump float;

in float v_alpha;
in vec2 v_tex_coord;
in float rotation_angle;

uniform sampler2D texture_sampler;

#define TAU 6.28318530718

// This is magic, it is not to be touched at risk of sanity loss, world distortion,
// or learning bits of math you never knew
vec2 otho_projection(vec2 tex_coords, float rotation_angle) {
    // https://math.stackexchange.com/questions/2357999/mapping-a-circle-to-a-hemisphere
    // Map the coordinate onto a hemisphere
    vec3 cart = vec3(tex_coords, sqrt(0.25 - tex_coords.x * tex_coords.x - tex_coords.y * tex_coords.y));
    cart = cart.zxy; // swizzler

    // Convert the cartesian coordinates to spherical coordinates
    float theta = atan(cart.y, cart.x);
    float phi = atan(length(cart.xy), cart.z);

    // Rotate the sphere
    theta += rotation_angle;

    // Normalize the spherical coordinates
    theta = mod(mod(theta, TAU) + TAU, TAU);
    phi = mod(mod(phi, TAU) + TAU, TAU);

    // Since the texure is an equirectangular projection, we can map the
    // spherical coordinates directly to the texture coordinates
    return vec2(theta / TAU, phi / (TAU / 2));
}

void main() {
    vec2 tex_coords = otho_projection(v_tex_coord, rotation_angle);
    vec4 texture_color = texture(texture_sampler, tex_coords);
    float alpha = texture_color.a * v_alpha;
    gl_FragColor = vec4(texture_color.rgb * alpha, alpha);
}