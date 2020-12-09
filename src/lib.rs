use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{
    console, window, Document,
    HtmlCanvasElement, CanvasRenderingContext2d,
    MouseEvent
};
use wasm_bindgen::{JsCast,JsValue};

fn draw_knob (c: &CanvasRenderingContext2d , x: f64, y: f64, r: f64) {
    c.begin_path();
    c.arc(x, y, r, 0.0, f64::consts::PI * 2.0).unwrap();
    c.stroke();

    c.begin_path();
    c.move_to(x, y);
    c.line_to(x, y - r);
    c.stroke();
}

fn draw_fader (c: &CanvasRenderingContext2d , x: f64, y: f64, h: f64) {
    c.stroke_rect(x, y, 1.0, h);
    c.stroke_rect(x - 15.0, y - 7.5, 30.0, 15.0);
}

fn draw_deck (c: &HtmlCanvasElement) {

    c.set_width(555);
    c.set_height(210);

    let context = c
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    context.translate(-0.5, -0.5).unwrap();
    context.set_text_baseline("middle");
    context.set_font("bold 15px sans-serif");

    context.set_fill_style(&JsValue::from_str("#111"));
    context.fill_rect(0.0, 0.0, 640.0, 400.0);

    context.set_line_width(1.0);
    context.set_stroke_style(&JsValue::from_str("#aaa"));

    draw_knob(&context, 112.5, 26.5, 10.0);
    draw_knob(&context, 75.0, 30.0, 15.0);
    draw_knob(&context, 75.0, 75.0, 15.0);
    draw_knob(&context, 75.0, 120.0, 15.0);

    draw_fader(&context, 30.0, 30.0, 90.0);
    draw_fader(&context, 525.0, 30.0, 90.0);

    context.set_fill_style(&JsValue::from_str("#fff"));
    context.set_text_align("left");
    context.fill_text("Song Title", 135.0, 27.0).unwrap();
    context.set_text_align("right");
    context.fill_text("00:00", 495.0, 27.0).unwrap();

    context.stroke_rect(105.0, 52.5, 390.0, 90.0);

    context.stroke_rect(105.0, 150.0, 45.0, 45.0);
    context.stroke_rect(155.0, 150.0, 45.0, 45.0);
    context.stroke_rect(205.0, 150.0, 45.0, 45.0);
}

macro_rules! on {
    (
        $element:    ident,
        $event_name: literal,
        $event_type: ty,
        $event_id:   ident,
        $handler:    block
    ) => {
        let closure = Closure::wrap(Box::new(move |$event_id: $event_type| $handler)
            as Box<dyn FnMut(_)>);
        $element.add_event_listener_with_callback(
            $event_name,
            closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    };
}

fn init_deck (d: &Document) {
    let canvas = d.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ()).unwrap();
    draw_deck(&canvas);
    on!(canvas, "mousemove", MouseEvent, event, {
        let x = event.x();
        let y = event.y();
        console::log_2(
            &JsValue::from_f64(x as f64),
            &JsValue::from_f64(y as f64)
        );
    });
    on!(canvas, "mousedown", MouseEvent, event, {
        let x = event.x();
        let y = event.y();
        console::log_3(
            &JsValue::from_str("down"),
            &JsValue::from_f64(x as f64),
            &JsValue::from_f64(y as f64)
        );
    });
    on!(canvas, "mouseup", MouseEvent, event, {
        let x = event.x();
        let y = event.y();
        console::log_3(
            &JsValue::from_str("up"),
            &JsValue::from_f64(x as f64),
            &JsValue::from_f64(y as f64)
        );
    });
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = window().unwrap().document().unwrap();
    init_deck(&document);
}
