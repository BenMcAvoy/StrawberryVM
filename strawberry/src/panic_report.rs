use std::sync::{Mutex, OnceLock};

static LAST_STATUS: OnceLock<Mutex<String>> = OnceLock::new();

pub fn set_last_status(status: String) {
    let lock = LAST_STATUS.get_or_init(|| Mutex::new(String::new()));

    match lock.lock() {
        Ok(mut guard) => *guard = status,
        Err(poisoned) => {
            // If the mutex is poisoned due to a previous panic, still try to record.
            *poisoned.into_inner() = status;
        }
    }
}

pub fn get_last_status() -> Option<String> {
    let lock = LAST_STATUS.get()?;

    match lock.lock() {
        Ok(guard) => Some(guard.clone()),
        Err(poisoned) => Some(poisoned.into_inner().clone()),
    }
}
