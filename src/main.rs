mod cli;
mod tool;
mod tools;
mod plugin;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logger
    env_logger::init();

    // Build CLI parser
    let matches = cli::build_cli().get_matches();

    // Create tool registry and register built‑in tools
    let mut registry = tool::ToolRegistry::new();
    tools::git_tool::register_git_tool(&mut registry);

    match matches.subcommand() {
        Some(("version", _)) => {
            println!("openclaude {}", env!("CARGO_PKG_VERSION"));
        }
        Some(("run", sub_m)) => {
            let tool_name = sub_m.get_one::<String>("tool").expect("tool arg");
            // Collect remaining args after the tool name
            let tool_args: Vec<String> = sub_m.get_raw("args")
                .map(|vals| vals.map(|v| v.to_string_lossy().to_string()).collect())
                .unwrap_or_default();
            match registry.get(tool_name.as_str()) {
                Some(tool) => {
                    match tool.run(&tool_args).await {
                        Ok(output) => println!("{}", output),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                None => eprintln!("Unknown tool: {}", tool_name),
            }
        }
        Some(("load-plugins", sub_m)) => {
            let dir = sub_m.get_one::<String>("dir").expect("dir arg");
            let path = std::path::Path::new(dir);
            match plugin::load_plugins(path) {
                Ok(tools) => {
                    for tool in tools {
                        registry.register_boxed(tool);
                    }
                    println!("Loaded plugins from {}", dir);
                }
                Err(e) => eprintln!("Failed to load plugins: {}", e),
            }
        },
        _ => unreachable!(),
    }

    Ok(())
}
