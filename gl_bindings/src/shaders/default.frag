#version 300 es

precision mediump float;

in vec3 v_color;
in vec2 v_texture;

uniform sampler3D uSampler;
uniform int u_layer;
uniform float u_percentage;

out vec4 color;

void main(void) {
    //color = texelFetch(uSampler, ivec3(ivec2(v_texture * 1024.0), u_layer), 0) + vec4(v_color, 1.0);
    color = texture(uSampler, vec3(v_texture, u_percentage)) + vec4(v_color, 1.0);
}