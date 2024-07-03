#version 330 core

precision mediump float;

in float v_alpha;
in vec2 v_tex_coord;
in float v_height;
in vec4 v_color;

#define TAU 6.28318530718

float atmo_density(float height) {
    return (1 - height) * exp(height * -2);
}

float calc_z(vec2 coords, float r) {
    return sqrt(r * r - coords.x * coords.x - coords.y * coords.y);
}

void main() {
    vec2 start = v_tex_coord * (1 + v_height) * 2;
    float alpha = 0;
    float step_size = v_height / 256.0;
    vec3 position = vec3(start, calc_z(start, 1 + v_height));
    float distance = length(position);
    while (distance > 1 && distance < 1.1 + v_height) {
        alpha += atmo_density((distance - 1) / v_height) * step_size;
        position.z -= step_size;
        distance = length(position);
    }
    float final_alpha = alpha * v_alpha * v_color.a;
    gl_FragColor = vec4(v_color.rgb * final_alpha, final_alpha);
}