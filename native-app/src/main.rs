use std::process::ExitCode;
use tokio::runtime::{self, Runtime};

fn main() -> ExitCode {
    let script_path: String = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => return ExitCode::from(42),
    };

    let runtime: Runtime = match runtime::Builder::new_current_thread().enable_io().build() {
        Ok(n) => n,
        Err(_) => return ExitCode::from(43),
    };

    let rt_done: ExitCode = runtime.block_on(coroutine(script_path));

    return rt_done;
}

async fn coroutine(script_path: String) -> ExitCode {
    let mut cmd = tokio::process::Command::new("node");
    cmd.current_dir("/");
    cmd.args(vec!["--experimental-strip-types", &script_path]);

    match cmd.output().await {
        Ok(output) => match output.status.success() {
            true => {
                let stdout_utf8: String = match String::from_utf8(output.stdout) {
                    Ok(n) => n,
                    Err(_) => return ExitCode::from(44),
                };

                let _stderr_utf8: String = match String::from_utf8(output.stderr) {
                    Ok(n) => n,
                    Err(_) => return ExitCode::from(45),
                };

                // eprintln!("{stderr_utf8}");
                println!("{stdout_utf8}");

                ExitCode::SUCCESS
            }
            false => match output.status.code() {
                Some(exit_code) => {
                    let exit_code: u8 = match exit_code.try_into() {
                        Ok(n) => n,
                        Err(_) => return ExitCode::from(46),
                    };
                    ExitCode::from(exit_code)
                }
                None => ExitCode::from(47),
            },
        },
        Err(_) => ExitCode::from(48),
    }
}
