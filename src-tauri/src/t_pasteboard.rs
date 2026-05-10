/// Cross‑platform drag‑drop image URL extraction.
///
/// - **macOS**: reads `NSPasteboardNameDrag` which is scoped to the
///   current drag operation and never returns stale data.
/// - **Windows / Linux**: falls back to the system clipboard.  During
///   a browser drag the image URL is normally placed on the clipboard
///   as well, so checking it immediately after the drop event gives
///   the correct URL.  The read is gated behind an `http(s)://` prefix
///   check to reject irrelevant clipboard text.

#[cfg(target_os = "macos")]
mod imp {
    use std::ffi::{c_char, CStr};

    unsafe extern "C" {
        fn lap_get_drag_image_url() -> *const c_char;
        fn lap_free_string(ptr: *const c_char);
    }

    pub fn get_drag_image_url() -> Option<String> {
        let ptr = unsafe { lap_get_drag_image_url() };
        if ptr.is_null() {
            return None;
        }
        let url = unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string();
        unsafe { lap_free_string(ptr) };
        Some(url)
    }
}

#[cfg(not(target_os = "macos"))]
mod imp {
    pub fn get_drag_image_url() -> Option<String> {
        // Browsers on Windows put the image URL on the system clipboard
        // during a drag, so reading it right after the drop is reliable.
        let mut clipboard = arboard::Clipboard::new().ok()?;
        let text = clipboard.get_text().ok()?;
        let text = text.trim();
        if text.starts_with("http://") || text.starts_with("https://") {
            Some(text.to_string())
        } else {
            None
        }
    }
}

pub fn get_drag_image_url() -> Option<String> {
    imp::get_drag_image_url()
}
