#version 330 core

in vec2 v_texture_coordinate;
in float v_rotation;

uniform sampler2D texture_sampler;

void main() {
    vec4 texture_color = texture(texture_sampler, v_texture_coordinate);
    gl_fragcolor = // TODO;
}