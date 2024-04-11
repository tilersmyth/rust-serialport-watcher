use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serialport;

const BAUD_RATE: u32 = 9600;

pub struct ScaleReader {
    port: Arc<Mutex<String>>,
    value: String,
}

impl ScaleReader {
    pub fn new(port: Arc<Mutex<String>>) -> ScaleReader {
        return ScaleReader {
            port,
            value: "".to_string(),
        };
    }

    fn read(&mut self) {
        let locked_port_name = self.port.lock().unwrap();

        if locked_port_name.is_empty() {
            println!("Device not detected!");
            return;
        }

        let port = serialport::new(locked_port_name.to_string(), BAUD_RATE)
            .timeout(Duration::from_millis(10))
            .open();

        match port {
            Ok(mut port) => {
                let mut serial_buf: Vec<u8> = vec![0; 1000];
                println!(
                    "Receiving data on {} at {} baud:",
                    &locked_port_name, &BAUD_RATE
                );

                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Err(e) => {
                eprintln!("Failed to open \"{}\". Error: {}", locked_port_name, e);
            }
        }
    }
}

impl std::future::Future for ScaleReader {
    type Output = ();
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.get_mut().read();
        std::thread::sleep(std::time::Duration::from_secs(1));
        cx.waker().wake_by_ref();
        return std::task::Poll::Pending;
    }
}
