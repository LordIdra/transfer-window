#version 330 core

precision mediump float;

in float v_alpha;
in vec2 v_tex_coord;
in float rotation_angle;

uniform sampler2D texture_sampler;

#define TAU 6.28318530718

vec2 otho_projection(vec2 tex_coords, float rotation_angle) {
    // Map the coordinate onto a hemisphere
    vec3 cart = vec3(tex_coords, sqrt(1.0 - dot(tex_coords, tex_coords)));
    cart = cart.zxy; // WHY?? OPENGL, WHYYYY?>???>?>:>{"^(!$&%^!(@*&%$)!@$^%&!)@$

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
    return vec2(theta / (TAU / 4), phi / (TAU / 6));
}

void main() {
    vec2 tex_coords = otho_projection(v_tex_coord, rotation_angle + 1e-6);
//    gl_FragColor = vec4(tex_coords, 0, 1);
    vec4 texture_color = texture(texture_sampler, tex_coords);
    float alpha = texture_color.a * v_alpha;
    gl_FragColor = vec4(texture_color.rgb * alpha, alpha);
}