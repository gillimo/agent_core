//! Screen capture module - Fast screen/window capture

use anyhow::Result;
use xcap::{Monitor, Window};
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::{IsIconic, SetForegroundWindow, ShowWindow, SW_RESTORE};

/// Capture a region of the screen, returns RGBA bytes
pub fn capture_region_impl(x: i32, y: i32, width: u32, height: u32) -> Result<Vec<u8>> {
    let monitors = Monitor::all().map_err(|e| anyhow::anyhow!("{}", e))?;
    let monitor = monitors.into_iter().next()
        .ok_or_else(|| anyhow::anyhow!("No monitor found"))?;
    let full_image = monitor.capture_image().map_err(|e| anyhow::anyhow!("{}", e))?;
    let cropped = image::DynamicImage::ImageRgba8(full_image)
        .crop_imm(x as u32, y as u32, width, height);
    Ok(cropped.to_rgba8().into_raw())
}

/// Capture full screen, returns (width, height, rgba_bytes)
pub fn capture_full_screen_impl() -> Result<(u32, u32, Vec<u8>)> {
    let monitors = Monitor::all().map_err(|e| anyhow::anyhow!("{}", e))?;
    let monitor = monitors.into_iter().next()
        .ok_or_else(|| anyhow::anyhow!("No monitor found"))?;
    let image = monitor.capture_image().map_err(|e| anyhow::anyhow!("{}", e))?;
    let width = image.width();
    let height = image.height();
    let bytes = image.into_raw();
    Ok((width, height, bytes))
}

/// List all visible windows, returns [(id, title), ...]
pub fn list_windows_impl() -> Result<Vec<(u32, String)>> {
    let windows = Window::all().map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(windows.iter().map(|w| (w.id(), w.title().to_string())).collect())
}

/// Find window by title (partial match), returns (id, title, x, y, width, height)
pub fn find_window_impl(title_contains: &str) -> Result<Option<(u32, String, i32, i32, u32, u32)>> {
    let windows = Window::all().map_err(|e| anyhow::anyhow!("{}", e))?;
    let search = title_contains.to_lowercase();
    for w in windows {
        if w.title().to_lowercase().contains(&search) {
            return Ok(Some((
                w.id(),
                w.title().to_string(),
                w.x(),
                w.y(),
                w.width(),
                w.height()
            )));
        }
    }
    Ok(None)
}

/// Find window by multiple title fragments (all must match), returns (id, title, x, y, width, height)
pub fn find_window_by_all_impl(title_parts: &[String]) -> Result<Option<(u32, String, i32, i32, u32, u32)>> {
    let windows = Window::all().map_err(|e| anyhow::anyhow!("{}", e))?;
    let parts: Vec<String> = title_parts.iter().map(|p| p.to_lowercase()).collect();
    for w in windows {
        let title = w.title().to_lowercase();
        if parts.iter().all(|p| title.contains(p)) {
            return Ok(Some((
                w.id(),
                w.title().to_string(),
                w.x(),
                w.y(),
                w.width(),
                w.height()
            )));
        }
    }
    Ok(None)
}

/// Capture a specific window by ID, returns (width, height, rgba_bytes)
pub fn capture_window_impl(window_id: u32) -> Result<(u32, u32, Vec<u8>)> {
    let windows = Window::all().map_err(|e| anyhow::anyhow!("{}", e))?;
    let window = windows.into_iter().find(|w| w.id() == window_id)
        .ok_or_else(|| anyhow::anyhow!("Window not found: {}", window_id))?;
    let image = window.capture_image().map_err(|e| anyhow::anyhow!("{}", e))?;
    let width = image.width();
    let height = image.height();
    let bytes = image.into_raw();
    Ok((width, height, bytes))
}

/// Capture window by title (partial match), returns (width, height, rgba_bytes)
pub fn capture_window_by_title_impl(title_contains: &str) -> Result<(u32, u32, Vec<u8>)> {
    let windows = Window::all().map_err(|e| anyhow::anyhow!("{}", e))?;
    let search = title_contains.to_lowercase();
    let window = windows.into_iter().find(|w| w.title().to_lowercase().contains(&search))
        .ok_or_else(|| anyhow::anyhow!("Window not found: {}", title_contains))?;
    let image = window.capture_image().map_err(|e| anyhow::anyhow!("{}", e))?;
    let width = image.width();
    let height = image.height();
    let bytes = image.into_raw();
    Ok((width, height, bytes))
}

/// Capture window by multiple title fragments (all must match), returns (width, height, rgba_bytes)
pub fn capture_window_by_all_impl(title_parts: &[String]) -> Result<(u32, u32, Vec<u8>)> {
    let window = find_window_by_all_impl(title_parts)?
        .ok_or_else(|| anyhow::anyhow!("Window not found: {:?}", title_parts))?;
    let windows = Window::all().map_err(|e| anyhow::anyhow!("{}", e))?;
    let window = windows.into_iter().find(|w| w.id() == window.0)
        .ok_or_else(|| anyhow::anyhow!("Window not found: {:?}", title_parts))?;
    let image = window.capture_image().map_err(|e| anyhow::anyhow!("{}", e))?;
    let width = image.width();
    let height = image.height();
    let bytes = image.into_raw();
    Ok((width, height, bytes))
}

/// Focus a window by title (partial match). Returns window metadata.
pub fn focus_window_by_title_impl(title_contains: &str) -> Result<(u32, String, i32, i32, u32, u32)> {
    let window = find_window_impl(title_contains)?
        .ok_or_else(|| anyhow::anyhow!("Window not found: {}", title_contains))?;
    let hwnd = window.0 as usize as HWND;
    unsafe {
        if IsIconic(hwnd) != 0 {
            ShowWindow(hwnd, SW_RESTORE);
        }
        if SetForegroundWindow(hwnd) == 0 {
            return Err(anyhow::anyhow!("Failed to focus window: {}", title_contains));
        }
    }
    Ok(window)
}

/// Focus a window by multiple title fragments (all must match). Returns window metadata.
pub fn focus_window_by_all_impl(title_parts: &[String]) -> Result<(u32, String, i32, i32, u32, u32)> {
    let window = find_window_by_all_impl(title_parts)?
        .ok_or_else(|| anyhow::anyhow!("Window not found: {:?}", title_parts))?;
    let hwnd = window.0 as usize as HWND;
    unsafe {
        if IsIconic(hwnd) != 0 {
            ShowWindow(hwnd, SW_RESTORE);
        }
        if SetForegroundWindow(hwnd) == 0 {
            return Err(anyhow::anyhow!("Failed to focus window: {:?}", title_parts));
        }
    }
    Ok(window)
}
