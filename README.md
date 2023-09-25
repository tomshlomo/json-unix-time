# JSON Timestamp Converter

[GitHub Pages](https://tomshlomo.github.io/json-unix-time/)

A tool to convert Unix timestamps in JSON data to human-readable dates and times. Written in rust, using the great library [egui](https://github.com/emilk/egui).


## Usage

1. Paste your JSON data.
2. The tool identifies numerical fields with Unix timestamps.
3. Displays the sorted times in a human-readable format.
4. For each timestamp, the duration from an anchor is shown. Right click a timestamp to set is as the anchor.
5. Timestamps are identified if they are within given years, configurable via the UI.

## License

This project is licensed under the [MIT License](LICENSE).

## Contact

For questions or suggestions, please open an