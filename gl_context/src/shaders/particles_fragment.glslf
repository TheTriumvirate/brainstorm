#version 300 es

precision mediump float;
in float lifetime;
out vec4 o_color;

void main() {
    if(length(gl_PointCoord - vec2(0.5, 0.5)) > 0.5) discard;
    o_color = vec4(0.8,lifetime, lifetime,0.4);
}