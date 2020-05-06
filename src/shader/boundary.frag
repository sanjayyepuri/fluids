precision mediump float;

uniform float delta_x;
uniform float scale;
uniform sampler2D x;

varying vec2 UV;

void main() {
    vec2 offset = vec2(delta_x, 0.0);
    if (UV.x == 1.0) {
        offset = vec2(-delta_x, 0.0);
    } else if (UV.y == 0.0) {
        offset = vec2(0.0, delta_x);
    } else if (UV.y == 1.0) {
        offset = vec2(0.0, -delta_x);
    } 
    vec2 col = scale * texture2D(x, UV + offset).xy;
    gl_FragColor = vec4(col, 0.0, 1.0);
}