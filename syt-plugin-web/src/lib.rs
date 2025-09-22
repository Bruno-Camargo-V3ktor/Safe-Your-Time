use wasm_bindgen::prelude::*;
use web_sys::{ HtmlElement, window };

#[wasm_bindgen]
pub fn execute(url: &str) -> Result<(), JsValue> {
    let blocked_list = get_blocked_sites();
    if is_blocked(url, &blocked_list) {
        create_block_overlay()?;
    }
    Ok(())
}

#[wasm_bindgen]
pub fn is_url_blocked(url: &str) -> bool {
    let blocked_list = get_blocked_sites();
    is_blocked(url, &blocked_list)
}

/////////////////////

fn get_blocked_sites() -> Vec<&'static str> {
    vec!["facebook.com", "twitter.com", "instagram.com", "tiktok.com"]
}

fn is_blocked(url: &str, blocked_list: &[&str]) -> bool {
    for site in blocked_list {
        if url.contains(site) {
            return true;
        }
    }
    false
}

fn create_block_overlay() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    if document.get_element_by_id("site-blocker-overlay").is_some() {
        return Ok(());
    }

    let overlay = document.create_element("div")?;
    overlay.set_id("site-block-overlay");
    overlay.set_inner_html(r#"
        <h1>Site Blocked</h1>
    "#);

    body.append_child(&overlay)?;
    let _ = window.stop();

    let body_html: HtmlElement = body.dyn_into()?;
    body_html.style().set_property("overflow", "hidden")?;

    Ok(())
}
