use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("kc")
        .about("Multi-provider AI chat TUI with pluggable tool system")
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
        .subcommand(
            Command::new("setup")
                .about("Configure API keys and base URLs")
                .subcommand(
                    Command::new("key")
                        .about("Persist an API key for a provider")
                        .arg(Arg::new("provider").required(true).help("Provider name (e.g. openai, deepseek)"))
                        .arg(Arg::new("key").required(true).help("API key value")),
                )
                .subcommand(
                    Command::new("url")
                        .about("Persist a base URL override for a provider")
                        .arg(Arg::new("provider").required(true).help("Provider name"))
                        .arg(Arg::new("url").required(true).help("Base URL")),
                )
                .subcommand(
                    Command::new("show")
                        .about("Show stored config (keys redacted)")
                        .arg(Arg::new("provider").required(false).help("Filter by provider")),
                ),
        )
}