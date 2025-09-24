use wasm_bindgen::prelude::*;
use web_sys::{ console, window };
use url::Url;

#[wasm_bindgen]
pub fn execute(url: &str) -> Result<(), JsValue> {
    let blocked_list = get_blocked_sites();
    console::log_1(&url.to_string().into());

    if is_blocked(url, &blocked_list) {
        blocked_site()?;
    }

    Ok(())
}

/////////////////////

fn get_blocked_sites() -> Vec<&'static str> {
    vec!["www.facebook.com", "twitter.com", "www.instagram.com", "www.tiktok.com"]
}

fn is_blocked(url: &str, blocked_list: &[&str]) -> bool {
    match Url::parse(url) {
        Ok(paser_url) => {
            let host_src = paser_url.host_str().unwrap_or("");

            if blocked_list.contains(&host_src) {
                return true;
            }
            return false;
        }
        Err(_) => {
            return false;
        }
    }
}

fn blocked_site() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    body.set_inner_html("");

    let div = document.create_element("div")?;
    div.set_id("blocker-div");
    div.set_inner_html("<h1>Blocked Access</h1>");

    body.append_child(&div)?;

    console::log_1(&"fim da funcao de overlay".into());

    Ok(())
}
