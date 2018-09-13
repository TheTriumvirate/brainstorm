#version 100

precision mediump float;
attribute vec2 position;
attribute vec3 color;
varying vec3 o_color;

void main() {
    gl_Position = vec4(position, 1.0, 1.0);
    o_color = color;
}
