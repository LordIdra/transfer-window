#version 330 core

precision mediump float;

in float v_alpha;
in vec2 v_tex_coord;
in float v_height;
in vec4 v_color;

#define TAU 6.28318530718

float atmo_density(float height) {
    return (1 - height) * exp(height * -3);
}

float calc_z(vec2 coords, float r) {
    return sqrt(r * r - coords.x * coords.x - coords.y * coords.y);
}

void main() {
    float alpha = 0;
    float step_size = v_height / 128.0;
    vec3 position = vec3(v_tex_coord, calc_z(v_tex_coord, 1 + v_height));
    float distance = length(position);
    for (int steps = int(position.z / step_size); steps >= 0; steps--) {
        float point_height = (distance - 1) / v_height;
        float density = atmo_density(point_height) * step_size;
        alpha += density;
        position.z -= step_size;
        distance = length(position);
    }
    float final_alpha = alpha * v_alpha * v_color.a;
    gl_FragColor = vec4(v_color.rgb * final_alpha, final_alpha);
}