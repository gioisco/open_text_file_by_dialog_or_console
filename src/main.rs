use std::path::PathBuf;
use structopt::StructOpt;
use native_dialog::{FileDialog, MessageDialog, MessageType};

/// Open file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let is_command_line = std::env::args().len() > 1;
    let path: PathBuf;

    let current_dir = std::env::current_dir().unwrap();

    if !is_command_line {
        let selecting_file = FileDialog::new()
        .set_location(&current_dir)
        .add_filter("Simple text", &["txt", "toml", "lock"])
        .add_filter("Image file", &["png", "jpg", "jpeg"])
        .show_open_single_file()
        .unwrap();
        path = match selecting_file {
            Some(file) => file,
            None => return,
        };

    } else {
        path = Cli::from_args().path;
    }

    let content = std::fs::read_to_string(&path);

    match content {
        Ok(content) => {
            for line in content.lines() {
                println!("{}", line);
            }
        }
        Err(error) => {
            let error_message = &format!("File: {:?}\nError reading: {}",
                    &path.into_os_string().into_string().unwrap(), error);
            println!("Oh noes: {}", error_message);
            if !is_command_line {
                MessageDialog::new()
                .set_type(MessageType::Info)
                .set_title("Error reading file")
                .set_text(error_message)
                .show_alert()
                .unwrap();
            }
        }
    }
}
