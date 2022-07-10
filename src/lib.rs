use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    console_log!("{}", s);
    alert(s);
}



#[wasm_bindgen]
pub fn start() -> web_sys::Element {
    console_log!("this is start function, wtf! let me run please");

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let canvas = document.create_element("div").expect("fail to create new element");
    canvas.set_text_content(Some("this is content of canvas"));
    console_log!("rust: create canvas: {:?}", canvas);

//let c = document
//    .create_element("div")
//    .expect("fail to create new element")
//    .dyn_into::<web_sys::HtmlCanvasElement>()
//    .map_err(|_| ())
//    .unwrap();

    // console_log!("rust: dyn_into: {:?}", c);
    return canvas;


    
     
// let context = canvas
//     .get_context("2d")
//     .unwrap()
//     .unwrap()
//     .dyn_into::<web_sys::CanvasRenderingContext2d>()
//     .unwrap();
// 
// context.begin_path();
// 
// // Draw the outer circle.
// context
//     .arc(75.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0)
//     .unwrap();
// 
// // Draw the mouth.
// context.move_to(110.0, 75.0);
// context.arc(75.0, 75.0, 35.0, 0.0, std::f64::consts::PI).unwrap();
// 
// // Draw the left eye.
// context.move_to(65.0, 65.0);
// context
//     .arc(60.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
//     .unwrap();
// 
// // Draw the right eye.
// context.move_to(95.0, 65.0);
// context
//     .arc(90.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
//     .unwrap();
// 
// context.stroke();

}


