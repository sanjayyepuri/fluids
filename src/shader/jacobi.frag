precision mediump float;

uniform float delta_x;
uniform float alpha;
uniform float r_beta;
uniform sampler2D x;
uniform sampler2D b;
varying vec2 UV;

void main() {
    vec2 xLeft  = texture2D(x, UV - vec2(delta_x, 0.0)).xy; 
    vec2 xRight = texture2D(x, UV + vec2(delta_x, 0.0)).xy; 
    vec2 xDown  = texture2D(x, UV - vec2(0.0, delta_x)).xy;
    vec2 xUp    = texture2D(x, UV + vec2(0.0, delta_x)).xy;  
    
    vec2 bCenter = texture2D(b, UV).xy; 

    gl_FragColor = vec4(r_beta * (xLeft + xRight + xUp + xDown + alpha*bCenter), 0.0, 1.0);
}
