precision mediump float;

uniform float delta_t;
uniform float rho;
uniform vec2 force;
uniform vec2 impulse_pos;
uniform sampler2D velocity_field_texture;
varying vec2 UV;

void main() {
    vec4 color = texture2D(velocity_field_texture, UV);

    vec2 delta = UV - impulse_pos;
    float scale = delta_t * exp(-(pow(delta.x, 2.0) + pow(delta.y, 2.0))/rho);

    color.xy += scale * force;

    gl_FragColor = color;
}