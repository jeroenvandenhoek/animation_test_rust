use web_sys;

fn main() {
    let window = web_sys::window().expect("no window found");
    let animation = |window: &web_sys::Window| {
        window.request_animation_frame(&animation);
    };

    animation(&window);
    println!("Hello, world!");
}

