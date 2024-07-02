use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	// Launch a new instance of Chrome
	let browser = Browser::new(
		LaunchOptionsBuilder::default()
			.headless(false) // We want to interact with the running instance, not a headless one
			.build()
			.unwrap(),
	)?;

	// Get all tabs
	let tabs = browser.get_tabs()?;

	for tab in tabs {
		// Get tab title and URL
		let title = tab.get_title().unwrap_or_else(|| "No title".to_string());
		let url = tab.get_url();

		// Print tab details
		println!("Title: {}, URL: {}", title, url);

		// Determine if the tab is suspended
		// This part depends on how your tab suspender indicates suspension.
		// Here, we assume a suspended tab has "Suspended" in its title or URL.
		let is_suspended = title.contains("Suspended") || url.contains("Suspended");

		println!("Is Suspended: {}", is_suspended);
	}

	Ok(())
}
