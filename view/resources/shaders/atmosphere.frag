#version 330 core

precision highp float;

in float v_alpha;
in vec2 v_tex_coord;
in float v_height;
in vec4 v_color;

#define TAU 6.28318530718

float atmo_density(float height, float r) {
    float inside = exp(5 * (r - .5));
    float outside = (1.5 - r) * exp(-18 * (r - .5));
    return min(inside, outside);
}

float calc_z(vec2 coords, float r) {
    return sqrt(r * r - coords.x * coords.x - coords.y * coords.y);
}

void main() {
    vec2 start_coord = v_tex_coord * (1 + v_height);
    float alpha = atmo_density(v_height, length(start_coord));
    float final_alpha = alpha;
    gl_FragColor = vec4(v_color.rgb * final_alpha, final_alpha);
}