#version 300 es

precision mediump float;

in vec3 v_color;
in vec2 v_texture;

uniform sampler3D uSampler;

out vec4 color;

void main(void) {
    color = texture(uSampler, vec3(v_texture, 0.5)) + vec4(v_color, 1.0);
}