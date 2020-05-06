precision mediump float;

uniform float delta_x;
uniform sampler2D w;
varying vec2 UV;

void main() {
    vec2 wLeft  = texture2D(w, UV - vec2(delta_x, 0.0)).xy; 
    vec2 wRight = texture2D(w, UV + vec2(delta_x, 0.0)).xy; 
    vec2 wDown  = texture2D(w, UV - vec2(0.0, delta_x)).xy;
    vec2 wUp    = texture2D(w, UV + vec2(0.0, delta_x)).xy;  

    float half_rdx = 1.0 / (2.0 * delta_x); 
    gl_FragColor = vec4(half_rdx * ((wRight.x - wLeft.x) + (wUp.y - wDown.y)), 0.0, 0.0, 1.0);
}