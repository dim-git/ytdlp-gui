# Changelog

## Unversioned

- Major: Better download progress messages (including progress for playlist videos) since there is better yt-dlp parsing
- Major: Move downloads logs from `config_dir` to `cache_dir` linux: $XDG_CACHE_HOME or $HOME/.cache -- windows: {FOLDERID_LocalAppData} -- macos: $HOME/Library/Caches
- Major: Dev: Better yt-dlp parsing
- Bugfix: Use `.to_string_lossy()` for download dir instead of `.to_str()`, that will solve [this issue](https://github.com/BKSalman/ytdlp-gui/issues/12)
- Minor: Use default configs if config file is broken
- Minor: Added `--version or -V` and `--help or -h` options to the binary to check the version
- Minor: Better error messages in general (there were almost none actually)
- Dev: Replace log4rs with tracing

## 0.3.0

- Major: Added general logs to stderr and a temporary file in temp directory
- Major: Added Download logs after finishing every download
- Major: Replaced radio buttons with a drop-down menu for selecting resolutions and formats
- Minor: Moved the download button to the bottom
- Minor: Moved the "Browse" button to the right of the path text box
- Minor: Options and settings now save on download instead of saving on app close
- Dev: Replaced env_logger with log4rs to use it for std logging and file logging

## 0.2.5

- Bugfix: Update the packaged yt-dlp version for windows, that will solve [this issue](https://github.com/BKSalman/ytdlp-gui/issues/13)
- Minor: Show message in modal when yt-dlp binary is missing

## 0.2.4

- Bugfix: Fixed crash when download folder is not set

## 0.2.3

- Minor: save current configs on application exit, instead of on every change

## 0.2.2

- Major: Added Config file to save previously chosen options, download path, and bin directory
- Dev: Small refactors

## 0.2.1

- Major: Update ``Iced`` to v0.7.0
- Bugfix: Merge format [#9](https://github.com/BKSalman/ytdlp-gui/issues/9)
