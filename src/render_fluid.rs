use web_sys::WebGlRenderingContext as GL;
use nalgebra::Vector2;

use crate::render; 
use crate::texture;

use std::rc::Rc;

pub fn advection(gl: &GL,
    advect_pass:        &render::RenderPass,
    delta_x:            f32,
    delta_t:            f32,
    src_color_field:    Rc<texture::Framebuffer>,
    vector_field:       &texture::Framebuffer,
    dst_color_field:    Rc<texture::Framebuffer>,
) ->  (Rc<texture::Framebuffer>, Rc<texture::Framebuffer>) {
    dst_color_field.bind(&gl);
    render::clear_framebuffer(&gl);

    advect_pass.use_program(&gl);

    gl.uniform1f(advect_pass.uniforms["delta_x"].as_ref(), delta_x); 
    gl.uniform1f(advect_pass.uniforms["delta_t"].as_ref(), delta_t); 
    gl.uniform1i(advect_pass.uniforms["color_field_texture"].as_ref(), 0);
    gl.uniform1i(advect_pass.uniforms["vec_field_texture"].as_ref(), 1);

    gl.active_texture(GL::TEXTURE0);
    gl.bind_texture(GL::TEXTURE_2D, Some(src_color_field.get_texture()));

    gl.active_texture(GL::TEXTURE1);
    gl.bind_texture(GL::TEXTURE_2D, Some(vector_field.get_texture()));

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&advect_pass.vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0); 
    
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&advect_pass.index_buffer));

    gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
    dst_color_field.unbind(&gl);

    (dst_color_field, src_color_field)
}

pub fn jacobi_method(gl: &GL,
    jacobi_pass:    &render::RenderPass,
    iter:           usize,
    delta_x:        f32, 
    alpha:          f32, 
    r_beta:         f32, 
    x:              Rc<texture::Framebuffer>, 
    b:              &texture::Framebuffer,
    dst:            Rc<texture::Framebuffer>,
) -> (Rc<texture::Framebuffer>, Rc<texture::Framebuffer>)
{
    let bufs = [&x, &dst];
    for k in 0..iter {
        let j_source = bufs[k % 2];
        let j_dst = bufs[(k + 1) % 2];

        j_dst.bind(&gl);
        jacobi_iteration(&gl, &jacobi_pass, delta_x, alpha, r_beta, &j_source, &b);            
        j_dst.unbind(&gl);
    }
    
    // lazy code: essentially we do jacobi `iter-1` or `iter` iterations
    (x, dst)
}

pub fn jacobi_iteration(gl: &GL, 
    jacobi_pass:    &render::RenderPass,
    delta_x:        f32, 
    alpha:          f32, 
    r_beta:         f32, 
    x:              &texture::Framebuffer, 
    b:              &texture::Framebuffer,
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
    divergence_pass:    &render::RenderPass,
    delta_x:            f32, 
    w:                  &texture::Framebuffer,
    dst:                Rc<texture::Framebuffer>,
) -> Rc<texture::Framebuffer> {
    dst.bind(&gl);
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
    dst.unbind(&gl);

    dst
}

pub fn subtract(gl: &GL,
    subtract_pass:  &render::RenderPass,
    delta_x:        f32, 
    p:              &texture::Framebuffer,
    w:              Rc<texture::Framebuffer>,
    dst:            Rc<texture::Framebuffer>, 
) -> (Rc<texture::Framebuffer>, Rc<texture::Framebuffer>) {
    dst.bind(&gl);
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
    dst.unbind(&gl);

    (dst, w)
}

pub fn boundary(gl: &GL,
    boundary_pass:  &render::RenderPass,
    delta_x:        f32, 
    scale:          f32,
    x:              Rc<texture::Framebuffer>,
    dst:            Rc<texture::Framebuffer>,
) -> (Rc<texture::Framebuffer>, Rc<texture::Framebuffer>) {
    dst.bind(&gl);
    boundary_pass.use_program(&gl);

    gl.uniform1f(boundary_pass.uniforms["delta_x"].as_ref(), delta_x);
    gl.uniform1f(boundary_pass.uniforms["scale"].as_ref(), scale);

    gl.uniform1i(boundary_pass.uniforms["x"].as_ref(), 0);

    gl.active_texture(GL::TEXTURE0);
    gl.bind_texture(GL::TEXTURE_2D, Some(x.get_texture()));

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&boundary_pass.vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0); 
    
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&boundary_pass.index_buffer));

    gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
    dst.unbind(&gl);

    (dst, x)
}

pub fn force(gl: &GL,
    force_pass:  &render::RenderPass,
    delta_t:        f32, 
    rho:            f32,
    force:          &Vector2<f32>,
    impulse_pos:    &Vector2<f32>,
    velocity_field_texture:     Rc<texture::Framebuffer>,
    dst:                        Rc<texture::Framebuffer>,
) -> (Rc<texture::Framebuffer>, Rc<texture::Framebuffer>) 
{
    dst.bind(&gl);
    force_pass.use_program(&gl);

    gl.uniform1f(force_pass.uniforms["delta_t"].as_ref(), delta_t);
    gl.uniform1f(force_pass.uniforms["rho"].as_ref(), rho);
    gl.uniform2f(force_pass.uniforms["force"].as_ref(), force.x, force.y);
    gl.uniform2f(force_pass.uniforms["impulse_pos"].as_ref(), impulse_pos.x, impulse_pos.y);

    gl.uniform1i(force_pass.uniforms["velocity_field_texture"].as_ref(), 0);

    gl.active_texture(GL::TEXTURE0);
    gl.bind_texture(GL::TEXTURE_2D, Some(velocity_field_texture.get_texture()));

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&force_pass.vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0); 
    
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&force_pass.index_buffer));

    gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
    dst.unbind(&gl);

    (dst, velocity_field_texture)
}