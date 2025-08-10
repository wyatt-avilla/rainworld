use dotenvy_macro::dotenv;
use embassy_executor::Spawner;
use esp_idf_hal::{adc::oneshot::AdcDriver, peripherals::Peripherals};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, timer::EspTaskTimerService,
};

mod config;
mod plant_with_hardware;
mod sensors;
mod server;
mod time;
mod wifi;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let _ = spawner;
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();
    let timer_service = EspTaskTimerService::new().unwrap();
    let static_adc = Box::leak(Box::new(AdcDriver::new(peripherals.adc1).unwrap()));

    let wifi = wifi::connect_to(
        dotenv!("WIFI_SSID"),
        dotenv!("WIFI_PASSWORD"),
        &mut peripherals.modem,
        sys_loop.clone(),
        nvs.clone(),
        timer_service.clone(),
    )
    .await
    .unwrap();

    log::info!(
        "Device ip: {}",
        wifi.wifi().ap_netif().get_ip_info().unwrap().ip
    );

    let _sntp = time::sync().await.unwrap();

    let plants_with_hardware = std::sync::Arc::new(
        config::plant_hardware_associations(static_adc, peripherals.pins).unwrap(),
    );

    let _server = server::new_server(&plants_with_hardware);

    core::future::pending::<()>().await;
}
