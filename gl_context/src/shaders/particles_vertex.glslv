#version 100

precision mediump float;
attribute vec4 position;
varying float lifetime;
uniform mat4 MVP;

void main() {
    gl_Position = MVP * vec4(position.xyz, 1.0);
    gl_PointSize = 8.0 / length(gl_Position);
    lifetime = position.a;
}
