#version 100

precision mediump float;

varying vec3 v_color;
varying vec2 v_texture;

uniform sampler2D uSampler;

void main(void) {
    gl_FragColor = texture2D(uSampler, v_texture) + vec4(v_color, 1.0);
}