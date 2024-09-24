use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint, WindowMode};
use std::{ffi::CString, ptr};

macro_rules! vertices {
    [$(($x:expr, $y:expr, $z:expr)),* $(,)?] => {
        [
        $(
            $x,
            $y,
            $z,
        )*
        ]
    };
}

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;

    void main()
    {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const ORANGE_FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    
    void main()
    {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

// make this actually yellow
const YELLOW_FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    
    void main()
    {
        FragColor = vec4(0.5f, 0.5f, 0.2f, 1.0f);
    }
"#;

unsafe fn init_shader_program() -> (u32, u32) {
    // init shaders
    let (vertex_shader, orange_fragment_shader, yellow_fragment_shader) = {
        unsafe fn log_shader_complation_err(shader: u32, name: &str) {
            let mut success = true as _;

            let mut info_log = vec![0; 512];
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success != true as _ {
                gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr().cast());
                println!("ERROR::SHADER::{}::COMPILATION_FAILED", name);
                println!("{}", String::from_utf8(info_log).unwrap());
            }
        }
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let vertex_shader_source = CString::new(VERTEX_SHADER_SOURCE).unwrap();

        gl::ShaderSource(
            vertex_shader,
            1,
            &vertex_shader_source.as_ptr(),
            ptr::null(),
        );
        gl::CompileShader(vertex_shader);
        log_shader_complation_err(vertex_shader, "VERTEX");

        // orange shader
        let orange_fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment_shader_source = CString::new(ORANGE_FRAGMENT_SHADER_SOURCE).unwrap();

        gl::ShaderSource(
            orange_fragment_shader,
            1,
            &fragment_shader_source.as_ptr(),
            ptr::null(),
        );
        gl::CompileShader(orange_fragment_shader);
        log_shader_complation_err(orange_fragment_shader, "FRAGMENT");

        // yellow shader
        let yellow_fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment_shader_source = CString::new(YELLOW_FRAGMENT_SHADER_SOURCE).unwrap();

        gl::ShaderSource(
            yellow_fragment_shader,
            1,
            &fragment_shader_source.as_ptr(),
            ptr::null(),
        );
        gl::CompileShader(yellow_fragment_shader);
        log_shader_complation_err(yellow_fragment_shader, "FRAGMENT");

        (
            vertex_shader,
            orange_fragment_shader,
            yellow_fragment_shader,
        )
    };

    unsafe fn log_shader_program_compilation_err(shader_program: u32, name: &str) {
        let mut success = true as _;
        let mut info_log = vec![0; 512];
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        if success != true as _ {
            gl::GetShaderInfoLog(
                shader_program,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr().cast(),
            );
            println!("ERROR::PROGRAM::{name}::LINKING_FAILED");
            println!("{}", String::from_utf8(info_log).unwrap());
        }
    }
    // init orange shader program
    let orange_shader_program = gl::CreateProgram();
    gl::AttachShader(orange_shader_program, vertex_shader);
    gl::AttachShader(orange_shader_program, orange_fragment_shader);
    gl::LinkProgram(orange_shader_program);
    log_shader_program_compilation_err(orange_shader_program, "ORANGE_SHADER");

    // init yellow shader program
    let yellow_shader_program = gl::CreateProgram();
    gl::AttachShader(yellow_shader_program, vertex_shader);
    gl::AttachShader(yellow_shader_program, yellow_fragment_shader);
    gl::LinkProgram(yellow_shader_program);
    log_shader_program_compilation_err(orange_shader_program, "ORANGE_SHADER");

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(orange_fragment_shader);
    gl::DeleteShader(yellow_fragment_shader);
    (orange_shader_program, yellow_shader_program)
}

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("glfw initialization failed");
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    #[cfg(target_os = "macos")]
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    let (mut window, _) = glfw
        .create_window(800, 600, "LearnOpenGL", WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    glfw.make_context_current(Some(&window));

    gl::load_with(|symbol| window.get_proc_address(symbol).cast());

    window.set_framebuffer_size_callback(framebuffer_size_callback);

    let (orange_shader_program, yellow_shader_program) = unsafe { init_shader_program() };

    let t: [[f32; 9]; 2] = [
        vertices![(-0.9, -0.5, 0.0), (-0.0, -0.5, 0.0), (-0.45, 0.5, 0.0)],
        vertices![(0.0, -0.5, 0.0), (0.9, -0.5, 0.0), (0.45, 0.5, 0.0)],
    ];

    let mut vbos = [0; 2];
    let mut vaos = [0; 2];
    unsafe {
        gl::GenVertexArrays(2, vaos.as_mut_ptr());
        gl::GenBuffers(2, vbos.as_mut_ptr());

        gl::BindVertexArray(vaos[0]);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbos[0]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (t[0].len() * size_of::<f32>()) as isize,
            t[0].as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            false as _,
            3 * size_of::<f32>() as i32,
            0 as _,
        );
        gl::EnableVertexAttribArray(0);

        gl::BindVertexArray(vaos[1]);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbos[1]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (t[1].len() * size_of::<f32>()) as isize,
            t[1].as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            false as _,
            3 * size_of::<f32>() as i32,
            0 as _,
        );

        gl::EnableVertexAttribArray(0);
    }

    // let square: [f32; 12] = vertices![
    //     (0.5, 0.5, 0.0),
    //     (0.5, -0.5, 0.0),
    //     (-0.5, -0.5, 0.0),
    //     (-0.5, 0.5, 0.0)
    // ];
    // let indices = [0, 1, 3, 1, 2, 3];

    // let (mut vbo, mut vao, mut ebo) = (0, 0, 0);

    // unsafe {
    //     gl::GenVertexArrays(1, &mut vao);
    //     gl::GenBuffers(1, &mut vbo);
    //     gl::GenBuffers(1, &mut ebo);

    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         (triangles.len() * size_of::<f32>()) as isize,
    //         triangles.as_ptr().cast(),
    //         gl::STATIC_DRAW,
    //     );

    //     gl::BindVertexArray(vao);
    //     gl::VertexAttribPointer(
    //         0,
    //         3,
    //         gl::FLOAT,
    //         false as _,
    //         3 * size_of::<f32>() as i32,
    //         0 as _,
    //     );
    //     gl::EnableVertexAttribArray(0);

    //     gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    //     gl::BufferData(
    //         gl::ELEMENT_ARRAY_BUFFER,
    //         (indices.len() * size_of::<f32>()) as isize,
    //         indices.as_ptr().cast(),
    //         gl::STATIC_DRAW,
    //     );

    //     gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    // }

    while !window.should_close() {
        process_input(&mut window);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(orange_shader_program);

            gl::BindVertexArray(vaos[0]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::UseProgram(yellow_shader_program);

            gl::BindVertexArray(vaos[1]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // gl::DrawElements(
            //     gl::TRIANGLES,
            //     indices.len() as i32,
            //     gl::UNSIGNED_INT,
            //     0 as _,
            // );

            // gl::BindVertexArray(0);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn framebuffer_size_callback(_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}

fn process_input(window: &mut glfw::Window) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }
}
