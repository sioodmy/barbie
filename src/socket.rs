#[macro_export]
macro_rules! make_socket {
    ($socket:literal, $sender:ident, $do:expr) => {
        gio::spawn_blocking(move || loop {
            let _ = std::fs::remove_file($socket);

            let listener = UnixListener::bind($socket).unwrap();

            for stream in listener.incoming() {
                match stream {
                    Ok(_) => {
                        $sender.send_blocking($do).expect("couldnt send");
                    }
                    Err(err) => {
                        error!("Problem while parsing socket data: {}", err);
                        break;
                    }
                }
            }
        });
    };
}
