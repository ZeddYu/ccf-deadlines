use std::future::Future;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, closure::Closure};
use wasm_bindgen_futures::spawn_local;

pub fn focus_element_by_id_after_render(id: String) {
    #[cfg(target_arch = "wasm32")]
    schedule_callback_after_delay(move || focus_element_by_id(&id), 0);

    #[cfg(not(target_arch = "wasm32"))]
    let _ = id;
}

const NON_CRITICAL_REQUEST_DELAY_MS: i32 = 1500;

pub fn schedule_non_critical_request(callback: impl FnOnce() + 'static) {
    schedule_callback_after_delay(callback, NON_CRITICAL_REQUEST_DELAY_MS);
}

pub fn schedule_non_critical_async_request(future: impl Future<Output = ()> + 'static) {
    schedule_non_critical_request(move || spawn_local(future));
}

pub fn schedule_callback_after_delay(callback: impl FnOnce() + 'static, delay_ms: i32) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let callback = Closure::once_into_js(callback);
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                delay_ms,
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = callback;
        let _ = delay_ms;
    }
}

#[cfg(target_arch = "wasm32")]
fn focus_element_by_id(id: &str) {
    let Some(element) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id(id))
        .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok())
    else {
        return;
    };

    let _ = element.focus();
}
