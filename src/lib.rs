use std::rc::Rc;
use std::cell::Cell;
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
    //canvas_container.set_text_content(Some("this is content of canvas"));
    console_log!("rust: create canvas: {:?}", canvas_container);

    let canvas = document
        .create_element("canvas")
        .expect("fail to create new element")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .expect("fail to cast canvas to canvas element");

    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid").expect("fail to set border");
    console_log!("create canvas: {:?}", canvas);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("fail to cast to context");

    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .expect("fail to add listener mousedown");
        closure.forget();
    }

    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if  pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .expect("fail to add listener mousemove");
        closure.forget();
    }

    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
            .expect("fail to add listener mouseup");
        closure.forget();
    }
    
    canvas_container.append_child(&canvas).expect("fail to append child canvas");
    return canvas_container;
}


