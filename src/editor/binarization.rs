use std::rc::Rc;
use std::sync::Mutex;
use anyhow::Result;
use web_sys::Event;

use crate::browser;
use crate::engine::Button;
use super::Editor;

#[derive(Debug, Clone)]
struct Temp {
    pub index: usize,
    pub max_index: usize,
}

fn binarization_step(image_data: &mut [u8], temp: &mut Temp, step: usize) -> bool {
    for i in 0..step {
        let index = temp.index + i;
        if index >= temp.max_index {
            temp.index = temp.max_index;
            return true;
        }
        if index % 4 == 3 {
            continue;
        }
        if image_data[index] > 128 {
            image_data[index] = 255;
        } else {
            image_data[index] = 0;
        }
    }
    temp.index += step;
    false
}

fn binarization_step_thread(editor: Rc<Mutex<Editor>>, button_element: &Button, temp: &mut Temp, step: usize) -> Result<()> {
    let mut continue_flag = false;
    match editor.try_lock() {
        Ok(mut editor) => {
            if let Some(image_data) = editor.get_image_data_mut() {
                if binarization_step(image_data, temp, step) {
                    editor.set_image_data()?;
                    editor.draw_image_data()?;
                    button_element.set_disabled(false);
                } else {
                    continue_flag = true;
                }
            } else {
                log!("No image data");
            }
        },
        Err(_) => {
            log!("Editor is locked");
        },
    }
    if continue_flag {
        let button_element = button_element.clone();
        let mut temp = temp.clone();
        let closure = browser::closure_once(move || {
            browser::spawn_local(async move {
                if let Err(err) = binarization_step_thread(editor, &button_element, &mut temp, step) {
                    error!("{:#?}", err);
                }
            });
        });
        browser::set_timeout_with_callback(
            &browser::window()?,
            closure,
        )?;
    }
    Ok(())
}

fn first_step(editor: Rc<Mutex<Editor>>, button_element: &Button) -> Result<()>{
    let mut temp = Temp {
        index: 0,
        max_index: 0,
    };
    match editor.try_lock() {
        Ok(mut editor) => {
            if let Some(image_data) = editor.get_image_data_mut() {
                temp.max_index = image_data.len();
            } else {
                log!("No image data");
            }
        },
        Err(_) => {
            log!("Editor is locked");
        },
    }
    if temp.max_index > 0 {
        binarization_step_thread(editor, button_element, &mut temp, 1000000)?;
    } else {
        button_element.set_disabled(false);
    }
    Ok(())
}

fn setup_binarization_event_closure(editor: Rc<Mutex<Editor>>, event: Event) -> Result<()> {
    let button_element = Button::new_from_event(&event)?;
    button_element.set_disabled(true);
    let closure = browser::closure_once(move || {
        let editor_clone = editor.clone();
        let button_element = button_element.clone();
        browser::spawn_local(async move {
            if let Err(err) = first_step(editor_clone, &button_element) {
                error!("{:#?}", err);
            }
        });
    });
    browser::set_timeout_with_callback(
        &browser::window()?,
        closure,
    )?;
    Ok(())
}

pub fn setup_binarization_event(editor: Rc<Mutex<Editor>>) -> Result<()> {
    let button_element = Button::new_from_id("binarization")?;

    let closure = browser::create_event_closure(move |event: Event| {
        let editor_clone = editor.clone();
        if let Err(err) = setup_binarization_event_closure(editor_clone, event) {
            error!("{:#?}", err);
        }
    });

    button_element.add_event_listener_with_callback(&closure)?;
    closure.forget();

    Ok(())
}
