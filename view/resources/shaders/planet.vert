#version 330 core

layout (location = 0) in vec2 x;
layout (location = 1) in vec2 y;
layout (location = 2) in vec2 texture_coordinate;
out vec2 v_texture_coordinate;
out float v_rotation;

uniform mat3 zoom_matrix;
uniform mat3 translation_matrix_upper;
uniform mat3 translation_matrix_lower;
uniform float rotation;

void main() {
    vec3 position_upper = zoom_matrix * translation_matrix_upper * vec3(x.x, y.x, 1.0);
    vec3 position_lower = zoom_matrix * translation_matrix_lower * vec3(x.y, y.y, 1.0);
    vec3 combined_position = position_upper + position_lower;
    gl_Position = vec4(combined_position.x, combined_position.y, 0.0, 1.0);
    v_texture_coordinate = texture_coordinate;
    v_rotation = rotation;
}