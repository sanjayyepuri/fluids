use nalgebra::Vector2;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell; 

use crate::log;

pub struct Gui {
    pub mouse_pressed: bool,

    pub mouse_pos: Vector2<f32>, 
    pub mouse_vec: Vector2<f32>,

    pub width: f32, 
    pub height: f32, 
}


impl Gui {
    pub fn new(width: f32, height: f32) -> Gui {
        Gui {
            mouse_pressed: false, 
            mouse_pos: Vector2::new(0.0, 0.0),
            mouse_vec: Vector2::new(0.0, 0.0), 
            width: width,
            height: height, 
        }
    }

    pub fn set_mouse_down(&mut self, x: f32, y: f32) {
        self.mouse_pos = Vector2::new(x / self.width, 1.0 - y / self.height);
        self.mouse_vec = Vector2::new(0.0, 0.0);

        self.mouse_pressed = true;
    }

    pub fn set_mouse_move(&mut self, x: f32, y: f32) {

        if !self.mouse_pressed {
            return;
        }
        let old_pos = self.mouse_pos;
        self.mouse_pos = Vector2::new(x / self.width, 1.0 - y / self.height);

        self.mouse_vec = self.mouse_pos - old_pos;
    }

    pub fn set_mouse_up(&mut self) {
        self.mouse_pressed = false;
    }
}

fn attach_mouse_down_handler(canvas: &web_sys::HtmlCanvasElement, gui: Rc<RefCell<Gui>>) -> Result<(), JsValue> {
    let handler: Box<dyn FnMut(_)> = Box::new(move |event: web_sys::MouseEvent| {
        let x = event.client_x() as f32;
        let y = event.client_y() as f32; 
        
        gui.borrow_mut().set_mouse_down(x, y);
    });

    let handler = Closure::wrap(handler);
    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_mouse_move_handler(canvas: &web_sys::HtmlCanvasElement, gui: Rc<RefCell<Gui>>) -> Result<(), JsValue> {
    let handler: Box<dyn FnMut(_)> = Box::new(move |event: web_sys::MouseEvent| {
        let x = event.client_x() as f32; 
        let y = event.client_y() as f32;
    
        gui.borrow_mut().set_mouse_move(x, y); 
    });


    let handler = Closure::wrap(handler);
    canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
    handler.forget(); 

    Ok(())
}

fn attach_mouse_up_handler(canvas: &web_sys::HtmlCanvasElement, gui: Rc<RefCell<Gui>>) -> Result<(), JsValue> {
    let handler: Box<dyn FnMut()> = Box::new(move || {
        gui.borrow_mut().set_mouse_up()
    });

    let handler = Closure::wrap(handler);
    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}


pub fn attach_mouse_handlers(canvas: &web_sys::HtmlCanvasElement, gui: Rc<RefCell<Gui>>) -> Result<(), JsValue>{
    attach_mouse_down_handler(&canvas, Rc::clone(&gui))?;
    attach_mouse_move_handler(&canvas, Rc::clone(&gui))?;
    attach_mouse_up_handler(&canvas, Rc::clone(&gui))?;

    Ok(())
} 