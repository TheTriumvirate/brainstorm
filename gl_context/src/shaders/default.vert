#version 100

precision mediump float;
attribute vec3 a_position;
attribute vec3 a_color;
attribute vec2 a_texture;

varying vec3 v_color;
varying vec2 v_texture;

uniform mat4 MVP;

void main(void) {
    gl_Position = MVP * vec4(a_position, 1.0);
    v_color = a_color;
    v_texture = a_texture;
}
