
uniform sampler2D qtexture;
varying mediump vec2 UV;
void main() {
    // gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
    gl_FragColor = texture2D(qtexture, UV);
}