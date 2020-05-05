precision mediump float;

uniform float scale;
uniform vec2 offset;
uniform sampler2D x;

varying vec2 UV;

void main() {
    glColor = scale * texture2D(x, UV + offset);
}