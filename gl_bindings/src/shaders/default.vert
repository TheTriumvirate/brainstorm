#version 300 es

precision mediump float;
in vec3 a_position;
in vec3 a_color;
in vec2 a_texture;

out vec3 v_color;
out vec2 v_texture;

uniform mat4 MVP;

void main(void) {
    gl_Position = MVP * vec4(a_position, 1.0);
    v_color = a_color;
    v_texture = a_texture;
}
