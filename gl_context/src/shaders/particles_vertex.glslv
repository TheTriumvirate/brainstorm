#version 300 es

precision mediump float;
in vec4 position;
out float lifetime;
uniform mat4 MVP;

void main() {
    gl_Position = MVP * vec4(position.xyz, 1.0);
    gl_PointSize = 4.0 / length(gl_Position);
    lifetime = position.a;
}
