//! Scrape google search results wth a chrome browser.

use thirtyfour::prelude::*;
use thirtyfour::support;

use std::time::Duration;

/// The google entry page
const GOOGLE_URL: &str = "https://google.com";

/// The url of the running webdriver
const WEBDRIVER_SERVER_URL: &str = "http://localhost:9515";

/// The number of seconds to wait after an actions was performed (search, scrolling)
const SECONDS_AFTER_ACTIONS: Duration = Duration::from_secs(5);

/// Return search results in a vector or urls given a keyword to search for and some other arguments:
/// - `chrome_binary`: the full path of the chrome binary
/// - `proxy`: the proxy to use if any
/// - `search_for`: the keyword(s) to search for in Google
/// - `number_of_scrolls`: the number of times to scroll down the page
pub async fn scrape(
    chrome_binary: String,
    proxy: Option<String>,
    search_for: String,
    number_of_scrolls: u8,
) -> WebDriverResult<Vec<String>> {
    // Setup the browser.
    let mut caps = DesiredCapabilities::chrome();
    let _ = caps.set_binary(chrome_binary.as_str());

    match proxy {
        Some(p) => {
            let _ = caps
                .add_chrome_arg(format!("--proxy-server={}", p).as_str())
                .unwrap();
        }
        None => (),
    };

    // Launch the browser session given the webdriver url and capabilities.
    let driver = WebDriver::new(WEBDRIVER_SERVER_URL, caps).await?;

    // Set the window size to fullscreen.
    driver.maximize_window().await?;

    // Navigate to the google entry page.
    driver.goto(GOOGLE_URL).await?;

    // Find the search form element by class name.
    let elem_form = driver.find(By::ClassName("gLFyf")).await?;

    // Type in the search terms.
    elem_form.send_keys(search_for + Key::Enter).await?;

    // Wait for the first batch of results to load
    support::sleep(SECONDS_AFTER_ACTIONS).await;

    // TODO: we can instead look for the header to implicitly wait for the page to load with something like this but with a regex to ignore the language:
    // assert_eq!(driver.title().await?, format!("{} - Search with Google", search_for));

    for _ in 0..number_of_scrolls {
        // Scroll down
        driver
            .execute(
                "window.scrollTo(0, document.body.scrollHeight);",
                Vec::new(),
            )
            .await?;

        // As we scroll down google will also request to press the `More results` button.
        let more_results = driver.find(By::ClassName("RVQdVd")).await?;
        // Click it ignoring the results as the button seems to be always on page but with no size at first.
        let _ = more_results.click().await;

        // Wait for scrolling load a new result batch
        support::sleep(SECONDS_AFTER_ACTIONS).await;
    }

    // The `g` class contain the results we are after.
    let results_found = driver.find_all(By::ClassName("g")).await?;

    // Create an empty vector to store the results.
    let mut return_vec = Vec::new();

    for r in results_found {
        // Find the link tag
        let link_title = r.find(By::Tag("a")).await?;
        // Get the url from it
        let link = link_title.attr("href").await?;

        // Insert link to results
        match link {
            Some(l) => return_vec.push(l),
            None => (),
        };
    }

    // Close the browser.
    driver.quit().await?;

    Ok(return_vec)
}
