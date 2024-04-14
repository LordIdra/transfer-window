#version 330 core

in float v_alpha;
in vec2 v_texture_coordinate;

uniform sampler2D texture_sampler;

void main() {
    vec4 texture_color = texture(texture_sampler, v_texture_coordinate);
    float alpha = texture_color.a * v_alpha;
    gl_FragColor = vec4(texture_color.rgb * alpha, alpha);
}