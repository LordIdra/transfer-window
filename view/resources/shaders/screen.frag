#version 330 core

out vec4 FragColor;
  
in vec2 v_texture_coords;

uniform sampler2D texture_sampler;

void main() {
    FragColor = texture(texture_sampler, v_texture_coords);
}