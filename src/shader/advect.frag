precision mediump float;

uniform float delta_t;
uniform sampler2D color_field_texture;
uniform sampler2D vec_field_texture;
varying vec2 UV;

void main() {
    vec2 u = texture2D(vec_field_texture, UV).xy; 
    u.x = (u.x * 2.0) - 1.0;
    u.y = (u.y * 2.0) - 1.0;
    vec2 pastCoord = fract(UV - (0.5 * delta_t * u)); 

    gl_FragColor = texture2D(color_field_texture, pastCoord); 
}


