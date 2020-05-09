precision mediump float;

uniform float delta_t;
uniform float delta_x;
uniform float vorticity;

uniform sampler2D v;
varying vec2 UV;

float curl(in float x, in float y, in sampler2D v) {
    float upx = texture2D(v, vec2(x, y + delta_x)).x;
    float downx = texture2D(v, vec2(x, y - delta_x)).x; 
    float lefty = texture2D(v, vec2(x - delta_x, y)).y; 
    float righty = texture2D(v, vec2(x + delta_x, y)).y; 

    return 0.5 * (upx - downx + lefty - righty);
}

void main() {
    float x = UV.x; 
    float y = UV.y; 

    float dx = abs(curl(x, y - delta_x, v)) - abs(curl(x, y + delta_x, v));
    float dy = abs(curl(x + delta_x, y, v)) - abs(curl(x - delta_x, y, v));
    
    vec2 d = vec2(0.5 * dx, 0.5 * dy);
    float len = length(d) + 1e-9; 
    d = vorticity/len * d; 

    gl_FragColor = texture2D(v, UV) + delta_t * curl(UV.x, UV.y, v) * vec4(d, 0, 0);
}