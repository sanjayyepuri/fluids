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
    gl_FragColor = vec4(scale * texture2D(x, UV + offset).xy, 0.0, 1.0);
}