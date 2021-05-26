use calc::prelude::*;
use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::spawn;

// to connect:
//
// ```shell
// telnet 127.0.0.1 8080
// ```

fn handle_client(mut stream: TcpStream, sum_hdl: Arc<Mutex<i64>>) -> Result<(), io::Error> {
    let mut buf = [0u8; 128];
    println!("Handling client...");

    loop {
        match stream.read(&mut buf) {
            Ok(bytes) if bytes == 0 => {
                continue;
            }
            Ok(bytes) => {
                let string_data = String::from_utf8_lossy(&buf[..bytes]);
                let parsed = match parse(&string_data) {
                    Ok(p) => p,
                    Err(e) => {
                        let formatted = format!("ERROR: {:?}\n", e);
                        stream.write_all(formatted.as_bytes())?;
                        return Ok(());
                    }
                };

                let evaled: i64 = match eval(&parsed) {
                    Ok(ev) => ev,
                    Err(e) => {
                        let formatted = format!("ERROR: {:?}\n", e);
                        stream.write_all(formatted.as_bytes())?;
                        return Ok(());
                    }
                };

                let current_sum: i64 = sum_hdl
                    .lock()
                    .map(|mut g| {
                        *g += evaled;
                        *g
                    })
                    .unwrap();

                println!("Sum: {}", current_sum);

                let formatted = format!("= {}\n", evaled);
                stream.write_all(formatted.as_bytes())?;
                return Ok(());
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        };
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let total_sum = Arc::new(Mutex::new(0i64));

    // accept connections and process them serially
    for stream in listener.incoming() {
        // When you clone an Arc, or Atomic Reference Counted wrapper,
        // ONLY a new handle is cloned. We create a NEW HANDLE to the
        // SAME DATA
        let new_sum_handle = total_sum.clone();
        // Spawn a thread
        let _ = spawn(move || handle_client(stream?, new_sum_handle));
    }

    Ok(())
}
