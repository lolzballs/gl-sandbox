#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;
layout (location = 2) in vec2 tex_coord;
layout (location = 3) in vec3 normal;

uniform mat4 mvp;

varying vec4 color0;

void main() {
    color0 = color;

    gl_Position = mvp * vec4(position, 1.0);
}
