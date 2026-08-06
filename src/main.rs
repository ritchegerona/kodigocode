mod cli;
mod config;
mod plugin;
mod tool;
mod tools;

use std::io::{self, Write};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("fatal: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    let cfg = config::load()?;
    log::info!(
        "Config loaded | log_level={} plugins_dir={}",
        cfg.log_level,
        cfg.plugins_dir.display()
    );

    config::ensure_plugins_dir(&cfg)?;

    let matches = cli::build_cli().get_matches();

    let mut registry = tool::ToolRegistry::new();
    tools::git_tool::register_git_tool(&mut registry);

    match matches.subcommand() {
        Some(("version", _)) => {
            println!("openclaude {}", env!("CARGO_PKG_VERSION"));
        }

        Some(("run", sub_m)) => {
            let tool_name = sub_m.get_one::<String>("tool").expect("tool arg");
            let tool_args: Vec<String> = sub_m
                .get_raw("args")
                .map(|vals| vals.map(|v| v.to_string_lossy().to_string()).collect())
                .unwrap_or_default();

            match registry.get(tool_name.as_str()) {
                Some(t) => match t.run(&tool_args).await {
                    Ok(output) => println!("{}", output),
                    Err(e) => {
                        log::error!("tool '{}' failed: {}", tool_name, e);
                        eprintln!("Error: {}", e);
                    }
                },
                None => eprintln!("Unknown tool: {}", tool_name),
            }
        }

        Some(("load-plugins", sub_m)) => {
            let dir = sub_m.get_one::<String>("dir").expect("dir arg");
            let path = std::path::Path::new(dir);
            match plugin::load_plugins(path) {
                Ok(tools) => {
                    let count = tools.len();
                    for t in tools {
                        registry.register_boxed(t);
                    }
                    println!("Loaded {} plugin(s) from {}", count, dir);
                }
                Err(e) => {
                    log::error!("plugin_load: {}", e);
                    eprintln!("Failed to load plugins: {}", e);
                }
            }
        }

        Some(("list", _)) => {
            let tools = registry.list();
            if tools.is_empty() {
                println!("No tools registered.");
            } else {
                println!("{:<20} {:<15} {}", "NAME", "SOURCE", "DESCRIPTION");
                for t in &tools {
                    println!("{:<20} {:<15} {}", t.name, t.source, t.description);
                }
            }
        }

        Some(("interactive", _)) => {
            interactive_repl(&mut registry).await?;
        }

        Some(("discover-plugins", sub_m)) => {
            let dir = sub_m
                .get_one::<String>("dir")
                .map(|s| s.as_str())
                .unwrap_or_else(|| cfg.plugins_dir.to_str().unwrap_or("."));
            match plugin::discover_plugins(std::path::Path::new(dir)) {
                Ok(plugins) => {
                    if plugins.is_empty() {
                        println!("No .so plugin files found in '{}'.", dir);
                    } else {
                        println!("Discovered {} plugin(s) in '{}':", plugins.len(), dir);
                        for p in &plugins {
                            println!("  {}", p);
                        }
                    }
                }
                Err(e) => eprintln!("Failed to scan plugins: {}", e),
            }
        }

        _ => {
            interactive_repl(&mut registry).await?;
        }
    }

    Ok(())
}

async fn interactive_repl(registry: &mut tool::ToolRegistry) -> Result<(), anyhow::Error> {
    println!("Entering interactive mode. Type 'help' or '?' for available commands.");
    println!("Type 'exit' or 'quit' to leave.\n");
    let stdin = io::stdin();
    loop {
        print!(">> ");
        io::stdout().flush()?;
        let mut line = String::new();
        if stdin.read_line(&mut line).is_err() {
            break;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        match trimmed {
            "exit" | "quit" => break,
            "help" | "?" => {
                println!("Available commands:");
                println!("  list               List registered tools");
                println!("  run <tool> [args]  Execute a tool");
                println!("  help               Show this help");
                println!("  exit / quit        Leave interactive mode");
            }
            "list" => {
                let tools = registry.list();
                if tools.is_empty() {
                    println!("No tools registered.");
                } else {
                    for t in &tools {
                        println!("  {}  —  {}", t.name, t.description);
                    }
                }
            }
            cmd if cmd.starts_with("run ") => {
                let rest = &cmd[4..];
                let parts: Vec<&str> = rest.split_whitespace().collect();
                if parts.is_empty() {
                    eprintln!("Usage: run <tool_name> [args...]");
                    continue;
                }
                let tool_name = parts[0];
                let tool_args: Vec<String> =
                    parts[1..].iter().map(|s| s.to_string()).collect();

                match registry.get(tool_name) {
                    Some(t) => match t.run(&tool_args).await {
                        Ok(output) => println!("{}", output),
                        Err(e) => eprintln!("Error: {}", e),
                    },
                    None => eprintln!("Unknown tool: {}", tool_name),
                }
            }
            _ => {
                eprintln!(
                    "Unknown command '{}'. Type 'help' for commands.",
                    trimmed
                );
            }
        }
    }
    println!("Goodbye.");
    Ok(())
}