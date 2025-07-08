//use rand::Rng;
//use std::{env, fs, thread, time::Duration};

//const DEFAULT_INTERVAL: u64 = 5;

// No additional files involved, just the main wg0.conf file. Comments at very beginning of file will be used.
// Interface.PrivateKey --> Peer.PublicKey
// Interface.Address & Peer[].AllowedIPs --> count upwards and fill in holes of IP addresses --> Interface.Address & Interface.DNS &
// Interface.ListenPort --- Peer.Endpoint
// yawgi:dns-default=Interface.Address (unless specified)
// The peer's Peer.AllowedIPs will be a choice on runtime, "Route traffic through VPN?"

fn main() {
    // Every minute, get a random song, then replace that file accordingly
    /*let scheduler = thread::spawn(move || {
        let interval: u64 = {
            match env::var("INTERVAL") {
                Ok(value) => match value.parse::<u64>() {
                    Ok(value) => value,
                    Err(_) => DEFAULT_INTERVAL,
                },
                Err(_) => DEFAULT_INTERVAL,
            }
        };
        println!("Running every {interval} seconds.");

        let timeout = Duration::from_secs(interval);
        let mut rng = rand::thread_rng();

        loop {
            thread::sleep(timeout);
            fs::write("/etc/test.txt", format!("{}", rng.gen_range(0..5000))).unwrap();
        }
    });
    scheduler.join().expect("Scheduler panicked.");*/
}
