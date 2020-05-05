use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer};

use wasm_bindgen::JsValue;

pub static QUAD_VERTICES: [f32; 12] = [
    -1.0, -1.0,  1.0,
     1.0, -1.0,  1.0,
     1.0,  1.0,  1.0,
    -1.0,  1.0,  1.0,
];

pub static QUAD_INDICES: [u16; 6] = [
    0,  1,  2,      
    0,  2,  3,
];

pub static BORDER_VERTICES: [f32; 12] = [
    -1.0, -1.0,  1.0,
     1.0, -1.0,  1.0,
     1.0,  1.0,  1.0,
    -1.0,  1.0,  1.0,
];

pub static BORDER_INDICES: [u16; 8] = [
    0,  1,
    1,  2,
    2,  3,
    3,  0, 
];

pub fn make_vertex_buffer(gl: &GL, data: &[f32]) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

    unsafe { 
        let vert_array = js_sys::Float32Array::view(data);

        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER, 
            &vert_array, 
            GL::STATIC_DRAW,
        );
    }

    Ok(buffer)
}

pub fn make_index_buffer(gl: &GL, data: &[u16]) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl.create_buffer().ok_or("failed to create index buffer")?;
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let index_array = js_sys::Uint16Array::view(data);
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &index_array,
            GL::STATIC_DRAW,
        )
    }

    Ok(buffer)
}