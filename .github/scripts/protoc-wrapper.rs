use std::{env, fs};
use std::path::PathBuf;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let exe_dir = match env::current_exe().and_then(|p| {
        p.parent()
            .map(PathBuf::from)
            .ok_or_else(|| std::io::Error::other("missing executable directory"))
    }) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("protoc wrapper: {e}");
            return ExitCode::from(1);
        }
    };
    let real = exe_dir.join("protoc-real.exe");
    let mut args = Vec::new();
    let mut dep_file: Option<PathBuf> = None;

    for arg in env::args_os().skip(1) {
        let text = arg.to_string_lossy();
        if text == "--dependency_out=/dev/stdout" {
            let path = env::temp_dir().join(format!(
                "grok-protoc-{}-{}.d",
                std::process::id(),
                unique_suffix()
            ));
            args.push(format!("--dependency_out={}", path.display()).into());
            dep_file = Some(path);
        } else if text == "--descriptor_set_out=/dev/null" {
            args.push("--descriptor_set_out=NUL".into());
        } else {
            args.push(arg);
        }
    }

    let status = match Command::new(&real).args(&args).status() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("protoc wrapper: failed to execute {}: {e}", real.display());
            return ExitCode::from(1);
        }
    };

    if !status.success() {
        return ExitCode::from(status.code().unwrap_or(1) as u8);
    }

    if let Some(path) = dep_file {
        match fs::read_to_string(&path) {
            Ok(contents) => print!("{contents}"),
            Err(e) => {
                eprintln!(
                    "protoc wrapper: failed to read dependency file {}: {e}",
                    path.display()
                );
                return ExitCode::from(1);
            }
        }
        let _ = fs::remove_file(path);
    }

    ExitCode::SUCCESS
}

fn unique_suffix() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}
