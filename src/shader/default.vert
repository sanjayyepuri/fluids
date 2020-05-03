uniform mat4 model_view_mat; 
uniform mat4 projection_mat; 

attribute vec4 vertex_position;
void main() {
    gl_Position =  projection_mat * model_view_mat * vertex_position;
}