extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_focus_polling(true);
    window.set_framebuffer_size_polling(true); // Enable framebuffer size event polling

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    window.make_current();

    while !window.should_close() {
        // Clear the window with a background color
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0); // Set the color (R, G, B, A)
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        window.swap_buffers(); // Swap the front and back buffers
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        glfw::WindowEvent::Focus(focused) => {
            if focused {
                println!("Focused");
            } else {
                println!("Not focused");
            }
        },
        glfw::WindowEvent::FramebufferSize(width, height) => {
            // Adjust the viewport when the window is resized
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
        }
        _ => {}
    }
}