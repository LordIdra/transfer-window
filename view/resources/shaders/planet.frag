#version 330 core

precision mediump float;

in float v_alpha;
in vec2 v_texture_coordinate;

uniform sampler2D texture_sampler;

void main() {
    // Rescale texture coordinates to match aspect ratio of texture
    ivec2 tex_size = textureSize(texture_sampler, 0);
    float scale = float(tex_size.y) / float(tex_size.x);
    vec2 tex_coords = vec2(v_texture_coordinate.x * scale, v_texture_coordinate.y);

    vec4 texture_color = texture(texture_sampler, tex_coords);
    float alpha = texture_color.a * v_alpha;
    gl_FragColor = vec4(texture_color.rgb * alpha, alpha);
}