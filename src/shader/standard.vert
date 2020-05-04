attribute vec4 vertex_position;
varying vec2 UV;
void main() {
    gl_Position = vertex_position;
    UV = vec2((vertex_position.x + 1.0)/2.0, (vertex_position.y + 1.0)/2.0);
}