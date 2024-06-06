#version 330 core

out vec4 FragColor;
  
in vec2 v_texture_coords;

uniform sampler2D texture_sampler;
uniform float alpha;

void main() {
    // do not question why setting a=0.0 works
    // I have no idea
    FragColor = vec4(texture(texture_sampler, v_texture_coords).rgb * alpha, 0.0);
}