use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{MediaQueryList, Event};
use super::window;

pub fn make_media_query_list(query: &str) -> Result<Option<MediaQueryList>> {
    window()?
        .match_media(query)
        .map_err(|err| anyhow!("Error creating media query list {:#?}", err))
}

pub fn get_color_scheme_media_query_list() -> Result<MediaQueryList> {
    let media_query_list = make_media_query_list("(prefers-color-scheme: dark)")?;
    if let Some(media_query_list) = media_query_list {
        Ok(media_query_list)
    } else {
        Err(anyhow!("No media query list found"))
    }
}

pub fn add_listener_with_opt_callback(
    media_query_list: &MediaQueryList,
    closure: Option<&super::EventClosure>,
) -> Result<()> {
    media_query_list
        .add_listener_with_opt_callback(closure.map(
            |closure| closure.as_ref().unchecked_ref()
        )).map_err(|err| anyhow!("Error adding listener with opt callback {:#?}", err))
}

pub fn event_current_target_to_media_query_list(event: &Event) -> Result<MediaQueryList> {
    event.current_target()
        .ok_or_else(|| anyhow!("No current target found"))?
        .dyn_into::<MediaQueryList>()
        .map_err(|element| anyhow!("Error converting {:#?} to MediaQueryList", element))
}
