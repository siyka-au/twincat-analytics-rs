use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::mqttbytes::v5::Packet;
use tokio::{task, time};

use rumqttc::v5::{AsyncClient, MqttOptions, Event};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    // color_backtrace::install();

    let mut mqttoptions = MqttOptions::new("test-1", "mqtt01.klipspringer.inductabend.net", 1883);
    mqttoptions
        .set_keep_alive(Duration::from_secs(5))
        .set_credentials("test", "SAeHZdsuZd7XJTTPAbR2vXD3p7FzjzCY");

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    task::spawn(async move {
        requests(client).await;
        time::sleep(Duration::from_secs(3)).await;
    });

    loop {
        let event = eventloop.poll().await;
        match &event {
            Ok(v) => {
                println!("Event = {v:?}");
                match &v {
                    Event::Incoming(p) => {
                        match p {
                            Packet::Publish(p)=>{
                                let mut file = File::create("symbols.bin")?;
                                file.write_all(&p.payload)?;
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            }
            Err(e) => {
                println!("Error = {e:?}");
                return Ok(());
            }
        }
    }
}

async fn requests(client: AsyncClient) {
    client
        .subscribe("ema/plc-stream01/Bin/Tx/Symbols", QoS::AtMostOnce)
        .await
        .unwrap();

    time::sleep(Duration::from_secs(120)).await;
}
