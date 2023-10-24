use std::rc::Rc;
use std::sync::Mutex;
use anyhow::Result;
use web_sys::Event;

use crate::browser;
use crate::engine::Button;
use super::Editor;

async fn forward_event_inner(editor: Rc<Mutex<Editor>>, id: u16) -> Result<()> {
    if !Editor::try_run_id(&editor, id) {
        forward_event(editor, id)?;
        return Ok(());
    }
    let mut editor = Editor::lock(&editor)?;
    let result = editor.draw_image_data().await;
    editor.set_disabled(false);
    editor.to_idle();
    result
}

fn forward_event(editor: Rc<Mutex<Editor>>, id: u16) -> Result<()> {
    browser::set_callback_once(move || {
        browser::spawn_local(async move {
            if let Err(err) = forward_event_inner(editor, id).await {
                error!("{:#?}", err);
            }
        });
    })
}

fn setup_forward_event_closure(editor: Rc<Mutex<Editor>>) -> Result<()> {
    let id = if let Some(id) = Editor::try_run(&editor) {
        id
    } else {
        return Ok(());
    };
    {
        let mut editor = Editor::lock(&editor)?;
        if let Some(_) = editor.redo() {
            editor.set_disabled(true);
        } else {
            editor.to_idle();
            return Ok(());
        }
    }
    forward_event(editor, id)
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

async fn backward_event_inner(editor: Rc<Mutex<Editor>>, id: u16) -> Result<()> {
    if !Editor::try_run_id(&editor, id) {
        backward_event(editor, id)?;
        return Ok(());
    }
    let mut editor = Editor::lock(&editor)?;
    let result = editor.draw_image_data().await;
    editor.set_disabled(false);
    editor.to_idle();
    result
}

fn backward_event(editor: Rc<Mutex<Editor>>, id: u16) -> Result<()> {
    browser::set_callback_once(move || {
        browser::spawn_local(async move {
            if let Err(err) = backward_event_inner(editor, id).await {
                error!("{:#?}", err);
            }
        });
    })
}

fn setup_backward_event_closure(editor: Rc<Mutex<Editor>>) -> Result<()> {
    let id = if let Some(id) = Editor::try_run(&editor) {
        id
    } else {
        return Ok(());
    };
    {
        let mut editor = Editor::lock(&editor)?;
        if let Some(_) = editor.undo() {
            editor.set_disabled(true);
        } else {
            editor.to_idle();
            return Ok(());
        }
    }
    backward_event(editor, id)
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
