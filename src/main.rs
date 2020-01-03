use web_sys;

fn main() {
    animation();
    println!("Hello, world!");
}

fn animation(){
    web_sys::window().expect("no window found").request_animation_frame(&animation);
}
