#version 300 es

precision mediump float;
in float dist;
out vec4 o_color;

void main() {
    if(length(gl_PointCoord - vec2(0.5, 0.5)) > 0.5) discard;
    o_color = vec4(0.8,0.1, 0.2,1.0);
}
