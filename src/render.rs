use std::collections::HashMap;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlShader, WebGlUniformLocation};

use wasm_bindgen::JsValue;

use crate::geometry;
use crate::shader;

pub fn clear_framebuffer(gl: &GL) {
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear_depth(1.0);
    gl.enable(GL::DEPTH_TEST);
    gl.depth_func(GL::LEQUAL);
    gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
}
 
pub struct RenderPass<'a> {
    shader_progam:      WebGlProgram,
    pub uniforms:       HashMap<&'a str, Option<WebGlUniformLocation>>,
    pub vertex_buffer:  WebGlBuffer,
    pub index_buffer:   WebGlBuffer,
    attrib_location:    i32,
}

impl RenderPass<'_>{
    pub fn new<'a>(
        gl:         &GL, 
        shaders:    [&WebGlShader; 2], 
        uniform_names:   Vec<&'a str>,
        attrib_name:&str, 
        vertices:   &[f32],
        indices:    &[u16],
    ) -> Result<RenderPass<'a>, JsValue>
    {
        let program = shader::link_program(&gl, shaders[0], shaders[1])?;
        let mut uniform_map = HashMap::new();

        for uni in uniform_names {
            uniform_map.insert(uni, gl.get_uniform_location(&program, uni));
        }

        let v_buffer = geometry::make_vertex_buffer(&gl, &vertices)?;
        let i_buffer = geometry::make_index_buffer(&gl, &indices)?;

        let a_loc = gl.get_attrib_location(&program, attrib_name);

        Ok(RenderPass {
            shader_progam: program, 
            uniforms: uniform_map,
            vertex_buffer: v_buffer,
            index_buffer: i_buffer,
            attrib_location: a_loc,
        })
    }

    pub fn use_program(&self, gl: &GL) {
        gl.use_program(Some(&self.shader_progam));
    }
}