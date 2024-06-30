#version 330 core

in float v_alpha;
in vec2 v_tex;

uniform sampler2D texture_sampler;

void main() {
    ivec2 tex_size = textureSize(texture_sampler, 0);
    float scale = float(tex_size.y) / float(tex_size.x);
    vec2 tex = vec2(v_tex.x * scale, v_tex.y);
    vec4 texture_color = texture(texture_sampler, tex);
    gl_FragColor = vec4(texture_color.rgb, v_alpha);
}