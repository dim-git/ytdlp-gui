use shared_child::SharedChild;
use std::{
    io::{self, BufRead, BufReader},
    path::PathBuf,
    process::Stdio,
    sync::Arc,
};

use iced::futures::channel::mpsc::UnboundedSender;

#[cfg(target_os = "windows")]
use crate::CREATE_NO_WINDOW;

#[derive(Debug, Clone)]
pub enum Message {
    Run(String),
    Stop,
    Finished,
    AlreadyExists,
    PlaylistNotChecked,
    Error(String),
}

#[derive(Default)]
pub struct Command {
    pub shared_child: Option<Arc<SharedChild>>,
}

impl Command {
    // #[allow(clippy::too_many_arguments)]

    // fn view(&self) -> Element<Message> {
    //     ..
    // }

    pub fn kill(&self) -> io::Result<()> {
        if let Some(child) = &self.shared_child {
            return child.kill();
        }
        Ok(())
    }

    pub fn start(
        &mut self,
        mut args: Vec<&str>,
        show_modal: &mut bool,
        modal_body: &mut String,
        bin_dir: Option<PathBuf>,
        sender: Option<UnboundedSender<String>>,
    ) {
        let mut command = std::process::Command::new(bin_dir.unwrap_or_default().join("yt-dlp"));

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(CREATE_NO_WINDOW);
        }

        let print = [
            "--print",
            r#"before_dl:__{"type": "pre_download", "video_id": "%(id)s"}"#,
            "--print",
            r#"playlist:__{"type": "end_of_playlist"}"#,
            "--print",
            r#"after_video:__{"type": "end_of_video"}"#,
        ];

        let progess_template = [
            "--progress-template",
            // format progress as a simple json
            r#"__{"type": "downloading", "video_title": "%(info.title)s", "eta": %(progress.eta)s, "downloaded_bytes": %(progress.downloaded_bytes)s, "total_bytes": %(progress.total_bytes)s, "elapsed": %(progress.elapsed)s, "speed": %(progress.speed)s, "playlist_count": %(info.playlist_count)s, "playlist_index": %(info.playlist_index)s }"#,
            // "--progress-template",
            // r#"postprocess:__{"type": "post_processing", "status": "%(progress.status)s"}"#,
        ];

        args.extend_from_slice(&print);
        args.extend_from_slice(&progess_template);

        args.push("--no-quiet");

        let Ok(shared_child) = SharedChild::spawn(
            command
                .args(args)
                .stderr(Stdio::piped())
                .stdout(Stdio::piped()),
        ) else {
            tracing::error!("Spawning child process failed");
            *show_modal = true;
            *modal_body = String::from("yt-dlp binary is missing");
            return;
        };

        self.shared_child = Some(Arc::new(shared_child));

        let Some(child) = self.shared_child.clone() else {
                        *show_modal = true;
                        *modal_body = String::from("Something went wrong");
                        tracing::error!("No child process");
                        return;
                    };

        *show_modal = true;
        *modal_body = String::from("Initializing...");

        if let Some(stderr) = child.take_stderr() {
            let Some(sender) = sender.clone() else {
                *show_modal = true;
                *modal_body = String::from("Something went wrong");
                return;
            };
            std::thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines().flatten() {
                    sender
                        .unbounded_send(format!("stderr:{line}"))
                        .unwrap_or_else(|e| tracing::error!("failed to send stderr: {e}"));
                }
            });
        }

        if let Some(stdout) = child.take_stdout() {
            let Some(sender) = sender else {
                *show_modal = true;
                *modal_body = String::from("Something went wrong");
                return;
            };
            std::thread::spawn(move || {
                let mut reader = BufReader::new(stdout);
                let mut buffer = vec![];
                loop {
                    let Ok(bytes_read) = reader.read_until(b'\r', &mut buffer) else {
                                        panic!("failed to read buffer");
                                };

                    if bytes_read == 0 {
                        break;
                    }

                    sender
                        .unbounded_send(String::from_utf8_lossy(&buffer).to_string())
                        .unwrap_or_else(|e| tracing::error!("failed to send stdout: {e}"));

                    buffer.clear();
                }
                // sender
                //     .unbounded_send(String::from("Finished"))
                //     .unwrap_or_else(|e| tracing::error!(r#"failed to send "Finished": {e}"#));
            });
        }
    }
}
