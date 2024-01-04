// cmd

use crate::xml::Tag;
use std::process::Command;

impl Tag {
    pub fn export(&self) {
        let mut command = Command::new(if cfg!(target_os = "windows") {
            "C:\\Program Files\\draw.io\\draw.io.exe"
        } else {
            "draw.io"
        });
        // command.args([
        //     "-x",
        //     "-f",
        //     "pdf",
        //     "-o",
        //     "./target/out.pdf",
        //     "--crop",
        //     "./xml/stacked_bars.drawio",
        // ]);
        // "C:\\Program Files\\draw.io\\draw.io.exe", is_headless: false }, arguments : ["--export", "--output", ".\\xml\\export\\org_line-Page-1.pdf", "--format", "pdf", "--quality", "90", "--border", "0", "--page-index", "0", ".\\xml\\org_line.drawio"]

        // command.args(["-x stacked.drawio"]);
        // command.args(["-x", "stacked.drawio"]);
        command.args([
            "--export",
            "--output",
            "C:\\Users\\Per Lindgren\\Documents\\rust\\drawio-rs\\xml\\export\\org_line-Page-3.pdf",
            "--format",
            "pdf",
            "--quality",
            "90",
            "--border",
            "0",
            "--page-index",
            "0",
            "--crop",
            "C:\\Users\\Per Lindgren\\Documents\\rust\\drawio-rs\\xml\\org_pie.drawio",
        ]);

        println!("command : {:?}", command);
        let output = command.output().expect("failed to execute process");
        println!("output {:?}", output);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_export() {
        Tag::new("hello_world").export();
    }
}
