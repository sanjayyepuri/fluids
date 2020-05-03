use web_sys::WebGlRenderingContext as GL;
use wasm_bindgen::JsValue;
use web_sys::*; 

pub struct Framebuffer {
    w_: i32, 
    h_: i32,
    fb_: WebGlFramebuffer,
    c_: WebGlTexture,
    // d_: WebGlTexture, 
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

    pub fn bind(&self, gl: &GL) {
        gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&(self.fb_)));
    }

    pub fn unbind(&self, gl: &GL) {
        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
    }

    // create the rgb texture for the framebuffer
    fn create_float_texture(gl: &GL, width: i32, height: i32) -> Result<WebGlTexture, JsValue> {
        let render_texture = gl.create_texture().ok_or("failed to create rgb texture")?;
        gl.bind_texture(GL::TEXTURE_2D, Some(&(render_texture)));
        // uh what lol
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D, 0, GL::RGB as i32, width, height, 0, GL::RGB, GL::UNSIGNED_BYTE, None)?;
        
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
        
        gl.bind_texture(GL::TEXTURE_2D, None);

        Ok(render_texture)
    }
}



