mod utils;
mod shader;
mod geometry;
mod texture;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer};

use nalgebra::{Isometry3, Perspective3, UnitQuaternion, Translation3, Point3, Vector3, Matrix4};

use std::f32::consts::PI;
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

    // let cube_vert_shader = shader::compile_shader(&gl, GL::VERTEX_SHADER, shader::CUBE_VERTEX_SHADER)?;
    // let cube_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::CUBE_FRAGMENT_SHADER)?;
    let standard_vert_shader = shader::compile_shader(&gl, GL::VERTEX_SHADER, shader::STANDARD_VERTEX_SHADER)?;
    let quad_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::QUAD_FRAGMENT_SHADER)?;
    let advect_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::ADVECT_FRAGMENT_SHADER)?;

// BEGIN: ADVECTION PROGRAM SETUP
    let advect_program = shader::link_program(&gl, &standard_vert_shader, &advect_frag_shader)?;
    let advect_vf_tex_id = gl.get_uniform_location(&advect_program, "vec_field_texture");
    let advect_cf_tex_id = gl.get_uniform_location(&advect_program, "color_field_texture");
    let advect_dt_tex_id = gl.get_uniform_location(&advect_program, "delta_t");
    let advect_vertex_pos_id = gl.get_attrib_location(&advect_program, "vertex_position");
    let advect_vertex_buffer = geometry::make_vertex_buffer(&gl, &geometry::QUAD_VERTICES)?;
    let advect_index_buffer = geometry::make_index_buffer(&gl, &geometry::QUAD_INDICES)?;
// END: ADVECTION PROGRAM SETUP 

// BEGIN: QUAD PROGRAM SETUP
    let quad_program  = shader::link_program(&gl, &standard_vert_shader, &quad_frag_shader)?;
    // setup uniforms
    let quad_tex_id = gl.get_uniform_location(&quad_program, "qtexture");
    let quad_vertex_pos_id = gl.get_attrib_location(&quad_program, "vertex_position");
    let quad_vertex_buffer = geometry::make_vertex_buffer(&gl, &geometry::QUAD_VERTICES)?;
    let quad_index_buffer = geometry::make_index_buffer(&gl, &geometry::QUAD_INDICES)?;
// END: QUAD PROGRAM SETUP   

    let width: i32 = 512;
    let height: i32 = 512;
    
    let cb_data = texture::make_rainbow_array(width, height);
    let cb_texture = texture::create_texture(&gl, width, height, &cb_data)?;
    let init_color_field_fb = texture::Framebuffer::create_with_texture(&gl, width, height, cb_texture)?;
    let color_fbs = [init_color_field_fb, texture::Framebuffer::new(&gl, width, height)?];

    let vf_data = texture::make_sine_vector_field(width as f32, height as f32);
    let vf_texture = texture::create_texture(&gl, width, height, &vf_data)?;
    let init_vector_field_fb = texture::Framebuffer::create_with_texture(&gl, width, height, vf_texture)?;
    let vector_fbs = [init_vector_field_fb, texture::Framebuffer::new(&gl, width, height)?];

    // set up model., view, and projection matrices 
    let fov = 45.0 * PI / 180.0;
    let aspect = 150.0 / 150.0; // TODO: this is hard coded right now... see if we can find this programatically
    let z_near = 0.1;
    let z_far = 100.0;
    
    // RenderLoop 
    let f = Rc::new(RefCell::new(None));
    let g = f.clone(); 

    let mut i = 0;
    let mut then = 0.0;
    let mainloop: Box<dyn FnMut(i32)> = Box::new(move |now| { 
        let now_sec = ((now as f32) * 0.001);
        let delta_t = now_sec - then;
        then = now_sec;

        let source_cf_fb = &color_fbs[i];
        let dest_cf_fb = &color_fbs[(i + 1) % 2];

        let source_vf_fb = &vector_fbs[i];
        let dest_vf_fv = &vector_fbs[(i + 1) % 2];

        // advect vector field
        {
            dest_vf_fv.bind(&gl);
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear_depth(1.0);
            gl.enable(GL::DEPTH_TEST);
            gl.depth_func(GL::LEQUAL);
            gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

            gl.active_texture(GL::TEXTURE0);
            gl.bind_texture(GL::TEXTURE_2D, Some(source_vf_fb.get_texture()));

            gl.active_texture(GL::TEXTURE1);
            gl.bind_texture(GL::TEXTURE_2D, Some(source_vf_fb.get_texture()));

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&advect_vertex_buffer));
            gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0); 
            
            gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&advect_index_buffer));

            gl.use_program(Some(&advect_program));

            gl.uniform1f(advect_dt_tex_id.as_ref(), delta_t); //TODO: GET THE ACTUAL DELTA TIME

            gl.uniform1i(advect_cf_tex_id.as_ref(), 0);
            gl.uniform1i(advect_vf_tex_id.as_ref(), 1);
            gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
            dest_vf_fv.unbind(&gl);
        }


        // advect color field
        {
            dest_cf_fb.bind(&gl);
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear_depth(1.0);
            gl.enable(GL::DEPTH_TEST);
            gl.depth_func(GL::LEQUAL);
            gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

            gl.active_texture(GL::TEXTURE0);
            gl.bind_texture(GL::TEXTURE_2D, Some(source_cf_fb.get_texture()));

            gl.active_texture(GL::TEXTURE1);
            gl.bind_texture(GL::TEXTURE_2D, Some(dest_vf_fv.get_texture()));

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&advect_vertex_buffer));
            gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0); 
            
            gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&advect_index_buffer));

            gl.use_program(Some(&advect_program));

            gl.uniform1f(advect_dt_tex_id.as_ref(), delta_t); //TODO: GET THE ACTUAL DELTA TIME

            gl.uniform1i(advect_cf_tex_id.as_ref(), 0);
            gl.uniform1i(advect_vf_tex_id.as_ref(), 1);
            gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);

            dest_cf_fb.unbind(&gl);
        }

        {   
            // render texture to screen 
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear_depth(1.0);
            gl.enable(GL::DEPTH_TEST);
            gl.depth_func(GL::LEQUAL);
            gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
            
            gl.active_texture(GL::TEXTURE0);
            gl.bind_texture(GL::TEXTURE_2D, Some(dest_cf_fb.get_texture()));

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&quad_vertex_buffer));
            gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0); 
            
            gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&quad_index_buffer));

            gl.use_program(Some(&quad_program));

            gl.uniform1i(quad_tex_id.as_ref(), 0);
            gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
        }
        
        i = (i + 1) % 2;

        request_animation_frame(f.borrow().as_ref().unwrap());
    });

    *g.borrow_mut() = Some(Closure::wrap(mainloop));
    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
