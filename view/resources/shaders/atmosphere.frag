#version 330 core

precision mediump float;

in float v_alpha;
in vec2 v_tex_coord;
in float v_height;
in vec4 v_color;

// 32 is a good amount for lower-end hardware, below 32 artifacts start to become really noticeable
// I like 64, 128 is also good for computers with dedicated GPUs
#define STEPS 64

float atmo_density(float height) {
    return (1 - height) * exp(height * -6);
}

float calc_z(vec2 coords, float r) {
    return sqrt(r * r - coords.x * coords.x - coords.y * coords.y);
}

void main() {
    vec2 start = v_tex_coord * (1 + v_height) * 2;
    float alpha = 0;
    float step_size = v_height / STEPS;
    vec3 position = vec3(start, calc_z(start, 1 + v_height) + 0.05);
    float distance = length(position);
    while (distance > 1 && distance < 1.1 + v_height) {
        alpha += atmo_density((distance - 1) / v_height) * step_size;
        position.z -= step_size;
        distance = length(position);
    }
    float final_alpha = alpha * v_alpha * v_color.a;
    gl_FragColor = vec4(v_color.rgb * final_alpha, final_alpha);
}