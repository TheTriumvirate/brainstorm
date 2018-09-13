#version 100

precision mediump float;
varying float lifetime;

void main() {
    if(length(gl_PointCoord - vec2(0.5, 0.5)) > 0.5) discard;
    const vec4 startCol = vec4(0.8, 0.2, 0.4, 0.4);
    const vec4 endCol = vec4(0.6, 1.0, 0.3, 0.4);
    gl_FragColor = mix(startCol, endCol, lifetime);
}
