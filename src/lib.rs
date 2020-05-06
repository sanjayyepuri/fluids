mod utils;
mod shader;
mod geometry;
mod texture;
mod render;
mod render_fluid;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;

use std::cell::RefCell; 
use std::rc::Rc;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut(i32)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on the window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook(); // this allows us to get more detailed information from rust runtime errors

    // setup webgl canvas 
    let canvas = document().get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas.get_context("webgl")?.unwrap().dyn_into::<GL>()?;

    gl.get_extension("OES_texture_float")?;
    gl.get_extension("OES_texture_float_linear")?;

    let width: i32 = 512;
    let height: i32 = 512;

    let cb_data = texture::make_rainbow_array(width, height);
    let color_fbs = [texture::Framebuffer::create_with_data(&gl, width, height, cb_data)?,
                     texture::Framebuffer::new(&gl, width, height)?];

    let vf_data = texture::make_rotational_vector_field(width as f32, height as f32);
    let vector_fbs = [texture::Framebuffer::create_with_data(&gl, width, height, vf_data)?,
                      texture::Framebuffer::new(&gl, width, height)?];

    let mut pressure_fbs = [texture::Framebuffer::new(&gl, width, height).unwrap(),
                      texture::Framebuffer::new(&gl, width, height).unwrap()];

    let mut divergence_fb = texture::Framebuffer::new(&gl, width, height)?;

    let standard_vert_shader = shader::compile_shader(&gl, GL::VERTEX_SHADER, shader::STANDARD_VERTEX_SHADER)?;
    let quad_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::QUAD_FRAGMENT_SHADER)?;
    let advect_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::ADVECT_FRAGMENT_SHADER)?;
    let jacobi_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::JACOBI_FRAGMENT_SHADER)?;
    let divergence_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::DIVERGE_FRAGMENT_SHADER)?;
    let subtract_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::SUB_FRAGMENT_SHADER)?;
    let bound_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::BOUND_FRAGMENT_SHADER)?;


    let advect_pass = render::RenderPass::new(&gl, 
        [&standard_vert_shader, &advect_frag_shader],
        vec!["delta_x", "vec_field_texture",  "color_field_texture", "delta_t"], "vertex_position",
        &geometry::QUAD_VERTICES, &geometry::QUAD_INDICES,
    )?;

    let quad_pass = render::RenderPass::new(&gl,
        [&standard_vert_shader, &quad_frag_shader],
        vec!["qtexture"], "vertex_position",
        &geometry::QUAD_VERTICES, &geometry::QUAD_INDICES,
    )?;

    let jacobi_pass = render::RenderPass::new(&gl, 
        [&standard_vert_shader, &jacobi_frag_shader],
        vec!["delta_x", "alpha", "r_beta", "x", "b"], "vertex_position",
        &geometry::QUAD_VERTICES, &geometry::QUAD_INDICES,
    )?;

    let divergence_pass = render::RenderPass::new(&gl, 
        [&standard_vert_shader, &divergence_frag_shader],
        vec!["delta_x", "w"], "vertex_position",
        &geometry::QUAD_VERTICES, &geometry::QUAD_INDICES,
    )?;

    let subtract_pass = render::RenderPass::new(&gl,
        [&standard_vert_shader, &subtract_frag_shader],
        vec!["delta_x", "p", "w"], "vertex_position",
        &geometry::QUAD_VERTICES, &geometry::QUAD_INDICES,
    )?;

    let boundary_pass = render::RenderPass::new(&gl,
        [&standard_vert_shader, &bound_frag_shader],
        vec!["delta_x", "scale", "x"], "vertex_position",
        &geometry::QUAD_VERTICES, &geometry::QUAD_INDICES,
    )?;

    // RenderLoop 
    let f = Rc::new(RefCell::new(None));
    let g = f.clone(); 

    let iter = 50;
    let delta_x = 1.0/width as f32;
    let viscocity = 1e-8;            // TODO: Let user edit this constant

    let mut i = 0;
    let mainloop: Box<dyn FnMut(i32)> = Box::new(move |now| { 
        let delta_t = 1.0/60.0;
        
        // use the convention 0 is source and 1 is destination
        let mut color_field_refs = [&color_fbs[i], &color_fbs[(i + 1) % 2]];
        // let mut vector_field_refs = [&vector_fbs[0],  &vector_fbs[1]];
        let mut vector_field_refs = [&vector_fbs[i],  &vector_fbs[(i + 1) % 2]];
        let mut pressure_field_refs = [&pressure_fbs[i],  &pressure_fbs[(i + 1) % 2]];
        
        {
            // advect vector field
            vector_field_refs[1].bind(&gl);
            render_fluid::advect_color_field(&gl, delta_x, delta_t, &advect_pass, &vector_field_refs[0], &vector_field_refs[0]);
            vector_field_refs[1].unbind(&gl);
        }

        {
            // viscuous diffusion
            let alpha   = delta_x.powf(2.0) / (viscocity * delta_t);
            let r_beta  = 1.0/(4.0 + alpha);
            for k in 0..iter {
                let j_dest = &vector_fbs[k % 2];
                let j_source = &vector_fbs[(k + 1) % 2];

                j_dest.bind(&gl);
                render_fluid::jacobi_iteration(&gl, &jacobi_pass, delta_x, alpha, r_beta, &j_source, &j_source);            
                j_dest.unbind(&gl);
            }
        }

        {
            // add external forces
        }

        {
            // compute pressure 
            divergence_fb.bind(&gl);
            render_fluid::divergence(&gl, &divergence_pass, delta_x, &vector_field_refs[1]);
            divergence_fb.unbind(&gl);


            let alpha   = -(delta_x.powf(2.0));    
            let r_beta  = 0.25;
            for k in 0..iter {
                let j_source = &pressure_field_refs[k % 2];
                let j_dest = &pressure_field_refs[(k + 1) % 2];

                j_dest.bind(&gl);
                render_fluid::jacobi_iteration(&gl, &jacobi_pass, delta_x, alpha, r_beta, &j_source, &divergence_fb);            
                j_dest.unbind(&gl);
            }
        }

        {
            // gradient subtraction
            vector_field_refs[0].bind(&gl);
            render_fluid::subtract(&gl, &subtract_pass, delta_x, &pressure_field_refs[1], &vector_field_refs[1]);
            vector_field_refs[0].unbind(&gl);
        }

        {
            // boundary conditions
            vector_field_refs[1].bind(&gl);
            render_fluid::boundary(&gl, &boundary_pass, delta_x, -1.0, &vector_field_refs[0]);
            vector_field_refs[1].unbind(&gl);

            pressure_field_refs[1].bind(&gl);
            render_fluid::boundary(&gl, &boundary_pass, delta_x, 1.0, &pressure_field_refs[0]);
            pressure_field_refs[1].unbind(&gl);
        }

        {
            // advect color field
            color_field_refs[1].bind(&gl);
            render_fluid::advect_color_field(&gl, delta_x, delta_t, &advect_pass, &color_field_refs[0], &vector_field_refs[1]);
            color_field_refs[1].unbind(&gl);
        }

        
        {   
            // render texture to screen 
            render::clear_framebuffer(&gl);

            quad_pass.use_program(&gl);
            gl.uniform1i(quad_pass.uniforms["qtexture"].as_ref(), 0);
            
            gl.active_texture(GL::TEXTURE0);
            gl.bind_texture(GL::TEXTURE_2D, Some(color_field_refs[1].get_texture()));

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&quad_pass.vertex_buffer));
            gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0); 
            
            gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&quad_pass.index_buffer));

            gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
        }
        
        i = (i + 1) % 2;

        request_animation_frame(f.borrow().as_ref().unwrap());
    });

    *g.borrow_mut() = Some(Closure::wrap(mainloop));
    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
