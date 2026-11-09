use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Match foreground AI clients to their X11 kitty window, independently of titles.
pub(crate) fn foreground_clients() -> BTreeMap<i64, String> {
    clients_in(Path::new("/proc"))
}

fn clients_in(proc_dir: &Path) -> BTreeMap<i64, String> {
    let mut clients = BTreeMap::new();
    let Ok(entries) = fs::read_dir(proc_dir) else {
        return clients;
    };
    for entry in entries.flatten() {
        if entry.file_name().to_string_lossy().parse::<u32>().is_err() {
            continue;
        }
        let path = entry.path();
        let Ok(comm) = fs::read_to_string(path.join("comm")) else {
            continue;
        };
        let program = comm.trim();
        if !matches!(program, "codex" | "ollama") {
            continue;
        }
        let Ok(stat) = fs::read_to_string(path.join("stat")) else {
            continue;
        };
        if !is_foreground(&stat) {
            continue;
        }
        let Ok(environ) = fs::read(path.join("environ")) else {
            continue;
        };
        let vars: Vec<_> = environ.split(|b| *b == 0).collect();
        if !vars.iter().any(|var| var.starts_with(b"KITTY_WINDOW_ID=")) {
            continue;
        }
        if let Some(window_id) = vars.iter().find_map(|var| {
            std::str::from_utf8(var.strip_prefix(b"WINDOWID=")?)
                .ok()?
                .parse::<i64>()
                .ok()
        }) {
            clients.insert(window_id, program.to_owned());
        }
    }
    clients
}

fn is_foreground(stat: &str) -> bool {
    // comm is parenthesized and may itself contain spaces or parentheses.
    let Some((_, fields)) = stat.rsplit_once(") ") else {
        return false;
    };
    let fields: Vec<_> = fields.split_whitespace().take(6).collect();
    if fields.len() != 6 || matches!(fields[0], "T" | "t" | "Z" | "X") {
        return false;
    }
    let pgrp = fields[2].parse::<i32>().unwrap_or(0);
    let tty = fields[4].parse::<i32>().unwrap_or(0);
    let foreground_pgrp = fields[5].parse::<i32>().unwrap_or(0);
    tty != 0 && pgrp > 0 && pgrp == foreground_pgrp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_live_foreground_terminal_processes_match() {
        assert!(is_foreground("42 (codex) S 1 42 1 34818 42 0"));
        assert!(is_foreground("42 (name (with) spaces) S 1 42 1 34818 42"));
        assert!(!is_foreground("42 (ollama) S 1 42 42 0 -1"));
        assert!(!is_foreground("42 (codex) S 1 42 1 34818 99"));
        assert!(!is_foreground("42 (codex) T 1 42 1 34818 42"));
        assert!(!is_foreground("42 (codex) Z 1 42 1 34818 42"));
        assert!(!is_foreground("invalid"));
    }

    #[test]
    fn clients_require_a_foreground_process_and_kitty_window() {
        let root = std::env::temp_dir().join(format!("workstyle-proc-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        for (pid, comm, stat, env) in [
            (
                1,
                "codex",
                "1 (codex) S 0 1 1 34818 1",
                "KITTY_WINDOW_ID=1\0WINDOWID=123\0",
            ),
            (
                2,
                "ollama",
                "2 (ollama) S 0 2 2 34819 2",
                "KITTY_WINDOW_ID=1\0WINDOWID=456\0",
            ),
            (
                3,
                "ollama",
                "3 (ollama) S 0 3 3 0 -1",
                "KITTY_WINDOW_ID=1\0WINDOWID=789\0",
            ),
            (
                4,
                "codex",
                "4 (codex) S 0 4 4 34820 9",
                "KITTY_WINDOW_ID=1\0WINDOWID=789\0",
            ),
            (5, "codex", "5 (codex) S 0 5 5 34820 5", "WINDOWID=789\0"),
            (
                6,
                "zsh",
                "6 (zsh) S 0 6 6 34820 6",
                "KITTY_WINDOW_ID=1\0WINDOWID=789\0",
            ),
        ] {
            let path = root.join(pid.to_string());
            fs::create_dir_all(&path).unwrap();
            fs::write(path.join("comm"), comm).unwrap();
            fs::write(path.join("stat"), stat).unwrap();
            fs::write(path.join("environ"), env).unwrap();
        }
        let result = clients_in(&root);
        fs::remove_dir_all(&root).unwrap();
        assert_eq!(
            result,
            BTreeMap::from([(123, "codex".into()), (456, "ollama".into())])
        );
    }
}
