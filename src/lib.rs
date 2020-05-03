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

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
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

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<GL>()?;

// BEGIN: CUBE PROGRAM SETUP
    let cube_vert_shader = shader::compile_shader(&gl, GL::VERTEX_SHADER, shader::CUBE_VERTEX_SHADER)?;
    let cube_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::CUBE_FRAGMENT_SHADER)?;
    let cube_program  = shader::link_program(&gl, &cube_vert_shader, &cube_frag_shader)?;
    // setup uniforms
    let cube_proj_mat_id = gl.get_uniform_location(&cube_program, "projection_mat");
    let cube_mv_mat_id = gl.get_uniform_location(&cube_program, "model_view_mat");
    // setup vertex attribute 
    let cube_vertex_pos_id = gl.get_attrib_location(&cube_program, "vertex_position");

    let cube_vertex_buffer = geometry::make_vertex_buffer(&gl, &geometry::CUBE_VERTICES)?;
    let cube_index_buffer = geometry::make_index_buffer(&gl, &geometry::CUBE_INDICES)?;
// END: CUBE PROGRAM SETUP    

// BEGIN: QUAD PROGRAM SETUP
    let quad_vert_shader = shader::compile_shader(&gl, GL::VERTEX_SHADER, shader::QUAD_VERTEX_SHADER)?;
    let quad_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::QUAD_FRAGMENT_SHADER)?;
    let quad_program  = shader::link_program(&gl, &quad_vert_shader, &quad_frag_shader)?;
    // setup uniforms
    let quad_tex_id = gl.get_uniform_location(&quad_program, "qtexture");

    let quad_vertex_pos_id = gl.get_attrib_location(&quad_program, "vertex_position");   

    let quad_vertex_buffer = geometry::make_vertex_buffer(&gl, &geometry::QUAD_VERTICES)?;
    let quad_index_buffer = geometry::make_index_buffer(&gl, &geometry::QUAD_INDICES)?;
// END: QUAD PROGRAM SETUP   

    let framebuffer = texture::Framebuffer::new(&gl, 400, 400)?;

    // set up model., view, and projection matrices 
    let fov = 45.0 * PI / 180.0;
    let aspect = 150.0 / 150.0; // TODO: this is hard coded right now... see if we can find this programatically
    let z_near = 0.1;
    let z_far = 100.0;
    
    // RenderLoop 
    let f = Rc::new(RefCell::new(None));
    let g = f.clone(); 

    let mut cube_rotation = 0.0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || { 
        // setup model view and projection matrices
        let projection_mat = Perspective3::<f32>::new(aspect, fov, z_near, z_far);

        let rotation = UnitQuaternion::<f32>::from_euler_angles(cube_rotation, 0.7 * cube_rotation, 0.0);
        let translation = Translation3::<f32>::new(0.0, 0.0, -6.0);
        let model_view_mat = Isometry3::<f32>::from_parts(translation, rotation);
        
        let mut proj_array = [0.; 16];
        proj_array.copy_from_slice(projection_mat.as_matrix().as_slice());

        let mut mv_array = [0.; 16]; 
        mv_array.copy_from_slice(model_view_mat.to_homogeneous().as_slice());

        // DRAW CUBE
        {   
            framebuffer.bind(&gl);
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear_depth(1.0);
            gl.enable(GL::DEPTH_TEST);
            gl.depth_func(GL::LEQUAL);
            gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&cube_vertex_buffer));
            gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0); 
            
            gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&cube_index_buffer));
            
            gl.use_program(Some(&cube_program));

            gl.uniform_matrix4fv_with_f32_array(cube_proj_mat_id.as_ref(), false, &mut proj_array);
            gl.uniform_matrix4fv_with_f32_array(cube_mv_mat_id.as_ref(), false, &mut mv_array);

            gl.draw_elements_with_i32(GL::TRIANGLES, 36, GL::UNSIGNED_SHORT, 0);
            framebuffer.unbind(&gl);
        }

        {   
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear_depth(1.0);
            gl.enable(GL::DEPTH_TEST);
            gl.depth_func(GL::LEQUAL);
            gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

            gl.bind_texture(GL::TEXTURE_2D, Some(framebuffer.get_texture()));

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&quad_vertex_buffer));
            gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0); 
            
            gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&quad_index_buffer));

            gl.use_program(Some(&quad_program));

            gl.uniform1i(quad_tex_id.as_ref(), 0);

            gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
        }

        cube_rotation += 0.01;

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
