use esp_idf_hal::{modem::Modem, sys::EspError};
use esp_idf_svc::{
    eventloop::{EspEventLoop, System},
    nvs::{EspNvsPartition, NvsDefault},
    timer::{EspTimerService, Task},
    wifi::{AsyncWifi, AuthMethod, ClientConfiguration, Configuration, EspWifi},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WifiError {
    #[error("Couldn't convert {0} into string")]
    CredentialConversion(&'static str),

    #[error("Couldn't initialize wifi driver ({0})")]
    Driver(EspError),

    #[error("Couldn't set wifi configuration ({0})")]
    Configuration(EspError),

    #[error("Couldn't start wifi ({0})")]
    Start(EspError),

    #[error("Couldn't connect to wifi ({0})")]
    Connect(EspError),

    #[error("Couldn't wait for network interface ({0})")]
    NetworkInterfaceWait(EspError),
}

pub async fn connect_to<'a>(
    ssid: &'static str,
    password: &'static str,
    modem: &'a mut Modem,
    sys_loop: EspEventLoop<System>,
    nvs: EspNvsPartition<NvsDefault>,
    timer_service: EspTimerService<Task>,
) -> Result<AsyncWifi<EspWifi<'a>>, WifiError> {
    let mut wifi = AsyncWifi::wrap(
        EspWifi::new(modem, sys_loop.clone(), Some(nvs)).map_err(WifiError::Driver)?,
        sys_loop,
        timer_service,
    )
    .map_err(WifiError::Driver)?;

    let configuration = Configuration::Client(ClientConfiguration {
        ssid: ssid
            .try_into()
            .map_err(|()| WifiError::CredentialConversion(ssid))?,
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: password
            .try_into()
            .map_err(|()| WifiError::CredentialConversion(ssid))?,
        channel: None,
        ..Default::default()
    });

    wifi.set_configuration(&configuration)
        .map_err(WifiError::Configuration)?;

    wifi.start().await.map_err(WifiError::Start)?;
    log::info!("Wifi started");

    wifi.connect().await.map_err(WifiError::Connect)?;
    log::info!("Wifi connected");

    wifi.wait_netif_up()
        .await
        .map_err(WifiError::NetworkInterfaceWait)?;
    log::info!("Wifi netif up");

    Ok(wifi)
}
