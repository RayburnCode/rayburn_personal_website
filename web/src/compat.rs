#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, RequestMode, Response};

    pub async fn fetch(url: &str) -> Result<String, JsValue> {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(url, &opts)?;
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;
        let text = JsFuture::from(resp.text()?).await?;
        Ok(text.as_string().unwrap())
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use reqwest::Client;

    pub async fn fetch(url: &str) -> Result<String, reqwest::Error> {
        Client::new().get(url).send().await?.text().await
    }
}