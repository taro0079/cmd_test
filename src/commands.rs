use std::process::Command;

pub fn execute_shell() {
    Command::new("sh")
        .arg("./src/shells/install.sh")
        .spawn()
        .expect("failed to execute process");
}

pub fn first_clean() {
    Command::new("clear")
        .spawn()
        .expect("failed to execute process");
}
