#version 300 es

precision mediump float;
in vec2 position;
in vec3 color;
out vec3 o_color;

void main() {
    gl_Position = vec4(position, 1.0, 1.0);
    o_color = color;
}
