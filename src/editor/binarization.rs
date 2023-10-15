use std::rc::Rc;
use std::sync::Mutex;
use anyhow::Result;
use wasm_bindgen::JsCast;
use web_sys::{
    Event, HtmlInputElement,
};

use crate::browser;
use super::Editor;

fn binarization(image_data: &mut [u8]) {
    for i in 0..image_data.len() {
        if i % 4 == 3 {
            continue;
        }
        if image_data[i] > 128 {
            image_data[i] = 255;
        } else {
            image_data[i] = 0;
        }
    }
}

fn handle_binarization(editor: Rc<Mutex<Editor>>) -> Result<()> {
    match editor.try_lock() {
        Ok(mut editor) => {
            if let Some(image_data) = editor.get_image_data_mut() {
                binarization(image_data);
                editor.set_image_data()?;
                editor.draw_image_data()?;
            } else {
                log!("No image data");
            }
        },
        Err(_) => {
            log!("Editor is locked");
        },
    }
    Ok(())
}

fn binarization_thread(editor: Rc<Mutex<Editor>>, input_element: &HtmlInputElement) -> Result<()> {
    handle_binarization(editor)?;
    input_element.set_disabled(false);
    Ok(())
}

fn setup_binarization_event_closure(editor: Rc<Mutex<Editor>>, event: Event) -> Result<()> {
    let input_element = browser::event_current_target(&event)?;
    if input_element.checked() {
        log!("Checked");
        input_element.set_disabled(true);
        let closure = browser::closure_once(move || {
            let editor_clone = editor.clone();
            let input_element = input_element.clone();
            browser::spawn_local(async move {
                if let Err(err) = binarization_thread(editor_clone, &input_element) {
                    error!("{:#?}", err);
                }
            });
        });
        browser::set_timeout_with_callback(
            &browser::window()?,
            closure,
        )?;
        log!("Checked end");
    } else {
        log!("Not Checked");
        match editor.try_lock() {
            Ok(_) => {},
            Err(_) => {
                log!("Editor is locked");
            },
        }
    }
    Ok(())
}

pub fn setup_binarization_event(editor: Rc<Mutex<Editor>>) -> Result<()> {
    let input_element = browser::input("test")?;

    let closure = browser::create_event_closure(move |event: Event| {
        let editor_clone = editor.clone();
        if let Err(err) = setup_binarization_event_closure(editor_clone, event) {
            error!("{:#?}", err);
        }
    });

    input_element.set_onchange(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    Ok(())
}
