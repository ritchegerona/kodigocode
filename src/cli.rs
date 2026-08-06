use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("kodigocode")
        .about("Rust‑based OpenClaude CLI")
        .subcommand(Command::new("version").about("Print version information"))
        .subcommand(
            Command::new("run")
                .about("Execute a tool")
                .arg(
                    Arg::new("tool")
                        .required(true)
                        .help("Tool name to run"),
                )
                .arg(
                    Arg::new("args")
                        .num_args(1..)
                        .help("Arguments to pass to the tool"),
                ),
        )
        .subcommand(
            Command::new("load-plugins")
                .about("Load plugins from a directory")
                .arg(
                    Arg::new("dir")
                        .required(true)
                        .help("Directory containing plugin .so files"),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List all available tools"),
        )
        .subcommand(
            Command::new("interactive")
                .about("Start an interactive REPL session"),
        )
}