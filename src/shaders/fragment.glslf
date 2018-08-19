#version 300 es

precision mediump float;
in float dist;
out vec4 o_color;

void main() {
    // Hard coded normalization with a few magic numbers
    o_color = vec4(1.0,0.0,abs(dist - 1.0) / 1.2, abs(abs(dist - 0.7071)  / 1.414 / 1.5-1.0));
}