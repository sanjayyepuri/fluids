use web_sys::WebGlRenderingContext as GL;
use crate::render; 
use crate::texture;

pub fn advect_color_field(gl: &GL,
    delta_x:        f32,
    delta_t:        f32,
    advect_pass:    &render::RenderPass,
    color_field:    &texture::Framebuffer,
    vector_field:   &texture::Framebuffer
) {
    render::clear_framebuffer(&gl);

    advect_pass.use_program(&gl);

    gl.uniform1f(advect_pass.uniforms["delta_x"].as_ref(), delta_x); 
    gl.uniform1f(advect_pass.uniforms["delta_t"].as_ref(), delta_t); 
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


pub fn jacobi_iteration(gl: &GL, 
    jacobi_pass: &render::RenderPass,
    delta_x: f32, 
    alpha: f32, 
    r_beta: f32, 
    x: &texture::Framebuffer, 
    b: &texture::Framebuffer,
) 
{
    render::clear_framebuffer(&gl);
    jacobi_pass.use_program(&gl);

    gl.uniform1f(jacobi_pass.uniforms["delta_x"].as_ref(), delta_x);
    gl.uniform1f(jacobi_pass.uniforms["alpha"].as_ref(), alpha);
    gl.uniform1f(jacobi_pass.uniforms["r_beta"].as_ref(), r_beta);
    
    gl.uniform1i(jacobi_pass.uniforms["x"].as_ref(), 0);
    gl.uniform1i(jacobi_pass.uniforms["b"].as_ref(), 1);

    gl.active_texture(GL::TEXTURE0);
    gl.bind_texture(GL::TEXTURE_2D, Some(x.get_texture()));

    gl.active_texture(GL::TEXTURE1);
    gl.bind_texture(GL::TEXTURE_2D, Some(b.get_texture()));

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&jacobi_pass.vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0); 
    
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&jacobi_pass.index_buffer));

    gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
}


pub fn divergence(gl: &GL,
    divergence_pass: &render::RenderPass,
    delta_x: f32, 
    w: &texture::Framebuffer,
) {
    render::clear_framebuffer(&gl);
    divergence_pass.use_program(&gl);

    gl.uniform1f(divergence_pass.uniforms["delta_x"].as_ref(), delta_x);

    gl.uniform1i(divergence_pass.uniforms["w"].as_ref(), 0);

    gl.active_texture(GL::TEXTURE0);
    gl.bind_texture(GL::TEXTURE_2D, Some(w.get_texture()));

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&divergence_pass.vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0); 
    
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&divergence_pass.index_buffer));

    gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
}

pub fn subtract(gl: &GL,
    subtract_pass: &render::RenderPass,
    delta_x: f32, 
    p: &texture::Framebuffer,
    w: &texture::Framebuffer,
) {
    render::clear_framebuffer(&gl);
    subtract_pass.use_program(&gl);

    gl.uniform1f(subtract_pass.uniforms["delta_x"].as_ref(), delta_x);

    gl.uniform1i(subtract_pass.uniforms["p"].as_ref(), 0);
    gl.uniform1i(subtract_pass.uniforms["w"].as_ref(), 1);

    gl.active_texture(GL::TEXTURE0);
    gl.bind_texture(GL::TEXTURE_2D, Some(p.get_texture()));
    gl.active_texture(GL::TEXTURE1);
    gl.bind_texture(GL::TEXTURE_2D, Some(w.get_texture()));

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&subtract_pass.vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0); 
    
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&subtract_pass.index_buffer));

    gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
}