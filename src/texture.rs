use web_sys::WebGlRenderingContext as GL;
use wasm_bindgen::JsValue;
use web_sys::*; 

use nalgebra::{Vector3};
use palette::rgb::Rgb;
use palette::encoding::srgb::Srgb;

use std::f32;
use std::f32::consts::PI;

pub struct Framebuffer{
    w_: i32, 
    h_: i32,
    fb_: WebGlFramebuffer,
    c_: WebGlTexture,
}

impl Framebuffer {
    pub fn new(gl: &GL, width: i32, height: i32) -> Result<Framebuffer, JsValue> {
        let fb = gl.create_framebuffer().ok_or("failed to create framebuffer")?;
        
        gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&fb));

        // create rgb texture
        let c = Framebuffer::create_float_texture(&gl, width, height)?;
        
        let attachment0 = GL::COLOR_ATTACHMENT0;
        gl.framebuffer_texture_2d(GL::FRAMEBUFFER, attachment0, GL::TEXTURE_2D, Some(&c), 0);
        
        gl.bind_framebuffer(GL::FRAMEBUFFER, None);

        Ok(Framebuffer {
            w_: width, 
            h_: height, 
            fb_: fb,
            c_: c,
        })
    }

    pub fn delete_buffers(&self, gl: &GL) {
        gl.delete_texture(Some(&self.c_));
        gl.delete_framebuffer(Some(&self.fb_));
    }

    pub fn create_with_data(gl: &GL, width: i32, height: i32, texture_data: Vec<f32>) -> Result<Framebuffer, JsValue>{
        let fb = gl.create_framebuffer().ok_or("failed to create framebuffer")?;
        let texture = create_texture(&gl, width, height, &texture_data)?;
        gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&fb));
        
        let attachment0 = GL::COLOR_ATTACHMENT0;
        gl.framebuffer_texture_2d(GL::FRAMEBUFFER, attachment0, GL::TEXTURE_2D, Some(&texture), 0);
        
        gl.bind_framebuffer(GL::FRAMEBUFFER, None);

        Ok(Framebuffer {
            w_: width, 
            h_: height, 
            fb_: fb,
            c_: texture,
        })
    }

    pub fn bind(&self, gl: &GL) {
        gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&(self.fb_)));
    }

    pub fn unbind(&self, gl: &GL) {
        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
    }

    pub fn get_texture(&self) -> &WebGlTexture {
        &self.c_
    }

    // create the rgb texture for the framebuffer
    fn create_float_texture(gl: &GL, width: i32, height: i32) -> Result<WebGlTexture, JsValue> {
        let render_texture = gl.create_texture().ok_or("failed to create rgb texture")?;
        gl.bind_texture(GL::TEXTURE_2D, Some(&(render_texture)));
        // uh what lol 
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D, 0, GL::RGBA as i32, width, height, 0, GL::RGBA, GL::FLOAT, None)?;
        
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
        
        gl.bind_texture(GL::TEXTURE_2D, None);

        Ok(render_texture)
    }
}

// https://stackoverflow.com/questions/9046643/webgl-create-texture
// post on how to create texture from pixel data. 
pub fn create_texture(gl: &GL, width: i32, height: i32, data: &[f32]) -> Result<WebGlTexture, JsValue> {
    let cb_texture =  gl.create_texture().ok_or("failed to create rgb texture")?;

    if data.len() != (width * height * 4) as usize {
        return Err(JsValue::from_str("invalid texture data"));
    }

    gl.bind_texture(GL::TEXTURE_2D, Some(&cb_texture));
    
    unsafe {
        let pixel_array = js_sys::Float32Array::view(data);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D, 0, GL::RGBA as i32, width, height, 0, GL::RGBA, GL::FLOAT, Some(&pixel_array))?;    
    }

    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);

    gl.bind_texture(GL::TEXTURE_2D, None);

    Ok(cb_texture)
}

pub fn make_checkerboard_array(width: i32, height: i32) -> Vec<f32> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    let block_size = width/10;
    for x in 0..width {
        for y in 0..height {
            let x_step = x/block_size;
            let y_step = y/block_size;

            let mut val = 0.0;
            if (x_step + y_step) % 2 == 0 {
                val = 1.0;
            } 

            data.push(val);
            data.push(val);
            data.push(val);
            data.push(1.0);
        }
    }
    
    data
} 

pub fn make_rainbow_array(width: i32, height: i32) -> Vec<f32> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    let mut colors = Vec::new();
    let mut c = Rgb::<Srgb, f32>::new(1.0, 0.0, 0.0);
    colors.push(c);
    for _ in 1..100 {
        c.green += 0.01;
        colors.push(c);
    }
    for _ in 1..100 {
        c.red -= 0.01;
        colors.push(c);
    }
    for _ in 1..100 {
        c.blue += 0.01;
        colors.push(c);
    }
    for _ in 1..100 {
        c.green -= 0.01;
        colors.push(c);
    }
    for _ in 1..100 {
        c.red += 0.01;
        colors.push(c);
    }
    for _ in 1..100 {
        c.blue -= 0.01;
        colors.push(c);
    }

    for r in 0..width {
        for c in 0..height {
            let size = colors.len() as i32;
            let mut sub = c as i32 - r as i32;
            while sub < size {
                sub += size;
            }
            let index = (sub) % size;
            let col = colors[index as usize];
            
            data.push(col.red); 
            data.push(col.green); 
            data.push(col.blue);
            data.push(1.0);
        }   
    }
    
    data
} 

pub fn make_sine_vector_field(width: f32, height: f32) -> Vec<f32> {
    let mut data = Vec::with_capacity((width * height * 4.0) as usize);
    
    for _ in 0..(height as i32){
        for c in 0..(width as i32) {
            // sine vector field is given by f(x, y) = [1, sin(2*pi*y)]
            let x: f32 = (c as f32 - width / 2.0)/(width/2.0);

            let v = Vector3::new(1.0, 0.5*(2.0*PI*(x as f32)).sin(), 0.0);
            
            data.push(v.x); 
            data.push(v.y); 
            data.push(0.0);
            data.push(1.0);
        }   
    }

    data
}

pub fn make_rotational_vector_field(width: f32, height: f32) -> Vec<f32> {
    let mut data = Vec::with_capacity((width * height * 4.0) as usize);
    
    for r in 0..(height as i32){
        for c in 0..(width as i32) {
            // sine vector field is given by f(x, y) = [1, sin(2*pi*y)]
            let x: f32 = (c as f32 - width / 2.0)/(width/2.0);
            let y: f32 = (height - r as f32 - height / 2.0)/(height/2.0);

            let v = Vector3::new((-2.0*PI*(y as f32)).sin(), (2.0*PI*(x as f32)).sin(), 0.0);
            
            data.push(v.x); 
            data.push(v.y); 
            data.push(0.0);
            data.push(1.0);
        }   
    }

    data
}

pub fn make_circular_vector_field(width: f32, height: f32) -> Vec<f32> {
    let mut data = Vec::with_capacity((width * height * 4.0) as usize);


    for r in 0..(height as i32){
        for c in 0..(width as i32) {
            // circular vector field is given by f(x, y) = [-y, x]
            let x: f32 = (c as f32 - width / 2.0)/(width/2.0);
            let y: f32 = (height - r as f32 - height / 2.0)/(height/2.0);
            
            let v = Vector3::new(y, x, 0.0);
            
            data.push(v.x); 
            data.push(v.y);  
            data.push(0.0);
            data.push(1.0);
        }   
    }

    data
}

pub fn make_divergent_vector_field(width: f32, height: f32) -> Vec<f32> {
    let mut data = Vec::with_capacity((width * height * 4.0) as usize);


    for r in 0..(height as i32){
        for c in 0..(width as i32) {
            let x: f32 = (c as f32 - width / 2.0)/(width/2.0);
            let y: f32 = (height - r as f32 - height / 2.0)/(height/2.0);
            
            let v = Vector3::new(x, -y, 0.0);
            
            data.push(v.x); 
            data.push(v.y); 
            data.push(0.0);
            data.push(1.0);
        }   
    }

    data
}


pub fn make_constant_vector_field(width: f32, height: f32) -> Vec<f32> {
    let mut data = Vec::with_capacity((width * height * 4.0) as usize);


    for _ in 0..(height as i32){
        for _ in 0..(width as i32) {    
            let v = Vector3::new(1.0, 0.0, 0.0);
            
            data.push(v.x); 
            data.push(v.y);  
            data.push(0.0);
            data.push(1.0);
        }   
    }

    data
}


