#version 100

precision mediump float;
attribute vec2 a_position;
attribute vec3 a_color;
attribute vec2 a_texture;

varying vec3 v_color;
varying vec2 v_texture;

void main(void) {
    gl_Position = vec4(a_position, 0.0, 1.0);
    v_color = a_color;
    v_texture = a_texture;
}
