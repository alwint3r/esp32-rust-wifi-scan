use std::sync::Arc;

use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use embedded_svc::wifi::Wifi;
use esp_idf_hal::delay;
use esp_idf_svc::{
    netif::EspNetifStack, nvs::EspDefaultNvs, sysloop::EspSysLoopStack, wifi::EspWifi,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let netif_stack = Arc::new(EspNetifStack::new().unwrap());
    let sys_loop_stack = Arc::new(EspSysLoopStack::new().unwrap());
    let default_nvs = Arc::new(EspDefaultNvs::new().unwrap());

    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)).unwrap();
    println!("WiFi created, about to scan!");

    let mut delay = delay::FreeRtos;

    loop {
        match wifi.scan() {
            Ok(ap_info) => {
                for ap in ap_info {
                    println!("SSID: {}, RSSI: {}", ap.ssid, ap.signal_strength as i8);
                }
            }
            Err(err) => {
                println!("Failed scanning WiFi access points: {}", err);
            }
        }

        delay.delay_ms(5000 as u32);
    }
}
