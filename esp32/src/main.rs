use dotenvy_macro::dotenv;
use embassy_executor::Spawner;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, timer::EspTaskTimerService,
};

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

    let _wifi = wifi::connect_to(
        dotenv!("WIFI_SSID"),
        dotenv!("WIFI_PASSWORD"),
        &mut peripherals.modem,
        sys_loop.clone(),
        nvs.clone(),
        timer_service.clone(),
    )
    .await
    .unwrap();

    core::future::pending::<()>().await;
}
