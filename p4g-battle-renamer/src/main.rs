use dotenvy::dotenv;
use rand::Rng;
use std::{env, fs, path::Path, thread, time::Duration};

const DEFAULT_INTERVAL: u64 = 60;

fn main() {
    // Every minute, get a random song, then replace that file accordingly
    let scheduler = thread::spawn(move || {
        dotenv().expect("\".env\" is required.");

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

        let leading_path: String =
            env::var("LEADING_PATH").expect("Expected .env key \"LEADING_PATH\"");
        let songs_unformatted: String =
            env::var("SONGS").expect("Expected forward-slash delimited .env key \"SONGS\"");
        let songs: Vec<&str> = songs_unformatted.split("/").collect();
        let target = env::var("TARGET").expect("Expected .env key \"TARGET\"");
        let timeout = Duration::from_secs(interval);
        let mut rng = rand::thread_rng();

        loop {
            thread::sleep(timeout);
            let song = songs[rng.gen_range(0..songs.len())];
            let path = Path::new(&leading_path).join(song);

            println!(
                "Replacing target \"{target}\" with path \"{}\".",
                path.display()
            );
            fs::copy(path, &target).unwrap();
        }
    });
    scheduler.join().expect("Scheduler panicked.");
}
