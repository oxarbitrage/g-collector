# G-Collector: Automate URL Collection from Google Search Results

G-Collector is a Rust-based tool designed to automate the collection of URLs from Google search results. It utilizes the `thirtyfour` Rust crate to streamline the manual process of web scraping.

## Introduction

The G-Collector project originated as a proof of concept to explore Rust's capabilities in browser automation. It has evolved into a convenient tool for web scraping and is shared here to assist others in getting started with browser automation in Rust.

## Prerequisites

Before you begin, ensure you have the following in place:

- Rust and Cargo installed. [Install Rust](https://www.rust-lang.org/tools/install)

- A compatible operating system, preferably Linux Mint, Ubuntu, or Debian.

- Note that Google frequently updates its search result pages, which may affect the program's functionality. There are no guarantees that it will work as expected.

- Be aware that making too many requests to Google can lead to IP blocking. Use the program responsibly.

- You will need [chromedriver](https://chromedriver.chromium.org/) and the Chrome browser installed.

## Installation

### Install chromedriver

1. Check your Chrome version (Help -> About Google Chrome).
2. Download the appropriate `chromedriver` zip file for your Chrome version.
3. Extract the zip file and run `./chromedriver` in a command terminal. This opens the webdriver server at `http://localhost:9515`.

### Optionally install `chrome` from `chrome-for-testing`

- You can also download the `chrome` binary from [chrome-for-testing](https://googlechromelabs.github.io/chrome-for-testing/). This version of Chrome is specifically designed for automated browser testing.

### Install `g-collector`

Use the following command to install G-Collector:

```sh
cargo install --path .
```

## Usage

G-Collector provides a set of command-line arguments for customizing your search:

```sh
$ g-collector --help
A command line program to scrape google results urls given a keyword to search for

Usage: g-collector [OPTIONS] --chrome <CHROME> --search-for <SEARCH_FOR>

Options:
  -c, --chrome <CHROME>                        Full path of the chrome binary
  -p, --proxy <PROXY>                          Proxy to use if any
  -s, --search-for <SEARCH_FOR>                The keyword(s) to search for in Google
  -n, --number-of-scrolls <NUMBER_OF_SCROLLS>  Number of times to scroll down the page [default: 1]
  -h, --help                                   Print help
  -V, --version                                Print version
$ 
```

### Example usages

To collect results for a keyword:

```sh
g-collector -c /path/to/chrome-for-testing/chrome-linux64/chrome -s "i love rust"
```

To use a SOCKS5 proxy (note: Tor might be blocked by Google):

```sh
g-collector -c /path/to/chrome-for-testing/chrome-linux64/chrome -s "i love rust" -p "socks5://localhost:9050"
```

Adjust the number of scrolls to control the number of results:

```sh
g-collector -c /path/to/chrome-for-testing/chrome-linux64/chrome -s "i love rust" -n 20
```

## Program architecture

G-Collector consists of a library and a binary. The library contains a single function, `scrape``, while the binary calls this function. This separation allows you to use the scrape library programmatically from other Rust code and process results as needed.

## License

This project is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute it according to the terms of this license.