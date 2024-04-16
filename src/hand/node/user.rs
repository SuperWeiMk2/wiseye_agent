use std::fs::File;
use std::process::Command;
use std::io;
use std::io::BufRead;

struct UserAdd {
    username: String,
    args: Vec<String>,
}

impl UserAdd {
    fn new(username: &str, args: &[&str]) -> Self {
        Self {
            username: username.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn useradd(&self) -> io::Result<()> {
        let mut command = Command::new("useradd");
        command.arg(&self.username);
        for arg in &self.args {
            command.arg(arg);
        }

        let status = command.status()?;

        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Failed to create user"))
        }
    }
}


// TODO: 不稳定
pub fn get_username_by_uid(uid: u32) -> Option<String> {
    if let Ok(passwd_file) = File::open("/etc/passwd") {
        let reader = io::BufReader::new(passwd_file);

        for line in reader.lines() {
            if let Ok(entry) = line {
                let fields: Vec<&str> = entry.split(':').collect();

                if fields.len() >= 3 {
                    if let Ok(parsed_uid) = fields[2].parse::<u32>() {
                        if parsed_uid == uid {
                            return Some(fields[0].to_string()); // Return the username
                        }
                    }
                }
            }
        }
    }

    None
}