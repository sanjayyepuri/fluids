precision mediump float;

uniform float delta_x;
uniform float scale;
uniform sampler2D x;

varying vec2 UV;

void main() {
    float eps = delta_x;
    vec2 offset = vec2(0.0, 0.0);
    if (UV.x - 0.0 < eps) {
        offset = vec2(delta_x, 0.0);
    } else if (1.0 - UV.x < eps) {
        offset = vec2(-delta_x, 0.0);
    } else if (UV.y - 0.0 < eps) {
        offset = vec2(0.0, delta_x);
    } else if (1.0 - UV.y < eps) {
        offset = vec2(0.0, -delta_x);
    } else {
        gl_FragColor = texture2D(x, UV);
        return;
    }
    vec2 col = scale * texture2D(x, UV + offset).xy;
    gl_FragColor = vec4(col, 0.0, 1.0);
}