precision mediump float;

uniform sampler2D qtexture;
varying mediump vec2 UV;
void main() {
    gl_FragColor = texture2D(qtexture, UV);
}