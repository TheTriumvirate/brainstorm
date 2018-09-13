use gl_context::{shaders::*, AbstractContext, Context, Buffer};

/// pre-condition: correct shader is bound
pub fn draw_indices(vertex_data: &Buffer<f32>, index_data: &Buffer<u16>) {
    let context = Context::get_context();
    
    OurShader::default().use_program();
    vertex_data.bind();
    index_data.bind();
    OurShader::default().bind_attribs();

    context.draw_elements(Context::TRIANGLES, index_data.len() as i32, Context::UNSIGNED_SHORT, 0);
}
