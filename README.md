# Rust-based Wikipedia Web Scraper

Rust-based web scraper that fetches, parses, and cleans Wikipedia articles. Built for integration with the Wiki-brief Rust backend.

---

## Features

- Scrapes Wikipedia articles by title  
- Cleans and extracts human-readable content  
- Exposes a REST API via Actix Web

---

## Disclaimer
This tool is specifically designed for use with Wikipedia, which generally allows web scraping and content reuse under its Creative Commons license.

Do not use this tool to scrape other websites without verifying their terms of service. Many websites prohibit automated scraping and content reuse, and doing so may result in legal consequences.

---

## Build and Run

Ensure [Rust is installed](https://www.rust-lang.org/tools/install).

### 1. Build the Project

```bash
cargo build
```

### 2. Run the Server

```bash
cargo run
```

The server will start at [http://localhost:8080](http://localhost:8080).

---

## API Usage

### Endpoint: `GET /web-scraper/{title}`

Returns content of the specified Wikipedia article.

#### Example

```bash
curl http://localhost:8080/web-scraper/turtle
```
