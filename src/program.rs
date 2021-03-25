use gl::types::*;

use crate::shader::Shader;
use std::ffi::CString;

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn new(shaders: Vec<(Shader, GLenum)>) -> Result<Self, String> {
        let program = unsafe { gl::CreateProgram() };
        unsafe {
            for (shader, _) in &shaders {
                gl::AttachShader(program, shader.id());
            }
            gl::LinkProgram(program);

            for (shader, _) in &shaders {
                gl::DetachShader(program, shader.id());
            }
        }
        let mut success: GLint = 1;
        unsafe {
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            }
            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            let error: CString = unsafe { CString::from_vec_unchecked(buffer) };
            unsafe {
                gl::GetProgramInfoLog(
                    program,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().to_string());
        }

        Ok(Program { id: program })
    }

    pub fn use_(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    pub fn set_uniform_mat4(&self, location: GLint, matrix: &glm::Mat4) {
        unsafe {
            gl::ProgramUniformMatrix4fv(
                self.id,
                location,
                1,
                gl::FALSE,
                glm::value_ptr(&matrix).as_ptr(),
            )
        };
    }

    pub fn set_uniform_vec3(&self, location: GLint, vector: &glm::Vec3) {
        unsafe { gl::ProgramUniform3fv(self.id, location, 1, glm::value_ptr(&vector).as_ptr()) };
    }

    #[allow(non_snake_case)]
    pub fn set_uniform_sampler(&self, location: GLint, texture_unit: GLuint) {
        unsafe { gl::ProgramUniform1i(self.id, location, texture_unit as GLint) };
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
