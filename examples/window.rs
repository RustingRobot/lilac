use lilac;

fn  main() {
    let mut window = lilac::init();
    println!("this is a test");
    while !window.exit() {
        window.update();
        window.render();
    }
}