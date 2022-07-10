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

    let canvas_container = document.create_element("div").expect("fail to create new element");
    canvas_container.set_text_content(Some("this is content of canvas"));
    console_log!("rust: create canvas: {:?}", canvas_container);

    let canvas = document
        .create_element("canvas")
        .expect("fail to create new element")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .expect("fail to cast canvas to canvas element");
    console_log!("create another div: {:?}", canvas);

     
    
    return canvas_container;
}


