# Cupcake

A modern TUI (Terminal User Interface) frontend for yt-dlp, built with Rust and [Ratatui](https://github.com/ratatui-org/ratatui).

## Features

-   **Multiple Download Modes**:

    -   **Single**: Download individual videos quickly
    -   **Batch**: Process multiple URLs at once
    -   **Playlist**: Download entire playlists with custom settings

-   **Real-time Download Monitoring**:

    -   Visual progress bar for active downloads
    -   Download speed tracking (MB/s)
    -   File size information
    -   Estimated time of arrival (ETA)
    -   Status indicators (Running, Paused, Queued, Completed, Failed)

-   **Task Management**:

    -   Add new download tasks easily
    -   Pause and resume downloads
    -   Queue management system
    -   Scroll through large task lists with integrated scrollbar

-   **User-Friendly Interface**:

    -   Tab-based navigation
    -   Keyboard shortcuts for all operations
    -   Highlighted selection for better visibility
    -   Clean, modern UI design with color themes

-   **Settings & Customization**:
    -   Configure download formats
    -   Set default download locations
    -   Manage application preferences

## Installation

### Prerequisites

-   Rust (latest stable version)
-   yt-dlp installed and available in your PATH

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/cupcake.git
cd cupcake

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Usage

Navigate the interface using the following keyboard shortcuts:

-   `Tab`/`Shift+Tab`: Switch between tabs
-   `↑`/`↓`: Navigate through tasks
-   `Enter`: Select/Activate current item
-   `Space`: Pause/Resume selected download
-   `a`: Add a new download task
-   `d`: Remove selected task
-   `q` or `Esc`: Quit the application

## Configuration

Cupcake uses yt-dlp under the hood and supports many of its configuration options. You can access them through the Settings tab within the application.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

-   [Ratatui](https://github.com/ratatui-org/ratatui) for the TUI framework
-   [yt-dlp](https://github.com/yt-dlp/yt-dlp) for the powerful download engine
-   [color-eyre](https://github.com/eyre-rs/color-eyre) for error handling
