use esp_idf_hal::sys::EspError;
use esp_idf_svc::sntp::{EspSntp, SntpConf};

pub async fn sync() -> Result<EspSntp<'static>, EspError> {
    let sntp = EspSntp::new(&SntpConf {
        servers: ["pool.ntp.org"],
        operating_mode: esp_idf_svc::sntp::OperatingMode::Poll,
        sync_mode: esp_idf_svc::sntp::SyncMode::Smooth,
    })?;

    while sntp.get_sync_status() != esp_idf_svc::sntp::SyncStatus::Completed {
        embassy_time::Timer::after(embassy_time::Duration::from_secs(1)).await;
    }

    Ok(sntp)
}
