#version 330 core

out vec4 FragColor;

uniform float time;
uniform float width;
uniform float height;
uniform float zoom;
uniform vec2 center;
uniform float size;
uniform float speed;

float zoom_multiplier = 100.0;

float rand(float x) {
    return fract(sin(x)*929846.0);
}

vec3 glow(float t, vec2 coords) {
    float intensity = 1.0 / pow(0.7 + 0.03 * (1.0 / size) * length(coords) / (zoom_multiplier * zoom), 1.1 + 0.1 * t);
    return vec3(1.0, 1.0, 1.0) * intensity * (exp(-0.05 * t) - 0.3);
}

vec3 dust(float t, vec2 coords) {
    float radius = 10.0 * size * pow(0.05 * t, 0.5) * (zoom_multiplier * zoom);
    float distance_intensity = 1.0 - pow(abs(length(coords) - radius), 0.2);
    float time_intensity = max(0.0, 1.0 - pow(0.0025*t, 0.1));
    return 0.5 * vec3(1.0, 1.0, 0.7) * max(0.0, distance_intensity) * time_intensity;
}

float flicker(float t) {
    float flicker_rate = 10000.0;
    float point_1 = rand(floor(t*flicker_rate));
    float point_2 = rand(floor(t*flicker_rate)+1.0);
    float interpolated_point = point_1 + (point_2 - point_1)*fract(time*flicker_rate);
    return 1.0 - 0.1 * interpolated_point * exp(-0.3 * t);
}

void main() {
    vec2 raw_uv = (gl_FragCoord.xy*2.0 - vec2(width, height)) / height;
    vec2 raw_center = (vec2(center.x, -center.y)*2.0 - vec2(width, height)) / height;
    float t = 0.01 * time * speed;
    vec2 coords = raw_uv - raw_center;
    vec3 col = glow(t, coords) * flicker(t) + dust(t, coords);
    FragColor = vec4(col, 1.0);
}