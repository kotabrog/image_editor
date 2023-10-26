use anyhow::Result;
use web_sys::{Event, MediaQueryList};

use crate::browser;
use crate::engine::Button;

fn setup_theme_button_event_inner() -> Result<()> {
    let data_theme = browser::get_attribute_from_body("data-theme")?;
    if data_theme == Some("dark".to_string()) {
        browser::set_attribute_to_body("data-theme", "light")?;
    } else {
        browser::set_attribute_to_body("data-theme", "dark")?;
    }
    Ok(())
}

fn setup_theme_button_event() -> Result<()>{
    let button_element = Button::new_from_id("theme")?;

    let closure = browser::create_event_closure(move |_event: Event| {
        if let Err(err) = setup_theme_button_event_inner() {
            error!("{:#?}", err);
        }
    });

    button_element.add_event_listener_with_callback(&closure)?;
    closure.forget();

    Ok(())
}

fn set_theme(media_query_list: MediaQueryList) -> Result<()> {
    if media_query_list.matches() {
        browser::set_attribute_to_body("data-theme", "dark")?;
    } else {
        browser::set_attribute_to_body("data-theme", "light")?;
    }
    Ok(())
}

fn setup_prefers_color_scheme_event_inner(event: Event) -> Result<()> {
    let media_query_list = browser::event_current_target_to_media_query_list(&event)?;
    set_theme(media_query_list)
}

fn setup_prefers_color_scheme_event() -> Result<()> {
    let media_query_list = browser::get_color_scheme_media_query_list()?;
    let closure = browser::create_event_closure(move |event: Event| {
        if let Err(err) = setup_prefers_color_scheme_event_inner(event) {
            error!("{:#?}", err);
        }
    });

    browser::add_listener_with_opt_callback(
        &media_query_list,
        Some(&closure),
    )?;
    closure.forget();

    Ok(())
}

pub fn setup_theme_event() -> Result<()> {
    setup_prefers_color_scheme_event()?;
    setup_theme_button_event()?;
    Ok(())
}
