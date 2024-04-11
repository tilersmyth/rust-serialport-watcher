use std::sync::{Arc, Mutex};

use serialport::{available_ports, SerialPortType};

const TARGET_VENDOR: u16 = 9025;

pub struct ScaleWatcher {
    port: Arc<Mutex<String>>,
}

impl ScaleWatcher {
    pub fn new(port: Arc<Mutex<String>>) -> ScaleWatcher {
        return ScaleWatcher { port };
    }

    fn check(&mut self) {
        match available_ports() {
            Ok(ports) => {
                let mut locked_port_name = self.port.lock().unwrap();
                for p in ports {
                    match p.port_type {
                        SerialPortType::UsbPort(info) => {
                            match info.vid {
                                TARGET_VENDOR => {
                                    *locked_port_name = p.port_name;
                                }
                                _ => {
                                    *locked_port_name = "".to_string();
                                    continue;
                                }
                            };
                        }
                        _ => {
                            *locked_port_name = "".to_string();
                            continue;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
                eprintln!("Error listing serial ports");
            }
        }
    }
}

impl std::future::Future for ScaleWatcher {
    type Output = ();
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.get_mut().check();
        std::thread::sleep(std::time::Duration::from_secs(1));
        cx.waker().wake_by_ref();
        return std::task::Poll::Pending;
    }
}
