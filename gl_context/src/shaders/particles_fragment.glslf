#version 300 es

precision mediump float;
in float lifetime;
out vec4 o_color;

void main() {
    if(length(gl_PointCoord - vec2(0.5, 0.5)) > 0.5) discard;
    const vec4 startCol = vec4(0.8, 0.2, 0.4, 0.4);
    const vec4 endCol = vec4(0.6, 1.0, 0.3, 0.4);
    o_color = mix(startCol, endCol, lifetime);
}