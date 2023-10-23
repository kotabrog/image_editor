use std::rc::Rc;
use std::sync::Mutex;
use anyhow::Result;
use web_sys::Event;

use crate::browser;
use crate::engine::Button;
use super::Editor;


fn setup_forward_event_closure(editor: Rc<Mutex<Editor>>) -> Result<()> {
    let mut forward_flag = false;
    if let Some(mut editor) = Editor::try_lock(&editor) {
        if let Some(_) = editor.redo() {
            editor.set_disabled(true);
            forward_flag = true;
        }
    }
    if !forward_flag {
        return Ok(());
    }
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

pub fn setup_forward_event(editor: Rc<Mutex<Editor>>) -> Result<()> {
    let button_element = Button::new_from_id("forward")?;

    let closure = browser::create_event_closure(move |_event: Event| {
        let editor_clone = editor.clone();
        if let Err(err) = setup_forward_event_closure(editor_clone) {
            error!("{:#?}", err);
        }
    });

    button_element.add_event_listener_with_callback(&closure)?;
    closure.forget();

    Ok(())
}

fn setup_backward_event_closure(editor: Rc<Mutex<Editor>>) -> Result<()> {
    let mut backward_flag = false;
    if let Some(mut editor) = Editor::try_lock(&editor) {
        if let Some(_) = editor.undo() {
            editor.set_disabled(true);
            backward_flag = true;
        }
    }
    if !backward_flag {
        return Ok(());
    }
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

pub fn setup_backward_event(editor: Rc<Mutex<Editor>>) -> Result<()> {
    let button_element = Button::new_from_id("back")?;

    let closure = browser::create_event_closure(move |_event: Event| {
        let editor_clone = editor.clone();
        if let Err(err) = setup_backward_event_closure(editor_clone) {
            error!("{:#?}", err);
        }
    });

    button_element.add_event_listener_with_callback(&closure)?;
    closure.forget();

    Ok(())
}

pub fn setup_back_and_forward_event(editor: Rc<Mutex<Editor>>) -> Result<()> {
    setup_backward_event(editor.clone())?;
    setup_forward_event(editor.clone())?;
    Ok(())
}
