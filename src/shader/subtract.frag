precision mediump float;

uniform float delta_x;
uniform sampler2D p;
uniform sampler2D w;
varying vec2 UV;

void main() {
    float pLeft  = texture2D(p, UV - vec2(delta_x, 0.0)).x; 
    float pRight = texture2D(p, UV + vec2(delta_x, 0.0)).x; 
    float pDown  = texture2D(p, UV - vec2(0.0, delta_x)).x;
    float pUp    = texture2D(p, UV + vec2(0.0, delta_x)).x;  

    vec4 color = texture2D(w, UV);
    float half_rdx = 1.0 / (2.0 * delta_x);
    color.xy -= half_rdx * vec2((pRight - pLeft), (pUp - pDown));

    gl_FragColor = color;
}