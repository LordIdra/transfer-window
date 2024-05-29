#version 330 core

out vec4 FragColor;
  
in vec2 v_texture_coords;

uniform sampler2D texture_sampler_lower;
uniform sampler2D texture_sampler_upper;

void main() {
    vec4 lower_color = texture(texture_sampler_lower, v_texture_coords);
    vec4 upper_color = texture(texture_sampler_upper, v_texture_coords);
    FragColor = upper_color + lower_color;
}