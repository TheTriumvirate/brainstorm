#version 100

precision mediump float;

varying vec3 v_color;
varying vec2 v_texture;

void main(void) {
    gl_FragColor = vec4(v_color, 1.0);
}