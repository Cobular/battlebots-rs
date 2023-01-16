use embedded_svc::wifi::{Configuration, AccessPointConfiguration};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::{wifi::EspWifi, eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};

pub fn init_wifi_driver(modem: Modem, sys_loop: EspSystemEventLoop, nvs: EspDefaultNvsPartition) -> EspWifi<'static> {
    let mut wifi_driver = EspWifi::new(modem, sys_loop, Some(nvs)).unwrap();

    wifi_driver
        .set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
            ssid: "YOUR_WIFI_SSID".into(),
            password: "YOUR_WIFI_PASSWORD".into(),
            ..Default::default()
        }))
        .unwrap();

    wifi_driver.start().unwrap();

    wifi_driver
}
