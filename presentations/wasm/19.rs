extern crate webplatform;

fn main() {
    let document = webplatform::init();
    let body = document.element_query("body")
        .unwrap();
    body.html_append("\
        <h1>This header brought to you by Rust</h1>\
        <button>Click me!</button>\
    ");
    
    let button = document.element_query("button")
        .unwrap();
    button.on("mouseenter", move |_| {
        println!("Mouse entered!");
        body.html_append("<p>Mouse entered!</p>");
    });
    button.on("click", |_| {
        println!("Clicked!");
        webplatform::alert("Clicked!");
    });

    webplatform::spin();
}