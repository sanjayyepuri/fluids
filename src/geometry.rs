use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer};

use wasm_bindgen::JsValue;

pub static CUBE_VERTICES: [f32; 72] = [
    // Front face
    -1.0, -1.0,  1.0,
     1.0, -1.0,  1.0,
     1.0,  1.0,  1.0,
    -1.0,  1.0,  1.0,

    // Back face
    -1.0, -1.0, -1.0,
    -1.0,  1.0, -1.0,
     1.0,  1.0, -1.0,
     1.0, -1.0, -1.0,

    // Top face
    -1.0,  1.0, -1.0,
    -1.0,  1.0,  1.0,
     1.0,  1.0,  1.0,
     1.0,  1.0, -1.0,

    // Bottom face
    -1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,
     1.0, -1.0,  1.0,
    -1.0, -1.0,  1.0,

    // Right face
     1.0, -1.0, -1.0,
     1.0,  1.0, -1.0,
     1.0,  1.0,  1.0,
     1.0, -1.0,  1.0,

    // Left face
    -1.0, -1.0, -1.0,
    -1.0, -1.0,  1.0,
    -1.0,  1.0,  1.0,
    -1.0,  1.0, -1.0,
];

pub static CUBE_INDICES: [u16; 36]= [
    0,  1,  2,      0,  2,  3,    // front
    4,  5,  6,      4,  6,  7,    // back
    8,  9,  10,     8,  10, 11,   // top
    12, 13, 14,     12, 14, 15,   // bottom
    16, 17, 18,     16, 18, 19,   // right
    20, 21, 22,     20, 22, 23,   // left
];

pub static QUAD_VERTICES: [f32; 12] = [
    -1.0, -1.0,  1.0,
     1.0, -1.0,  1.0,
     1.0,  1.0,  1.0,
    -1.0,  1.0,  1.0,
];

pub static QUAD_INDICES: [u16; 6] = [
    0,  1,  2,      0,  2,  3,
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