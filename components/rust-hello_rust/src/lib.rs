#![no_std]

#[cfg(all(feature = "alloc-mode", feature = "no-alloc-mode"))]
compile_error!("Enable exactly one of `alloc-mode` or `no-alloc-mode`.");

#[cfg(not(any(feature = "alloc-mode", feature = "no-alloc-mode")))]
compile_error!("Enable one of `alloc-mode` or `no-alloc-mode`.");

#[cfg(feature = "alloc-mode")]
use esp_idf_svc::log::EspLogger;

#[unsafe(no_mangle)]
extern "C" fn rust_main() -> i32 {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    #[cfg(feature = "alloc-mode")]
    {
        EspLogger::initialize_default();
        log::info!("Hello, world!");
    }

    #[cfg(feature = "no-alloc-mode")]
    unsafe {
        esp_idf_sys::puts(b"Hello, world from Rust (no_alloc)!\0".as_ptr().cast());
    }

    42
}
