#version 330 core

out vec4 out_color;

varying vec4 color0;
varying vec2 tex_coord0;

uniform sampler2D tex;

void main() {
    vec4 color = texture(tex, tex_coord0);
    out_color = mix(color, color0, 0.5);
}
