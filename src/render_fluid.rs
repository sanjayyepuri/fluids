use web_sys::WebGlRenderingContext as GL;
use crate::render; 
use crate::texture;

pub fn advect_color_field(gl: &GL,
    delta_t: f32,
    advect_pass: &render::RenderPass,
    color_field: &texture::Framebuffer,
    vector_field: &texture::Framebuffer
) {
    render::clear_framebuffer(&gl);

    advect_pass.use_program(&gl);

    gl.uniform1f(advect_pass.uniforms["delta_t"].as_ref(), delta_t); //TODO: GET THE ACTUAL DELTA TIME
    gl.uniform1i(advect_pass.uniforms["color_field_texture"].as_ref(), 0);
    gl.uniform1i(advect_pass.uniforms["vec_field_texture"].as_ref(), 1);

    gl.active_texture(GL::TEXTURE0);
    gl.bind_texture(GL::TEXTURE_2D, Some(color_field.get_texture()));

    gl.active_texture(GL::TEXTURE1);
    gl.bind_texture(GL::TEXTURE_2D, Some(vector_field.get_texture()));

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&advect_pass.vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0); 
    
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&advect_pass.index_buffer));

    gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
}