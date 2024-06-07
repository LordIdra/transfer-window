// https://learnopengl.com/Advanced-Lighting/Bloom
#version 330 core

out vec4 FragColor;
in vec2 v_texture_coords;

uniform sampler2D image;
uniform bool is_horizontal;
uniform float weight[5] = float[] (0.357027, 0.1945946, 0.1216216, 0.054054, 0.016216);

void main() {
    vec2 offset = 1.0 / textureSize(image, 0); // gets size of single texel
    vec3 result = texture(image, v_texture_coords).rgb * weight[0]; // current fragment's contribution
    if (is_horizontal) {
        for (int i = 1; i < 5; ++i) {
            result += texture(image, v_texture_coords + vec2(offset.x * i, 0.0)).rgb * weight[i];
            result += texture(image, v_texture_coords - vec2(offset.x * i, 0.0)).rgb * weight[i];
        }
    } else {
        for (int i = 1; i < 5; ++i) {
            result += texture(image, v_texture_coords + vec2(0.0, offset.y * i)).rgb * weight[i];
            result += texture(image, v_texture_coords - vec2(0.0, offset.y * i)).rgb * weight[i];
        }
    }
    FragColor = vec4(result, 1.0);
}