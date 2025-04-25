use ferlory_components::STATE;
use gloo::timers::callback::Timeout;

pub fn notify_send(content: Option<&'static str>, timeout_ms: Option<u32>) {
    STATE.write().hidden = false;
    if let Some(content) = content {
        STATE.write().content = content;
    }
    match timeout_ms {
        None => STATE.write().timeout_handle = None,
        Some(timeout_ms) => {
            let handle = Timeout::new(timeout_ms, || {
                STATE.write().hidden = true;
            });
            STATE.write().timeout_handle = Some(handle)
        }
    }
}
