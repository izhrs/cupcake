mod model;
mod update;
mod view;

use std::path::PathBuf;

use color_eyre::Result;
use ratatui::DefaultTerminal;
use url::Url;

use crate::{
    model::{
        state::AppState,
        task::{Task, TaskStatus},
    },
    update::{message::Msg, update},
    view::draw,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    state: AppState,
}

impl App {
    fn new() -> Self {
        let dummy_taks = vec![
            Task {
                name: "ubuntu-22.04-desktop-amd64.iso".to_string(),
                source: Url::parse(
                    "https://releases.ubuntu.com/22.04/ubuntu-22.04-desktop-amd64.iso",
                )
                .unwrap(),
                destination: PathBuf::from("/home/user/Downloads/ubuntu-22.04-desktop-amd64.iso"),
                speed: 1.0,
                size: 3700.0,
                progress: 0.5,
                eta: "1m".to_string(),
                status: TaskStatus::Running,
            },
            Task {
                name: "big_buck_bunny_1080p.mp4".to_string(),
                source: Url::parse("https://www.example.com/big_buck_bunny_1080p.mp4").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/big_buck_bunny_1080p.mp4"),
                speed: 2.0,
                size: 825.0,
                progress: 0.75,
                eta: "2m".to_string(),
                status: TaskStatus::Paused,
            },
            Task {
                name: "node_modules.tar.gz".to_string(),
                source: Url::parse("https://www.example.com/node_modules.tar.gz").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/node_modules.tar.gz"),
                speed: 0.5,
                size: 154.0,
                progress: 0.25,
                eta: "3m".to_string(),
                status: TaskStatus::Queued,
            },
            Task {
                name: "linux-kernel-6.2.0.tar".to_string(),
                source: Url::parse("https://www.example.com/linux-kernel-6.2.0.tar").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/linux-kernel-6.2.0.tar"),
                speed: 3.0,
                size: 1200.0,
                progress: 1.0,
                eta: "0m".to_string(),
                status: TaskStatus::Finished,
            },
            Task {
                name: "game_assets.zip".to_string(),
                source: Url::parse("https://www.example.com/game_assets.zip").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/game_assets.zip"),
                speed: 0.0,
                size: 4500.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "fedora-38-x86_64.iso".to_string(),
                source: Url::parse("https://releases.fedoraproject.org/38/fedora-38-x86_64.iso")
                    .unwrap(),
                destination: PathBuf::from("/home/user/Downloads/fedora-38-x86_64.iso"),
                speed: 0.0,
                size: 2800.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "movie_collection.tar".to_string(),
                source: Url::parse("https://www.example.com/movie_collection.tar").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/movie_collection.tar"),
                speed: 0.0,
                size: 15000.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "project_backup_2023-10-15.zip".to_string(),
                source: Url::parse("https://www.example.com/project_backup_2023-10-15.zip")
                    .unwrap(),
                destination: PathBuf::from("/home/user/Downloads/project_backup_2023-10-15.zip"),
                speed: 0.0,
                size: 750.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "archlinux-2023.05.01-x86_64.iso".to_string(),
                source: Url::parse(
                    "https://archlinux.org/iso/2023.05.01/archlinux-2023.05.01-x86_64.iso",
                )
                .unwrap(),
                destination: PathBuf::from("/home/user/Downloads/archlinux-2023.05.01-x86_64.iso"),
                speed: 0.0,
                size: 850.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "4k_nature_documentary.mkv".to_string(),
                source: Url::parse("https://www.example.com/4k_nature_documentary.mkv").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/4k_nature_documentary.mkv"),
                speed: 0.0,
                size: 12000.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "debian-11-amd64.iso".to_string(),
                source: Url::parse("https://www.example.com/debian-11-amd64.iso").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/debian-11-amd64.iso"),
                speed: 0.0,
                size: 3200.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "sample_video_1080p_60fps.mp4".to_string(),
                source: Url::parse("https://www.example.com/sample_video_1080p_60fps.mp4").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/sample_video_1080p_60fps.mp4"),
                speed: 0.0,
                size: 1500.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "website_templates.zip".to_string(),
                source: Url::parse("https://www.example.com/website_templates.zip").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/website_templates.zip"),
                speed: 0.0,
                size: 225.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "react_native_project.tar.gz".to_string(),
                source: Url::parse("https://www.example.com/react_native_project.tar.gz").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/react_native_project.tar.gz"),
                speed: 0.0,
                size: 180.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "centos-stream-9-x86_64.iso".to_string(),
                source: Url::parse("https://www.example.com/centos-stream-9-x86_64.iso").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/centos-stream-9-x86_64.iso"),
                speed: 0.0,
                size: 2500.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "conference_recordings.tar".to_string(),
                source: Url::parse("https://www.example.com/conference_recordings.tar").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/conference_recordings.tar"),
                speed: 0.0,
                size: 8500.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "anime_series_s01_complete.mkv".to_string(),
                source: Url::parse("https://www.example.com/anime_series_s01_complete.mkv")
                    .unwrap(),
                destination: PathBuf::from("/home/user/Downloads/anime_series_s01_complete.mkv"),
                speed: 0.0,
                size: 5800.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "docker_images.tar".to_string(),
                source: Url::parse("https://www.example.com/docker_images.tar").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/docker_images.tar"),
                speed: 0.0,
                size: 3400.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "kali-linux-2023.2-live.iso".to_string(),
                source: Url::parse("https://www.example.com/kali-linux-2023.2-live.iso").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/kali-linux-2023.2-live.iso"),
                speed: 0.0,
                size: 4100.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "machine_learning_datasets.zip".to_string(),
                source: Url::parse("https://www.example.com/machine_learning_datasets.zip")
                    .unwrap(),
                destination: PathBuf::from("/home/user/Downloads/machine_learning_datasets.zip"),
                speed: 0.0,
                size: 2700.0,
                progress: 0.0,
                eta: "N/A".to_string(),
                status: TaskStatus::Failed,
            },
            Task {
                name: "Emily_Wills_4K.mp4".to_string(),

                source: Url::parse("https://www.example.com/Emily_Wills_4K.mp4").unwrap(),
                destination: PathBuf::from("/home/user/Downloads/Emily_Wills_4K.mp4"),
                speed: 5.2,
                size: 2439.0,
                progress: 1.00,
                eta: "0m".to_string(),
                status: TaskStatus::Finished,
            },
        ];

        Self {
            state: AppState::new(dummy_taks),
        }
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state.running {
            terminal.draw(|frame| draw(&mut self.state, frame))?;

            let event = crossterm::event::read()?;

            if let Some(msg) = Msg::from_event(event, self.state.focused_block.clone()) {
                update(&mut self.state, msg);
            }
        }
        Ok(())
    }
}
