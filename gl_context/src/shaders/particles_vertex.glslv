#version 300 es

precision mediump float;
in vec3 position;
out float dist;

uniform mat4 MVP;

void main() {
    gl_Position = MVP * vec4(position, 1.0);
    gl_PointSize = 1.0;
    dist = gl_Position.z;
}
