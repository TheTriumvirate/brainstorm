#version 300 es

precision mediump float;
in vec2 position;
in vec4 color;
out vec4 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    gl_PointSize = 4.0;
    v_color = color;
}
