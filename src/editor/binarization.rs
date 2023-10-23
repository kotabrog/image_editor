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

fn final_step(editor: Rc<Mutex<Editor>>) -> Result<()> {
    browser::set_callback_once(move || {
        browser::spawn_local(async move {
            if let Some(editor) = Editor::try_lock(&editor) {
                if let Err(err) = editor.draw_image_data().await {
                    error!("{:#?}", err);
                }
                editor.set_disabled(false);
            }
        });
    })
}

fn binarization_step_thread(editor: Rc<Mutex<Editor>>, button_element: Button, mut temp: Temp, step: usize) -> Result<()> {
    let mut continue_flag = false;
    if let Some(mut editor) = Editor::try_lock(&editor) {
        if let Some(image_data) = editor.get_image_data_mut() {
            if binarization_step(image_data, &mut temp, step) {
                editor.data_to_image_data()?;
            } else {
                continue_flag = true;
            }
        } else {
            log!("No image data");
        }
    }
    if continue_flag {
        browser::set_callback_once(move || {
            browser::spawn_local(async move {
                if let Err(err) = binarization_step_thread(editor, button_element, temp, step) {
                    error!("{:#?}", err);
                }
            });
        })?;
    } else {
        final_step(editor)?
    }
    Ok(())
}

fn set_disabled_false(editor: Rc<Mutex<Editor>>) -> Result<()> {
    if let Some(editor) = Editor::try_lock(&editor) {
        editor.set_disabled(false);
    } else {
        let editor = editor.clone();
        browser::set_callback_once(move || {
            browser::spawn_local(async move {
                if let Err(err) = set_disabled_false(editor) {
                    error!("{:#?}", err);
                }
            });
        })?;
    }
    Ok(())
}

fn first_step(editor: Rc<Mutex<Editor>>, button_element: Button) -> Result<()>{
    let mut temp = Temp {
        index: 0,
        max_index: 0,
    };
    if let Some(mut editor) = Editor::try_lock(&editor) {
        if editor.have_image_data() {
            editor.clone_push();
            if let Some(image_data) = editor.get_image_data() {
                temp.max_index = image_data.len();
            }
        } else {
            log!("No image data");
        }
    }
    if temp.max_index > 0 {
        binarization_step_thread(editor, button_element, temp, 1000000)?;
    } else {
        set_disabled_false(editor)?;
    }
    Ok(())
}

fn setup_binarization_event_closure(editor: Rc<Mutex<Editor>>, event: Event) -> Result<()> {
    let button_element = Button::new_from_event(&event)?;
    if let Some(editor) = Editor::try_lock(&editor) {
        editor.set_disabled(true);
    } else {
        return Ok(())
    }
    browser::set_callback_once(move || {
        browser::spawn_local(async move {
            if let Err(err) = first_step(editor, button_element) {
                error!("{:#?}", err);
            }
        });
    })
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
