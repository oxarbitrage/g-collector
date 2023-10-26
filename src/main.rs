//! A command line program to scrape google results urls given a keyword to search for.

use clap::Parser;

/// A command line program to scrape google results urls given a keyword to search for.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Full path of the chrome binary
    #[arg(short, long)]
    chrome: String,

    /// Proxy to use if any
    #[arg(short, long, default_value = None)]
    proxy: Option<String>,

    /// The keyword(s) to search for in Google
    #[arg(short, long)]
    search_for: String,

    /// Number of times to scroll down the page
    #[arg(short, long, default_value_t = 1)]
    number_of_scrolls: u8,
}

#[tokio::main]
async fn main() {
    // parse the command line arguments
    let args = Args::parse();

    // call the scraper from the library
    let results = g_collector::scrape(
        args.chrome,
        args.proxy,
        args.search_for,
        args.number_of_scrolls,
    )
    .await
    .unwrap();

    // Print the results
    // TODO: remove duplicated urls if any.
    for r in results {
        println!("{}", r);
    }
}
