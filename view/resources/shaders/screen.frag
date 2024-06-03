#version 330 core

out vec4 FragColor;
  
in vec2 v_texture_coords;

uniform sampler2D texture_sampler_bloom;
uniform sampler2D texture_sampler_explosion;
uniform sampler2D texture_sampler_normal;

void main() {
    vec4 bloom_color = texture(texture_sampler_bloom, v_texture_coords);
    vec4 explosion_color = texture(texture_sampler_explosion, v_texture_coords);
    vec4 normal_color = texture(texture_sampler_normal, v_texture_coords);
    float blend_amount = 1.0 - max(normal_color.r, max(normal_color.g, normal_color.b));
    FragColor = normal_color + blend_amount * (bloom_color + explosion_color);
}