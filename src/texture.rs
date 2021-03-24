use gl::types::*;
use image::RgbImage;
pub struct Texture {
    name: GLuint,
}
impl Texture {
    pub fn name(&self) -> GLuint {
        self.name
    }
    pub fn new(texture_image: RgbImage) -> Self {
        let texture_width = texture_image.width();
        let texture_height = texture_image.height();
        let mut texture_name = 0;
        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut texture_name);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            gl::TextureStorage2D(
                texture_name,
                4,
                gl::RGB8,
                texture_width as GLsizei,
                texture_height as GLsizei,
            );
            gl::TextureSubImage2D(
                texture_name,
                0,
                0,
                0,
                texture_width as GLsizei,
                texture_height as GLsizei,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                texture_image.into_raw().as_ptr() as *const GLvoid,
            );

            gl::GenerateTextureMipmap(texture_name);
            gl::TextureParameteri(
                texture_name,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as GLint,
            );
            gl::TextureParameteri(
                texture_name,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as GLint,
            );
            gl::TextureParameteri(
                texture_name,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST_MIPMAP_NEAREST as GLint,
            );
            gl::TextureParameteri(
                texture_name,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as GLint,
            );
        };

        Self {
            name: texture_name,
        }
    }
}
