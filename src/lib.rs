mod utils;
mod shader;
mod geometry;
mod texture;
mod render;
mod render_fluid;
mod gui;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;

use nalgebra::Vector3;

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

    let jacobi_slider = document().get_element_by_id("jacobi_slider").unwrap();
    let jacobi_slider: web_sys::HtmlInputElement = jacobi_slider.dyn_into::<web_sys::HtmlInputElement>()?;

    let viscocity_slider = document().get_element_by_id("viscocity_slider").unwrap();
    let viscocity_slider: web_sys::HtmlInputElement = viscocity_slider.dyn_into::<web_sys::HtmlInputElement>()?;

    let random_color = document().get_element_by_id("random_color").unwrap();
    let random_color: web_sys::HtmlInputElement = random_color.dyn_into::<web_sys::HtmlInputElement>()?;

    let splat_color = document().get_element_by_id("splat_color").unwrap();
    let splat_color: web_sys::HtmlInputElement = splat_color.dyn_into::<web_sys::HtmlInputElement>()?;

    let speed_slider = document().get_element_by_id("speed_slider").unwrap();
    let speed_slider: web_sys::HtmlInputElement = speed_slider.dyn_into::<web_sys::HtmlInputElement>()?;

    let vorticity_slider = document().get_element_by_id("vorticity_slider").unwrap();
    let vorticity_slider: web_sys::HtmlInputElement = vorticity_slider.dyn_into::<web_sys::HtmlInputElement>()?;

    let vector_field_select = document().get_element_by_id("vector_field_select").unwrap();
    let vector_field_select: web_sys::HtmlSelectElement = vector_field_select.dyn_into::<web_sys::HtmlSelectElement>()?;

    let color_field_select = document().get_element_by_id("color_field_select").unwrap();
    let color_field_select: web_sys::HtmlSelectElement = color_field_select.dyn_into::<web_sys::HtmlSelectElement>()?;

    let width: i32 = canvas.width() as i32;
    let height: i32 = canvas.height() as i32;
    let gui = Rc::new(RefCell::new(gui::Gui::new(width as f32, height as f32)));

    gui::attach_mouse_handlers(&canvas, Rc::clone(&gui), canvas.offset_left() as f32, canvas.offset_top() as f32)?;

    let gl = canvas.get_context("webgl")?.unwrap().dyn_into::<GL>()?;
    gl.get_extension("OES_texture_float")?;
    gl.get_extension("OES_texture_float_linear")?;

    let standard_vert_shader = shader::compile_shader(&gl, GL::VERTEX_SHADER, shader::STANDARD_VERTEX_SHADER)?;
    let quad_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::QUAD_FRAGMENT_SHADER)?;
    let advect_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::ADVECT_FRAGMENT_SHADER)?;
    let jacobi_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::JACOBI_FRAGMENT_SHADER)?;
    let divergence_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::DIVERGE_FRAGMENT_SHADER)?;
    let subtract_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::SUB_FRAGMENT_SHADER)?;
    let bound_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::BOUND_FRAGMENT_SHADER)?;
    let force_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::FORCE_FRAGMENT_SHADER)?;
    let color_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::COLOR_FRAGMENT_SHADER)?;
    let vorticity_frag_shader = shader::compile_shader(&gl, GL::FRAGMENT_SHADER, shader::VORT_FRAGMENT_SHADER)?;

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

    let force_pass = render::RenderPass::new(&gl,
        [&standard_vert_shader, &force_frag_shader],
        vec!["delta_t", "rho", "force", "impulse_pos", "velocity_field_texture"], "vertex_position",
        &geometry::QUAD_VERTICES, &geometry::QUAD_INDICES,
    )?;

    let color_pass = render::RenderPass::new(&gl,
        [&standard_vert_shader, &color_frag_shader],
        vec!["delta_t", "rho", "color", "impulse_pos", "color_field_texture"], "vertex_position",
        &geometry::QUAD_VERTICES, &geometry::QUAD_INDICES,
    )?;

    let vorticity_pass = render::RenderPass::new(&gl, 
        [&standard_vert_shader, &vorticity_frag_shader],
        vec!["delta_t", "delta_x", "vorticity", "v"], "vertex_position", 
        &geometry::QUAD_VERTICES, &geometry::QUAD_INDICES,
    )?;

    // RenderLoop 
    let f = Rc::new(RefCell::new(None));
    let g = f.clone(); 

    let delta_x = 1.0/width as f32;

    let vf_data = texture::make_waves_vector_field(width as f32, height as f32);
    let cb_data = texture::make_rainbow_array(width, height);

    let mut cur_vector = 0;
    let mut cur_color = 0;

    let mut src_velocity_field = Rc::new(texture::Framebuffer::create_with_data(&gl, width, height, vf_data)?);
    let mut dst_velocity_field = Rc::new(texture::Framebuffer::new(&gl, width, height)?);

    let mut src_pressure_field = Rc::new(texture::Framebuffer::new(&gl, width, height)?);
    let mut dst_pressure_field = Rc::new(texture::Framebuffer::new(&gl, width, height)?);

    let mut divergence_fb = Rc::new(texture::Framebuffer::new(&gl, width, height)?);

    let mut src_color_field = Rc::new(texture::Framebuffer::create_with_data(&gl, width, height, cb_data)?);
    let mut dst_color_field = Rc::new(texture::Framebuffer::new(&gl, width, height)?);

    let rainbow_colors = texture::get_rainbow_array();

    let mainloop: Box<dyn FnMut(i32)> = Box::new(move |now| { 
        let gui = gui.borrow();

        let iter = jacobi_slider.value_as_number() as usize;
        let delta_t = 1.0/60.0;

        let vector_field_select_value = vector_field_select.selected_index();
        let color_field_select_value = color_field_select.selected_index();

        if vector_field_select_value != cur_vector {
            src_velocity_field.delete_buffers(&gl);
            src_pressure_field.delete_buffers(&gl);
            let data = texture::get_vector_field_with_value(vector_field_select_value, width, height);
            src_velocity_field = Rc::new(texture::Framebuffer::create_with_data(&gl, width, height, data).unwrap());
            src_pressure_field = Rc::new(texture::Framebuffer::new(&gl, width, height).unwrap());

            cur_vector = vector_field_select_value;
        }
        
        if color_field_select_value != cur_color {
            src_color_field.delete_buffers(&gl);
            let data = texture::get_color_field_with_value(color_field_select_value, width, height);
            src_color_field = Rc::new(texture::Framebuffer::create_with_data(&gl, width, height, data).unwrap());

            cur_color = color_field_select_value;
        }

        {
            // advect vector field
            let result = render_fluid::advection(&gl, &advect_pass,
                delta_x, delta_t,  
                Rc::clone(&src_velocity_field), &src_velocity_field, Rc::clone(&dst_velocity_field));
            
            src_velocity_field = result.0;
            dst_velocity_field = result.1; // rust does not have destructuring assignment yet https://github.com/rust-lang/rfcs/issues/372
        }

        {
            // viscuous diffusion
            let viscocity = (10.0_f32).powf(viscocity_slider.value_as_number() as f32);
            let alpha   = delta_x.powf(2.0) / (viscocity * delta_t);
            let r_beta  = 1.0/(4.0 + alpha);

            let bufs = [&src_velocity_field, &dst_velocity_field];
            for k in 0..iter {
                let j_source = bufs[k % 2];
                let j_dst = bufs[(k + 1) % 2];

                j_dst.bind(&gl);
                render_fluid::jacobi_iteration(&gl, &jacobi_pass, delta_x, alpha, r_beta, &j_source, &j_source);            
                j_dst.unbind(&gl);
            }
        }

        {
            if gui.mouse_pressed {
                // add forces
                let rho = 1e-3;
                let speed = speed_slider.value_as_number() as f32;
                let force = speed * gui.mouse_vec;
                let impulse_pos = gui.mouse_pos;
                let result = render_fluid::force(&gl, &force_pass,
                    delta_t, rho, &force, &impulse_pos,  
                    Rc::clone(&src_velocity_field), Rc::clone(&dst_velocity_field));
                
                src_velocity_field = result.0;
                dst_velocity_field = result.1;

                // add dye
                let rand_checked = random_color.checked();
                let selected_color = &splat_color.value()[1..];
                let color_hex = hex::decode(selected_color).unwrap();
                let mut r = color_hex[0] as f32 / 255.0;
                let mut g = color_hex[1] as f32 / 255.0;
                let mut b = color_hex[2] as f32 / 255.0;
                
                if rand_checked {
                    let now_sec = now as f32 * 0.25;
                    let rand_color = rainbow_colors[(now_sec % rainbow_colors.len() as f32) as usize];
                    r = rand_color.red;
                    g = rand_color.green;
                    b = rand_color.blue;
                }

                let col = Vector3::new(r, g, b);

                let result = render_fluid::color(&gl, &color_pass,
                    delta_t, rho, &col, &impulse_pos,  
                    Rc::clone(&src_color_field), Rc::clone(&dst_color_field));
                
                src_color_field = result.0;
                dst_color_field = result.1;
            }
        }

        {
            // compute pressure 
            divergence_fb = render_fluid::divergence(&gl, &divergence_pass, 
                delta_x, &src_velocity_field, Rc::clone(&divergence_fb));

            let alpha   = -(delta_x.powf(2.0));    
            let r_beta  = 0.25;

            let result = render_fluid::jacobi_method(&gl, &jacobi_pass, iter, 
                delta_x, alpha, r_beta, 
                Rc::clone(&src_pressure_field), &divergence_fb, Rc::clone(&dst_pressure_field));

            src_pressure_field = result.0;
            dst_pressure_field = result.1;

        }

        {
            // gradient subtraction
            let result = render_fluid::subtract(&gl, &subtract_pass, 
                delta_x, &src_pressure_field, 
                Rc::clone(&src_velocity_field), Rc::clone(&dst_velocity_field));
                
            src_velocity_field = result.0;
            dst_velocity_field = result.1;
        }

        {
            // boundary conditions
            let v_result = render_fluid::boundary(&gl, &boundary_pass, 
                delta_x, -1.0, Rc::clone(&src_velocity_field), Rc::clone(&dst_velocity_field));
            src_velocity_field = v_result.0;
            dst_velocity_field = v_result.1;

            let p_result = render_fluid::boundary(&gl, &boundary_pass, 
                delta_x, 1.0, Rc::clone(&src_pressure_field), Rc::clone(&dst_pressure_field));
                src_pressure_field = p_result.0;
                dst_pressure_field = p_result.1;
        }

        {   
            let vorticity = vorticity_slider.value_as_number() as f32;
            let result = render_fluid::vorticity_confinement(&gl, &vorticity_pass, 
                delta_t, delta_x, vorticity, 
                Rc::clone(&src_velocity_field), Rc::clone(&dst_velocity_field));
            
            src_velocity_field = result.0;
            dst_velocity_field = result.1;
            
        }

        {
            // advect color field
            let result = render_fluid::advection(&gl, &advect_pass,
                 delta_x, delta_t, 
                 Rc::clone(&src_color_field), &src_velocity_field, Rc::clone(&dst_color_field));
            
            src_color_field = result.0;
            dst_color_field = result.1;
        }

        
        {   
            // render texture to screen 
            render::clear_framebuffer(&gl);

            quad_pass.use_program(&gl);
            gl.uniform1i(quad_pass.uniforms["qtexture"].as_ref(), 0);
            
            gl.active_texture(GL::TEXTURE0);
            gl.bind_texture(GL::TEXTURE_2D, Some(src_color_field.get_texture()));

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&quad_pass.vertex_buffer));
            gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0); 
            
            gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&quad_pass.index_buffer));

            gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
        }
        
        request_animation_frame(f.borrow().as_ref().unwrap());
    });

    *g.borrow_mut() = Some(Closure::wrap(mainloop));
    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
