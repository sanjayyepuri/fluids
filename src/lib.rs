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

    let width: i32 = 512;
    let height: i32 = 512;

    let cb_data = texture::make_checkerboard_array(width, height);
    let color_fbs = [texture::Framebuffer::create_with_data(&gl, width, height, cb_data)?,
                     texture::Framebuffer::new(&gl, width, height)?];

    let vf_data = texture::make_sine_vector_field(width as f32, height as f32);
    let vector_fbs = [texture::Framebuffer::create_with_data(&gl, width, height, vf_data)?,
                      texture::Framebuffer::new(&gl, width, height)?];

    let standard_vert_shader = shader::compile_shader(&gl, GL::VERTEX_SHADER, shader::STANDARD_VERTEX_SHADER)?;
    let quad_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::QUAD_FRAGMENT_SHADER)?;
    let advect_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::ADVECT_FRAGMENT_SHADER)?;

    let advect_pass = render::RenderPass::new(&gl, 
        [&standard_vert_shader, &advect_frag_shader],
        vec!["vec_field_texture",  "color_field_texture", "delta_t"], "vertex_position",
        &geometry::QUAD_VERTICES, &geometry::QUAD_INDICES,
    )?;

    let quad_pass = render::RenderPass::new(&gl,
        [&standard_vert_shader, &quad_frag_shader],
        vec!["qtexture"], "vertex_position",
        &geometry::QUAD_VERTICES, &geometry::QUAD_INDICES,
    )?;

    // RenderLoop 
    let f = Rc::new(RefCell::new(None));
    let g = f.clone(); 

    let mut i = 0;
    let mut then = 0.0;
    let mainloop: Box<dyn FnMut(i32)> = Box::new(move |now| { 
        let now_sec = (now as f32) * 0.001;
        let delta_t = now_sec - then;
        then = now_sec;

        let source_cf_fb = &color_fbs[i];
        let dest_cf_fb = &color_fbs[(i + 1) % 2];

        let source_vf_fb = &vector_fbs[i];
        let dest_vf_fb = &vector_fbs[(i + 1) % 2];
        
        {
            // advect vector field
            dest_vf_fb.bind(&gl);
            render_fluid::advect_color_field(&gl, delta_t, &advect_pass, &source_vf_fb, &source_vf_fb);
            dest_vf_fb.unbind(&gl);
        }

        {
            // advect color field
            dest_cf_fb.bind(&gl);
            render_fluid::advect_color_field(&gl, delta_t, &advect_pass, &source_cf_fb, &dest_vf_fb);
            dest_cf_fb.unbind(&gl);
        }

        {   
            // render texture to screen 
            render::clear_framebuffer(&gl);

            quad_pass.use_program(&gl);
            gl.uniform1i(quad_pass.uniforms["qtexture"].as_ref(), 0);
            
            gl.active_texture(GL::TEXTURE0);
            gl.bind_texture(GL::TEXTURE_2D, Some(dest_cf_fb.get_texture()));

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
