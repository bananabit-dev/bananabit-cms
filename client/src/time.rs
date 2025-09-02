//! Cross-platform time utilities that work on both native and WASM

#[cfg(not(target_arch = "wasm32"))]
pub use chrono::{DateTime, Utc};

#[cfg(target_arch = "wasm32")]
use js_sys::Date;

/// Get current timestamp as ISO 8601 string
pub fn now_iso8601() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        let date = Date::new_0();
        date.to_iso_string().as_string().unwrap()
    }
}

/// Get current date as YYYY-MM-DD string
pub fn today_date() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        chrono::Utc::now().format("%Y-%m-%d").to_string()
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        let date = Date::new_0();
        let year = date.get_full_year();
        let month = date.get_month() + 1; // JS months are 0-based
        let day = date.get_date();
        format!("{:04}-{:02}-{:02}", year, month, day)
    }
}

/// Generate a simple UUID-like string (for WASM compatibility)
pub fn generate_id() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        uuid::Uuid::new_v4().to_string()
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Math;
        // Generate a simple random ID that looks like a UUID
        let timestamp = Date::now() as u64;
        let random1 = (Math::random() * 1000000.0) as u32;
        let random2 = (Math::random() * 1000000.0) as u32;
        format!("{:08x}-{:04x}-4{:03x}-{:04x}-{:08x}{:04x}", 
                timestamp & 0xffffffff,
                (timestamp >> 32) & 0xffff,
                random1 & 0xfff,
                0x8000 | (random2 & 0x3fff),
                random1,
                random2 & 0xffff)
    }
}