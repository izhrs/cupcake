# Cupcake

A modern and beautiful TUI frontend for yt-dlp, built with Rust and [Ratatui](https://github.com/ratatui-org/ratatui).

## Features

- **Multiple Download Modes**:

    - **Single**: Download individual videos quickly
    - **Batch**: Process multiple URLs at once
    - **Playlist**: Download entire playlists with custom settings

- **Real-time Download Monitoring**:

    - Visual progress bar for active downloads
    - Download speed tracking (MB/s)
    - File size information
    - Estimated time of arrival (ETA)
    - Status indicators (Running, Paused, Queued, Completed, Failed)

- **Task Management**:

    - Add new download tasks easily
    - Pause and resume downloads
    - Queue management system
    - Scroll through large task lists with integrated scrollbar

- **User-Friendly Interface**:

    - Tab-based navigation
    - Vim-like keybindings (hjkl, gg, G, visual/select mode planned)
    - Keyboard shortcuts for all operations
    - Highlighted selection for better visibility
    - Clean, modern UI design with color themes

- **Settings & Customization**:
    - Configure download formats
    - Set default download locations
    - Manage application preferences
    - Theme selection (Catppuccin, Dracula, Nord, Rose Pine, Latte, etc.)
    - Planned: Lua-based plugin system and configuration

## Installation

You can download the latest prebuilt binary from [Releases](https://github.com/izhrs/cupcake/releases).


### Prerequisites

- Rust (latest stable version)
- yt-dlp installed and available in your PATH

### From Source

```bash
# Clone the repository
git clone https://github.com/izhrs/cupcake.git
cd cupcake

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Usage

Navigate the interface using the following keyboard shortcuts:

- `Tab`/`Shift+Tab`: Switch between tabs
- `↑`/`↓` or `k`/`j`: Navigate through tasks
- `Enter`: Select/Activate current item
- `Space`: Pause/Resume selected download
- `a`: Add a new download task
- `d`: Remove selected task (planned)
- `q` or `Esc`: Quit the application
- `h`/`l`: Move focus between menu/content (with Ctrl for quick switch)
- `gg`/`G`: Jump to first/last task (planned)
- Visual/select mode for multi-task operations (planned)

## Configuration

Cupcake uses yt-dlp under the hood and supports many of its configuration options. You can access them through the Settings tab within the application.

- Theme selection: Choose from built-in color schemes (Catppuccin, Dracula, Nord, Rose Pine, Latte, etc.)
- Default download location: Set your preferred directory
- Plugin system: Planned Lua-based plugins for extensibility

## Themes

Cupcake supports multiple themes for a modern look. Themes are defined in code and can be selected via the Settings tab. Each theme customizes background, foreground, border, accent, and status colors.

Example themes:

- Catppuccin Mocha/Latte
- Dracula
- Rose Pine
- Nord

You can extend or customize themes by editing `src/model/theme.rs`.

## Architecture & Code Documentation

Cupcake is built using an Elm-inspired architecture, adapted for Rust and Ratatui. The core principles are:

- **Model**: Central state struct (`Model`) holds all application data, including download managers, UI state, input fields, and theme.
- **Update**: All state mutations are driven by messages (`Message` enum in `src/update/message.rs`). Messages are sent via async channels and processed in the update loop.
- **View**: UI components render based on the current model state. Some Ratatui widgets are stateful and may mutate their own state directly within component functions.

### State Mutation

There are three main ways state is mutated:

1. **Message Passing**: Async channels (`tokio::sync::mpsc::UnboundedSender`) send messages to the update loop, which then calls appropriate model methods.
2. **Shared State Access**: Shared state is managed via `Arc<RwLock<T>>` for panels and tabs, allowing concurrent read access and controlled mutation via async methods.
3. **Direct Internal Methods**: Some structs (like download managers) expose methods that mutate their own data, called directly in the update function or components. Some Ratatui widgets also mutate their state directly in view components.

### Async Event-Based Runtime

Cupcake uses an async runtime (Tokio) to handle events, downloads, and UI updates. Download tasks are spawned as async jobs, and their progress/status is communicated back to the main model via messages.

### Keybindings

Cupcake now uses Vim-like keybindings for navigation and actions, making it familiar for Neovim/Vim users. Visual/select mode for multi-task operations (delete, pause, resume) is planned.

### Extensibility

A Lua-based plugin system is planned, allowing users to extend functionality, customize themes, and configure default download locations and other preferences.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the GNU General Public License v2.0. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Ratatui](https://github.com/ratatui-org/ratatui) for the TUI framework
- [tui-input](https://github.com/sayanarijit/tui-input) for input handling
- [tui-tree-widget](https://github.com/edjopato/tui-rs-tree-widget) for menu
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) for the powerful download engine
- [color-eyre](https://github.com/eyre-rs/color-eyre) for error handling

## Roadmap

- [x] Single video download mode
- [x] Real-time progress bar and status updates
- [x] Download speed, ETA, and file size display
- [ ] Task queue management (Add, Scroll, View status)
- [x] Theme system with Catppuccin, Dracula, Nord, Rose Pine, etc.
- [x] Async download task handling with message passing
- [x] Vim-like navigation and keybindings
- [ ] Batch and playlist download modes
- [ ] Pause/resume and multi-task selection (visual/select mode)
- [ ] Format selection (Audio/Video)
- [ ] Extra arguments for yt-dlp
- [ ] Lua-based plugin system for extensibility
- [ ] Advanced configuration (theme, download location, etc.)
- [ ] Improved error handling and metadata extraction
- [ ] More keyboard shortcuts and Vim-like navigation
- [ ] Optimisation
- [ ] Documentation and code comments
