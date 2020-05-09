precision mediump float;

uniform float delta_t;
uniform float rho;
uniform vec3 color;
uniform vec2 impulse_pos;
uniform sampler2D color_field_texture;
varying vec2 UV;

void main() {
    vec4 origColor = texture2D(color_field_texture, UV);

    float eps = 0.025;
    vec2 delta = UV - impulse_pos;
    if (length(delta) < eps) {
        origColor.xyz = color;
    }

    gl_FragColor = origColor;
}