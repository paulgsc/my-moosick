use chrono::{DateTime, Duration, Utc};
use dirs::home_dir;
use rusqlite::{Connection, Result};
use std::path::PathBuf;

fn main() -> Result<()> {
	let chrome_data_dir = get_chrome_data_dir();
	let current_session_file = chrome_data_dir.join("Current Session");
	let current_tabs_file = chrome_data_dir.join("Current Tabs");

	// Parse Current Session and Current Tabs files
	// This part requires understanding Chrome's binary format for these files
	let tabs = parse_chrome_session_files(current_session_file, current_tabs_file);

	// Connect to the Chrome history database
	let history_db = chrome_data_dir.join("History");
	let conn = Connection::open(history_db)?;

	for tab in tabs {
		let last_visited = get_last_visit_time(&conn, &tab.url)?;
		println!("URL: {}, Title: {}, Last Visited: {}", tab.url, tab.title, last_visited);
	}

	Ok(())
}

fn get_chrome_data_dir() -> PathBuf {
	// This will vary based on the operating system
	home_dir().unwrap().join(".config").join("google-chrome")
}

fn parse_chrome_session_files(session_file: PathBuf, tabs_file: PathBuf) -> Vec<Tab> {
	// Implement parsing logic here
	// This is a complex task that requires understanding Chrome's binary format
	vec![] // Placeholder
}

fn get_last_visit_time(conn: &Connection, url: &str) -> Result<DateTime<Utc>> {
	let mut stmt = conn.prepare("SELECT last_visit_time FROM urls WHERE url = ? LIMIT 1")?;
	let mut rows = stmt.query(&[url])?;

	if let Some(row) = rows.next()? {
		let timestamp: i64 = row.get(0)?;
		// Chrome stores time as microseconds since 1601-01-01 UTC
		// We need to convert this to a more standard format
		// This conversion is simplified and may need adjustment
		let seconds_since_epoch = (timestamp / 1_000_000) - 11_644_473_600;
		Ok(DateTime::from_timestamp(seconds_since_epoch, 0).unwrap())
	} else {
		Ok(Utc::now()) // Fallback if not found
	}
}

fn get_recent_tabs_with_active_time(conn: &Connection) -> Result<Vec<Tab>> {
	let mut stmt = conn.prepare(
		"
        SELECT u.url, u.title, u.last_visit_time,
               SUM(CASE WHEN v.visit_duration > 0 THEN v.visit_duration ELSE 0 END) as total_time
        FROM urls u
        LEFT JOIN visits v ON u.id = v.url
        GROUP BY u.id
        ORDER BY u.last_visit_time DESC
        LIMIT 50",
	)?;

	let tab_iter = stmt.query_map([], |row| {
		Ok(Tab {
			url: row.get(0)?,
			title: row.get(1)?,
			last_visited: chrome_time_to_datetime(row.get(2)?),
			active_time: Duration::microseconds(row.get(3)?),
		})
	})?;

	tab_iter.collect()
}

fn chrome_time_to_datetime(chrome_time: i64) -> DateTime<Utc> {
	let seconds_since_epoch = (chrome_time / 1_000_000) - 11_644_473_600;
	DateTime::from_timestamp(seconds_since_epoch, 0).unwrap_or(Utc::now())
}

struct Tab {
	url: String,
	title: String,
	last_visited: DateTime<Utc>,
	active_time: Duration,
}
