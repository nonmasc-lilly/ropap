extern crate glfw;
extern crate gl33;
use glfw::Context;

pub struct Renderer {
    changed:        bool,
    glfw:           glfw::Glfw,
    window:         glfw::PWindow,
    resolution_x:   u32,
    resolution_y:   u32,
    gl:             gl33::GlFns,
    vao:            u32,
    vbo:            u32,
    shader:         u32,
    texture:        u32,
    pixel_map:      Vec<u32>
}

impl Renderer {
    fn create_window(
            glfw_instance: &mut glfw::Glfw,
            width: u32,
            height: u32,
            title: &str) -> glfw::PWindow {
        glfw_instance.window_hint(glfw::WindowHint::ContextVersion(3,3));
        glfw_instance.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw_instance.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGl));
        glfw_instance.window_hint(glfw::WindowHint::Resizable(false));
        let (window, _events) = glfw_instance.create_window(
            width,
            height,
            title,
            glfw::WindowMode::Windowed
        ).unwrap();
        window
    }
    fn compile_shader(gl: &gl33::GlFns, source: &str, kind: gl33::ShaderType) -> u32 {
        unsafe {
            let mut success: i32 = 0;
            let ret = gl.CreateShader(kind);
            gl.ShaderSource(
                ret,
                1,
                &(source.as_bytes().as_ptr().cast()),
                &(source.len().try_into().unwrap())
            );
            gl.CompileShader(ret);
            gl.GetShaderiv(
                ret,
                gl33::gl_enumerations::GL_COMPILE_STATUS,
                &mut success
            );
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl.GetShaderInfoLog(
                    ret,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast()
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Shader Compile Error: {}", String::from_utf8_lossy(&v));
            }
            ret
        }
    }
    fn link_shaders(gl: &gl33::GlFns, shader_objects: &[u32]) -> u32 {
        unsafe {
            let ret = gl.CreateProgram();
            let mut success: i32 = 0;
            for object in shader_objects {
                gl.AttachShader(ret, *object);
            }
            gl.LinkProgram(ret);
            gl.GetProgramiv(
                ret,
                gl33::gl_enumerations::GL_LINK_STATUS,
                &mut success
            );
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl.GetShaderInfoLog(
                    ret,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast()
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
            ret
        }
    }
    pub fn new(resolution_x: u32, resolution_y: u32, window_width: u32, window_height: u32,
            title: &str) -> Renderer {
        const VERTICES: [[f32; 5]; 6] = [
            [-1.0,  1.0, 0.0,  0.0, 1.0],
            [ 1.0, -1.0, 0.0,  1.0, 0.0],
            [ 1.0,  1.0, 0.0,  1.0, 1.0],
            [-1.0,  1.0, 0.0,  0.0, 1.0],
            [-1.0, -1.0, 0.0,  0.0, 0.0],
            [ 1.0, -1.0, 0.0,  1.0, 0.0]
        ];
        const VERTICES_BYTE_SIZE: u32 = 120;
        const VERTICES_STRIDE:    u32 = 20;
        const VERT_SHADER: &str = 
            "#version 330 core\n\
            layout (location = 0) in vec3 pos;\n\
            layout (location = 1) in vec2 uv;\n\
            out vec2 tof_uv;\n\
            void main() {\n\
            \tgl_Position = vec4(pos.x, pos.y, pos.z, 1.0);\n\
            \ttof_uv = uv;\n\
            }\n";
        const FRAG_SHADER: &str =
            "#version 330 core\n\
            in vec2 tof_uv;\n\
            out vec4 out_color;\n\
            uniform sampler2D otexture;\n\
            void main() {\n\
            \tout_color = texture(otexture, tof_uv);\n\
            }\n";
        /* check that resolution is valid */
        if resolution_x > window_width || resolution_y > window_height {
            panic!("Invalid resolution: res={},{} win={},{}",
                resolution_x, resolution_y,
                window_width, window_height
            );
        }
        /* GLFW init */
        let mut glfw_instance: glfw::Glfw = glfw::init_no_callbacks().unwrap();
        let mut window: glfw::PWindow = Self::create_window(
            &mut glfw_instance,
            window_width,
            window_height,
            title
        );
        window.make_current();

        /* OpenGL init */
        let gl: gl33::GlFns = unsafe {
            gl33::GlFns::load_from(&|p| glfw_instance.get_proc_address_raw(
                    std::ffi::CStr::from_ptr(p as *const i8).to_str().unwrap()
            )).unwrap()
        };
        unsafe {
            gl.BlendFunc(
                gl33::gl_enumerations::GL_SRC_ALPHA,
                gl33::gl_enumerations::GL_SRC_ALPHA
            );
            gl.ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        /* setup objects */
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            assert_ne!(vao, 0);
            gl.GenBuffers(1, &mut vbo);
            assert_ne!(vbo, 0);
            gl.BindVertexArray(vao);
            gl.BindBuffer(gl33::gl_enumerations::GL_ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl33::gl_enumerations::GL_ARRAY_BUFFER,
                VERTICES_BYTE_SIZE.try_into().unwrap(),
                VERTICES.as_ptr().cast(),
                gl33::gl_enumerations::GL_STATIC_DRAW
            );
            gl.VertexAttribPointer(
                0,
                3,
                gl33::gl_enumerations::GL_FLOAT,
                0 as std::ffi::c_uchar,
                VERTICES_STRIDE.try_into().unwrap(),
                0 as *const _
            );
            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(
                1,
                2,
                gl33::gl_enumerations::GL_FLOAT,
                0 as std::ffi::c_uchar,
                VERTICES_STRIDE.try_into().unwrap(),
                12 as *const _
            );
            gl.EnableVertexAttribArray(1);
        }
        
        /* shader */
        let vertex_shader: u32 = Self::compile_shader(
            &gl,
            VERT_SHADER,
            gl33::gl_enumerations::GL_VERTEX_SHADER
        );
        let fragment_shader: u32 = Self::compile_shader(
            &gl,
            FRAG_SHADER,
            gl33::gl_enumerations::GL_FRAGMENT_SHADER
        );
        let shader: u32 = Self::link_shaders(
            &gl,
            &[vertex_shader, fragment_shader]
        );
        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);

        /* texture */
        let pixel_map: Vec<u32> = vec![0; (resolution_x*resolution_y*3).try_into().unwrap()];
        let mut texture: u32 = 0;
        unsafe {
            gl.GenTextures(1, &mut texture);
            gl.BindTexture(gl33::gl_enumerations::GL_TEXTURE_2D, texture);
            gl.TexParameteri(
                gl33::gl_enumerations::GL_TEXTURE_2D,
                gl33::gl_enumerations::GL_TEXTURE_WRAP_S,
                gl33::gl_enumerations::GL_REPEAT.0.try_into().unwrap()
            );
            gl.TexParameteri(
                gl33::gl_enumerations::GL_TEXTURE_2D,
                gl33::gl_enumerations::GL_TEXTURE_WRAP_T,
                gl33::gl_enumerations::GL_REPEAT.0.try_into().unwrap()
            );
            gl.TexParameteri(
                gl33::gl_enumerations::GL_TEXTURE_2D,
                gl33::gl_enumerations::GL_TEXTURE_MIN_FILTER,
                gl33::gl_enumerations::GL_NEAREST.0.try_into().unwrap()
            );
            gl.TexParameteri(
                gl33::gl_enumerations::GL_TEXTURE_2D,
                gl33::gl_enumerations::GL_TEXTURE_MAG_FILTER,
                gl33::gl_enumerations::GL_NEAREST.0.try_into().unwrap()
            );
            gl.TexImage2D(
                gl33::gl_enumerations::GL_TEXTURE_2D,
                0,
                gl33::gl_enumerations::GL_RGBA.0.try_into().unwrap(),
                resolution_x.try_into().unwrap(),
                resolution_y.try_into().unwrap(),
                0,
                gl33::gl_enumerations::GL_RGBA,
                gl33::gl_enumerations::GL_UNSIGNED_INT_8_8_8_8,
                pixel_map.as_ptr().cast()
            );
        }

        let ret: Renderer = Renderer {
            changed:        false,
            glfw:           glfw_instance,
            window:         window,
            resolution_x:   resolution_x,
            resolution_y:   resolution_y,
            gl:             gl,
            vao:            vao,
            vbo:            vbo,
            shader:         shader,
            texture:        texture,
            pixel_map:      pixel_map
        };
        return ret;
    }
    pub fn is_closed(&mut self) -> bool {
        return self.window.should_close();
    }
    pub fn update(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();
        unsafe {
            self.gl.Clear(gl33::gl_enumerations::GL_COLOR_BUFFER_BIT);
            if self.changed {
                self.gl.BindTexture(gl33::gl_enumerations::GL_TEXTURE_2D, self.texture);
                self.gl.TexSubImage2D(
                    gl33::gl_enumerations::GL_TEXTURE_2D,
                    0,
                    0, 0,
                    self.resolution_x.try_into().unwrap(),
                    self.resolution_y.try_into().unwrap(),
                    gl33::gl_enumerations::GL_RGBA,
                    gl33::gl_enumerations::GL_UNSIGNED_INT_8_8_8_8,
                    self.pixel_map.as_ptr().cast()
                );
                self.changed = false;
            }
        }
        self.draw();
    }
    pub fn draw(&mut self) {
        unsafe {
            self.gl.BindTexture(gl33::gl_enumerations::GL_TEXTURE_2D, self.texture);
            self.gl.BindVertexArray(self.vao);
            self.gl.UseProgram(self.shader);
            self.gl.DrawArrays(gl33::gl_enumerations::GL_TRIANGLES, 0, 6);
        }
    }
    pub fn destroy(&mut self) {
        unsafe {
            self.gl.DeleteVertexArrays(1, &self.vao);
            self.gl.DeleteBuffers(1, &self.vbo);
            self.gl.DeleteProgram(self.shader);
        }
    }
    pub fn put_pixel(&mut self, x: u32, y: u32, color: u32) {
        self.pixel_map[(x+(self.resolution_y-y-1)*self.resolution_x) as usize] = color;
        self.changed = true;
    }
    pub fn put_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        let f_width = if x + width > self.resolution_x { self.resolution_x - x } else { width };
        let f_height = if y + height > self.resolution_y { self.resolution_y - y } else { height };
        for i in 0..f_height {
            let beginning:  usize = (x+(self.resolution_y-(y+i)-1)*self.resolution_x) as usize;
            let end:        usize = beginning+(f_width as usize);
            self.pixel_map[beginning..end].fill(color);
        }
        self.changed = true;
    }
    pub fn get_key_pressed(&mut self, key: glfw::Key) -> bool {
        return self.window.get_key(key) == glfw::Action::Press;
    }
    pub fn get_key_released(&mut self, key: glfw::Key) -> bool {
        return self.window.get_key(key) == glfw::Action::Release;
    }
}
