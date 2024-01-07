// cmd

use crate::xml::Tag;
use std::env;
use std::process::Command;

impl Tag {
    pub fn export(&self) {
        let path = env::current_dir().unwrap();

        println!("cwd {:?}", path);

        let mut command = Command::new(if cfg!(target_os = "windows") {
            // "C:\\Program Files\\draw.io\\draw.io.exe"
            "cmd"
        } else {
            "draw.io"
        });

        // command.args([
        //     "--export",
        //     "--output",
        //     "./export/org_pie.pdf",
        //     "--format",
        //     "pdf",
        //     // "--quality",
        //     // "90",
        //     // "--border",
        //     // "0",
        //     // "--page-index",
        //     // "0",
        //     "--crop",
        //     "./xml/org_pie.drawio",
        // ]);

        command.args(["/C", "draw.io.exe", "--help"]);

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
