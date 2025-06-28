# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Remote Media Pi (RMP-Server) is a Rust HTTP server that provides a REST API for managing torrents and searching torrent content. It acts as a bridge between torrent search engines (currently 1337x.to), Transmission BitTorrent client, and web clients.

## Common Commands

### Build and Test
```bash
cargo build           # Build the project
cargo test            # Run all tests
cargo run             # Run the application locally
```

### Docker Development
```bash
docker build . -t rmp                  # Build Docker image
docker run --rm -i rmp cargo build     # Build in container
docker run --rm -i rmp cargo test      # Test in container
```

### Release Build
```bash
cargo build --release  # Build with LTO optimizations
```

## Environment Configuration

The application requires these environment variables (see `.env` file):
- `D_ADDR`: UDP discovery service bind address (default: "0.0.0.0:9191")
- `S_ADDR`: HTTP server bind address (default: "0.0.0.0:9090") 
- `TURL`: Transmission RPC URL (default: "http://127.0.0.1:9091/transmission/rpc")
- `RUST_LOG`: Logging level (default: "info")

## Architecture Overview

### Core Components

**Main Application** (`src/main.rs`):
- Actix-Web HTTP server on configurable port
- Concurrent UDP discovery service for network discovery
- Dependency injection for Client and Searcher components

**HTTP API Handlers** (`src/handlers.rs`):
- `GET /torrent/search?terms=<query>` - Search torrents across sources
- `GET /torrent/list` - List active torrents from Transmission
- `POST /torrent/add` - Add torrent by URL/magnet link
- `POST /torrent/resume/{id}` - Resume paused torrent
- `POST /torrent/pause/{id}` - Pause active torrent  
- `DELETE /torrent/remove/{id}` - Remove torrent

**Torrent Module** (`src/torrent/`):
- `client.rs`: Transmission RPC client wrapper with typed interface
- `search.rs`: Multi-threaded search coordinator using barrier synchronization
- `downloader.rs`: HTTP client trait for fetching web content
- `source.rs`: Source adapter definitions and implementations

### Design Patterns

**Adapter Pattern**: Extensible torrent source implementations
- `L337xTo` adapter for 1337x.to using web scraping with CSS selectors
- Easy to add new torrent sources by implementing `SourceAdapter` trait

**Dependency Injection**: Components injected via Actix-Web data
- `Client` for Transmission operations
- `Searcher` configured with available source adapters

**Concurrent Search**: Multi-threaded search with barrier synchronization
- Each source adapter searches in parallel threads
- Results collected, deduplicated, and sorted by seeders

## Key Dependencies

- **actix-web 3.2.0**: Web framework and HTTP server
- **transmission-rpc**: Custom fork for Transmission BitTorrent client integration
- **reqwest 0.10.8**: HTTP client with rustls-tls and blocking support
- **scraper 0.12.0**: HTML parsing and CSS selection for web scraping
- **tokio 0.2.25**: Async runtime with full feature set

## Testing Approach

Tests are embedded inline with `#[cfg(test)]` modules:
- Unit tests for source adapters with mock HTML data
- `FakeDownloader` implementations for testing without network calls
- Focus on URL building and HTML parsing logic

## Development Notes

- Uses Rust 2018 edition with LTO enabled for release builds
- Async/await pattern throughout the web layer
- Error handling with Result types propagated through call stack
- Comprehensive logging with configurable levels
- Timeout configurations (30s) for external HTTP requests