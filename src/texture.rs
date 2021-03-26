use gl::types::*;
use image::RgbImage;
pub struct Texture2D {
    name: GLuint,
}
impl Texture2D {
    pub fn bind(&self, texture_unit: GLuint) {
        unsafe { gl::BindTextureUnit(texture_unit, self.name) };
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
            gl::TextureParameteri(texture_name, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        };

        Self { name: texture_name }
    }
}

pub struct TextureCubeMap {
    name: GLuint,
}
impl TextureCubeMap {
    pub fn bind(&self, texture_unit: GLuint) {
        unsafe { gl::BindTextureUnit(texture_unit, self.name) };
    }
    pub fn new(texture_images: [RgbImage; 6]) -> Self {
        // https://www.reddit.com/r/opengl/comments/556zac/how_to_create_cubemap_with_direct_state_access/
        let mut texture_name = 0;
        unsafe {
            gl::CreateTextures(gl::TEXTURE_CUBE_MAP, 1, &mut texture_name);

            gl::TextureStorage2D(
                texture_name,
                1,
                gl::RGB8,
                texture_images[0].width() as GLsizei,
                texture_images[0].height() as GLsizei,
            );

            for (face, img) in texture_images.iter().enumerate() {
                let img = img.clone();
                // See the OpenGL 4.5 Core spec[0], page 212, paragraph 3
                // [0]: https://www.khronos.org/registry/OpenGL/specs/gl/glspec45.core.pdf
                gl::TextureSubImage3D(
                    texture_name,
                    0,
                    0,
                    0,
                    face as GLsizei,
                    img.width() as GLsizei,
                    img.height() as GLsizei,
                    1,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    img.into_raw().as_ptr() as *const GLvoid,
                );
            }

            gl::TextureParameteri(texture_name, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TextureParameteri(texture_name, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            gl::TextureParameteri(texture_name, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as GLint);

            gl::TextureParameteri(texture_name, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TextureParameteri(texture_name, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        };

        Self { name: texture_name }
    }
}
