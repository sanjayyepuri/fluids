use web_sys::WebGlRenderingContext as GL;
use web_sys::*; 

pub struct FrameBuffer {
    w_: i32, 
    h_: i32,
    fb_: WebGlFramebuffer,
    c_: WebGlTexture,
    d_: WebGlTexture, 
}

impl FrameBuffer {
    pub fn create(mut &self, gl: GL, width: i32, height: i32) {
        self.w_ = width;
        self.h_ = height;  
        
        self.fb_ = gl.create_framebuffer().ok_or("failed to create framebuffer")?;
        
        self.bind();

        // create rgb texture
        self.c_ = create_float_texture(gl, self.w_, self.h_);
        
        let attachment0 = GL::COLOR_ATTACHMENT0;
        gl.framebuffer_texture_2d(GL::FRAMEBUFFER, attachment0, GL::TEXTURE_2D, Some(&(self.c_)), 0);
        
        self.unbind();
    }

    pub fn bind(&self) {
        gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&(self.fb_)));
    }

    pub fn unbind(&self) {
        gl.bind_framebuffer(GL::FRAMEBUFFER, None());
    }
}

// create the rgb texture for the framebuffer
fn create_float_texture(gl: GL, width: i32, height: i32) -> WebGlTexture {
    let render_texture = gl.create_texture().ok_or("failed to create rgb texture");
    gl.bind_texture(GL::TEXTURE_2D, Some(&(render_texture)));
    // uh what lol
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
        GL::TEXTURE_2D, 0, GL::RGB, w_, h_, 0, GL::RGB, GL::UNSIGNED_BYTE,  None());
    
    gl.texParameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR);
    gl.texParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE);
    gl.texParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE);
    
    gl.bind_texture(GL::TEXTURE_2D, None());

    return render_texture;
}

