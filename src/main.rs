use std::sync::Arc;
use std::{thread, time::Duration};
use anyhow::bail;
use anyhow::Result;

use embedded_svc::ping::Ping;
use embedded_svc::wifi::Wifi;
use embedded_svc::{wifi::{Configuration, ClientConfiguration, AccessPointConfiguration, Status, ClientStatus, ClientConnectionStatus, ClientIpStatus, ApIpStatus, ApStatus}, ipv4};
use esp_idf_svc::{netif::EspNetifStack, sysloop::EspSysLoopStack, nvs::EspDefaultNvs, wifi::EspWifi, ping};
// use esp_idf_sys as _;
// use log::info; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_svc::http::client::EspHttpClient;

const SSID: &str = "Xiaomi_85FE";
const PASS: &str = "aa11aa041212";

// const SSID: &str = env!("RUST_ESP32_STD_DEMO_WIFI_SSID");
// const PASS: &str = env!("RUST_ESP32_STD_DEMO_WIFI_PASS");

mod github;
mod bilibili;
pub mod profile;

fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let netif_stack = Arc::new(EspNetifStack::new()?);
    let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
    let default_nvs = Arc::new(EspDefaultNvs::new()?);



    // let url = String::from("https://api.github.com/users/buhe/followers");

    // info!("About to fetch content from {}", url);

    // let mut client = EspHttpClient::new_default()?;

    // let response = client.get(&url)?.submit()?;

    // let body: Result<Vec<u8>, _> = Bytes::<_, 64>::new(response.reader()).collect();

    // let body = body?;
    // let str = String::from_utf8_lossy(&body).into_owned();
    // println!(
    //     "Body \n{:?}",
    //     &str
    // );

    // let users: Vec<User> = serde_json::from_str(&str).unwrap();
    // println!("Hello, world!bugu22: {:?}", users.len());
    let mut i = 0;
    loop {
        
        let wifi = wifi(
            netif_stack.clone(),
            sys_loop_stack.clone(),
            default_nvs.clone(),
        )?;
        // println!("...start...");
        let mut client = EspHttpClient::new_default()?;
        let mut res = vec![];
        res.push(github::init(&mut client)?);
        res.push(bilibili::init(&mut client)?);
        
        // for r in  res {
            // println!("{:?}", r);
        // }
        drop(client);
        drop(wifi);
        i = i + 1;
        println!("...{}...", i);
        
        // println!("...end...");
        thread::sleep(Duration::from_millis(20000));
    }
}

fn wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    // info!("Wifi created, about to scan");

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);

    let channel = if let Some(ours) = ours {
        // info!(
        //     "Found configured access point {} on channel {}",
        //     SSID, ours.channel
        // );
        Some(ours.channel)
    } else {
        // info!(
        //     "Configured access point {} not found during scanning, will go with unknown channel",
        //     SSID
        // );
        None
    };

    wifi.set_configuration(&Configuration::Mixed(
        ClientConfiguration {
            ssid: SSID.into(),
            password: PASS.into(),
            channel,
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: "aptest".into(),
            channel: channel.unwrap_or(1),
            ..Default::default()
        },
    ))?;

    // info!("Wifi configuration set, about to get status");

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(ip_settings))),
        ApStatus::Started(ApIpStatus::Done),
    ) = status
    {
        // println!("Wifi connected");

        ping(&ip_settings)?;
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}

fn ping(ip_settings: &ipv4::ClientSettings) -> Result<()> {
    // info!("About to do some pings for {:?}", ip_settings);

    let ping_summary =
        ping::EspPing::default().ping(ip_settings.subnet.gateway, &Default::default())?;
    if ping_summary.transmitted != ping_summary.received {
        bail!(
            "Pinging gateway {} resulted in timeouts",
            ip_settings.subnet.gateway
        );
    }

    // info!("Pinging done");

    Ok(())
}