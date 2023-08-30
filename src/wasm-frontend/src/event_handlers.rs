use std::{cell::RefCell, rc::Rc};

use casserole_core::{graphics::Position, platform::Platform, ui_library::Widget, AppRunner};
use wasm_bindgen::{prelude::Closure, JsCast};

pub fn register_event_listeners<PlatformType: Platform + 'static, AppType: Widget + 'static>(
    app_runner: Rc<RefCell<AppRunner<PlatformType, AppType>>>,
) {
    let window = web_sys::window().unwrap();

    let resize_ar: Rc<RefCell<AppRunner<PlatformType, AppType>>> = app_runner.clone();
    let resize_closure: Closure<dyn Fn(_)> = Closure::new(move |_: web_sys::InputEvent| {
        resize_ar.borrow_mut().on_window_resize();
    });
    window
        .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())
        .unwrap();
    resize_closure.forget();

    let beforeunload_ar: Rc<RefCell<AppRunner<PlatformType, AppType>>> = app_runner.clone();
    let beforeunload_closure: Closure<dyn Fn(_)> = Closure::new(move |_: web_sys::InputEvent| {
        beforeunload_ar.clone().borrow_mut().on_quit();
    });
    window
        .add_event_listener_with_callback(
            "beforeunload",
            beforeunload_closure.as_ref().unchecked_ref(),
        )
        .unwrap();
    beforeunload_closure.forget();

    let mousemove_ar: Rc<RefCell<AppRunner<PlatformType, AppType>>> = app_runner.clone();
    let mousemove_closure: Closure<dyn Fn(_)> = Closure::new(move |event: web_sys::InputEvent| {
        mousemove_ar.clone().borrow_mut().on_mouse_move(Position {
            x: event.page_x() as f64,
            y: event.page_y() as f64,
        });
    });
    window
        .add_event_listener_with_callback("mousemove", mousemove_closure.as_ref().unchecked_ref())
        .unwrap();
    mousemove_closure.forget();
}
