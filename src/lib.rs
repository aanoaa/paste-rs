//! `paste-rs` pastebin service to learn Rust
pub mod config;
pub mod paste;
pub mod server;
pub mod util;

/// environment variable for configuration file path
pub const CONFIG_ENV: &str = "CONFIG";
pub const DEFAULT_MIME_TYPE: &str = "text/plain";
pub const DEFAULT_EXTENSION: &str = "txt";

/// template landing page content
pub const LANDING_PAGE: &str = r#"
  USAGE

      POST :HOST:

          Send the raw data along. Will respond with a link to the paste.

      GET :HOST:/<id>

          Retrieve the paste with the given id as plain-text.

      DELETE :HOST:/<id>

          Delete the paste with the given id.

  EXAMPLES

      Paste a file named 'file.txt' using cURL:

          curl --data-binary @file.txt :HOST:

      Paste from stdin using cURL:

          echo "Hello, world." | curl --data-binary @- :HOST:

      Delete an existing paste with <id> using cURL:

          curl -X DELETE :HOST:/<id>

      A shell function that can be added to `.bashrc` or `.bash_profle` for
      quick pasting from the command line. The command takes a filename or reads
      from stdin if none was supplied and outputs the URL of the paste to
      stdout: `paste file.txt` or `echo "hi" | paste`.

          function paste() {
              local file=${1:-/dev/stdin}
              curl --data-binary @${file} :HOST:
          }
"#;
