use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};


/* SHADERS */

pub static STANDARD_VERTEX_SHADER: &'static str = include_str!("./standard.vert");
pub static QUAD_FRAGMENT_SHADER: &'static str = include_str!("./quad.frag");

pub static ADVECT_FRAGMENT_SHADER: &'static str = include_str!("./advect.frag");
pub static JACOBI_FRAGMENT_SHADER: &'static str = include_str!("./jacobi.frag");
pub static DIVERGE_FRAGMENT_SHADER: &'static str = include_str!("./divergence.frag");
pub static FORCE_FRAGMENT_SHADER:  &'static str = include_str!("./force.frag");
pub static COLOR_FRAGMENT_SHADER:  &'static str = include_str!("./dye.frag");
pub static SUB_FRAGMENT_SHADER:    &'static str = include_str!("./subtract.frag");
pub static BOUND_FRAGMENT_SHADER:  &'static str = include_str!("./boundary.frag");


pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}