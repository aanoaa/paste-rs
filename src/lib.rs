//! `paste-rs` pastebin service to learn Rust
pub mod config;
pub mod paste;
pub mod server;

/// environment variable for configuration file path
pub const CONFIG_ENV: &str = "CONFIG";

/// template landing page content
pub const LANDING_PAGE: &str = r#"
  API USAGE

      POST :HOST:

          Send the raw data along. Will respond with a link to the paste.

          If the response code is 201 (CREATED), then the entire paste was
          uploaded. If the response is 206 (PARTIAL), then the paste exceeded
          the server's maximum upload size, and only part of the paste was
          uploaded. If the response code is anything else, an error has
          occurred. Pasting is heavily rate limited.

      GET :HOST:/<id>

          Retrieve the paste with the given id as plain-text.

      GET :HOST:/<id>.<ext>

          Retrieve the paste with the given id. If ext is a known code file
          extension, the paste is syntax highlighted and returned as HTML. If
          ext is a known file extension, the paste is returned with the
          extension's corresponding Content-Type. Otherwise, the paste is
          returned as plain text.

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
