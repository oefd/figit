use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::Mutex;

static INSTALLED: Mutex<Vec<String>> = Mutex::new(vec![]);

/// Ensure all packages are installed, returning a vec
/// of packages that were not already installed.
pub fn ensure_installed(pkgs: &Vec<String>) -> Vec<String> {
    let mut installed = INSTALLED.lock().unwrap();
    if installed.len() == 0 {
        populate_installed(&mut installed);
    }

    let mut newly_installed = vec![];
    let mut install_cmd = Command::new("pacman");
    install_cmd
        .stdout(Stdio::piped())
        .arg("-S")
        .arg("--noconfirm");
    for pkg in pkgs.iter().filter(|pkg| !installed.contains(pkg)) {
        newly_installed.push(pkg.clone());
        install_cmd.arg(pkg);
    }

    if newly_installed.len() > 0 {
        let status = install_cmd.spawn().unwrap().wait().unwrap();
        assert!(status.success());
        populate_installed(&mut installed);
    }

    newly_installed
}

fn populate_installed(installed: &mut Vec<String>) {
    let mut handle = Command::new("pacman")
        .stdout(Stdio::piped())
        .arg("-Qqs")
        .spawn()
        .unwrap();
    let mut stdout = handle.stdout.take().unwrap();
    let mut output = "".to_string();
    installed.truncate(0);

    stdout.read_to_string(&mut output).unwrap();
    output
        .lines()
        .map(String::from)
        .for_each(|line| installed.push(line));
}
