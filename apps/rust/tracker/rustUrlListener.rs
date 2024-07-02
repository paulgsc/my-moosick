use rusqlite::{Connection, Result};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use url::Url;

fn get_chrome_history_path() -> PathBuf {
	let home = dirs::home_dir().expect("Could not find home directory");
	home.join("Library/Application Support/Google/Chrome/Default/History")
}

fn is_youtube_video_url(url: &str) -> bool {
	if let Ok(parsed_url) = Url::parse(url) {
		parsed_url.host_str() == Some("www.youtube.com") && parsed_url.path() == "/watch"
	} else {
		false
	}
}

fn main() -> Result<()> {
	let history_path = get_chrome_history_path();
	let mut last_url = String::new();

	println!("Monitoring Chrome history for YouTube URLs...");

	loop {
		// Connect to the Chrome history database
		let conn = Connection::open(&history_path)?;

		// Query for the most recent URL
		let mut stmt = conn.prepare("SELECT url FROM urls ORDER BY last_visit_time DESC LIMIT 1")?;

		let url: String = stmt.query_row([], |row| row.get(0))?;

		if url != last_url && is_youtube_video_url(&url) {
			println!("New YouTube video detected: {}", url);

			// Append the URL to a file
			let mut file = OpenOptions::new().create(true).append(true).open("youtube_urls.txt")?;
			writeln!(file, "{}", url)?;

			last_url = url;
		}

		// Sleep for 5 seconds before checking again
		sleep(Duration::from_secs(5));
	}
}
