# KodigoCode – Feature Roadmap

The following high‑level capabilities are planned for the next phases of KodigoCode.  Each item can be broken down into concrete engineering tasks (e.g., design, implementation, tests, documentation).

## 1️⃣ Interactive Chat Support ✅ Completed
- **ChatAgent** (already added) – LLM‑backed conversational interface.
- Persistent chat history stored in the **project memory** store.
- REPL‑style CLI mode (`kc chat`) that keeps the session open.
- UI integration: embed a chat pane in the TUI dashboard.

## 2️⃣ Tool Calling (function calling) ✅ Completed
- Extend the `LLMProvider` interface to support a `functionCalling` mode where the model can request execution of a registered tool (e.g., `fs.read`, `git.diff`).
- Register built‑in tools (`FsTool`, `GitTool`, `SearchTool`, `ShellTool`) with the `MasterAgent` so the LLM can invoke them.
- Define a JSON schema for each tool to enable proper argument validation.

## 3️⃣ Git Integration ✅ Implemented baseline
- Full‑featured `GitAgent` with actions: `status`, `diff`, `add`, `commit`, `branch`, `checkout`, `push`, `pull`.
- Expose git commands as CLI sub‑commands (`kc git <action>`).
- Show real‑time git status in the TUI dashboard (status bar, badge icons).

## 4️⃣ Multi‑File Editing ✅ Completed
- Add a **file editor** component to the TUI (using Ink + `ink-text-input` or a terminal editor library).
- Enable the `CodingAgent` to generate edits for multiple files (return a JSON diff format).  The agent will write those changes via the `FsTool`.
- Implement safe write‑back with backups and a preview diff before applying.

## 5️⃣ Natural Language Coding ✅ Basic implementation
- Enhance `CodingAgent` to accept natural‑language prompts and produce code snippets, file creations, or refactorings.
- Use the **symbol‑search** tool to locate relevant symbols before generating code.
- Provide a `kc codegen "<prompt>"` command that writes the result to a new file or updates an existing one.

## 6️⃣ MCP (Model‑Client‑Plugin) Support ✅ Completed
- Define a lightweight **MCP protocol** (JSON‑RPC over WebSocket) to allow external language models or services to communicate with KodigoCode.
- Implement a server (`apps/daemon`) that listens for MCP connections and routes requests to agents/tools.
- Allow the user to configure external endpoints in `kodigo.toml`.

## 7️⃣ Multi‑Model Support ✅ Completed
- Extend `providerFactory` to instantiate different providers (OpenAI, Anthropic, Google Gemini, Ollama, etc.) based on the `[model]` section of the config.
- Provide a CLI flag `--model <name>` to override the default provider for a single command.
- Add a **model registry** (`packages/models/src/registry.ts`) that maps model names to provider classes.

## 8️⃣ Plugin Marketplace ✅ Completed
- Establish a `plugins/` directory convention where each subfolder is an npm‑style plugin exposing a `KodigoPlugin`.
- Create a **PluginManager UI** in the TUI to list installed plugins, enable/disable them, and view metadata.
- Publish a simple **plugin manifest** (`plugins/marketplace.json`) that can be consumed by a future web‑based marketplace.
- Add CLI commands `kc plugins list`, `kc plugins install <pkg>`, `kc plugins remove <pkg>`.

## 9️⃣ Multi‑Agent Workflows
- Design a **workflow engine** that can chain agents (e.g., `chat → codegen → test → git commit`).
- Provide a declarative YAML workflow syntax (`workflows/*.yml`).
- Implement a **Planner** that schedules agents via the existing `Planner` class, respecting priorities and dependencies.

## 🔟 Project Memory ✅ Completed
- Upgrade the `MemoryStore` to a persistent JSON database (e.g., SQLite via `better-sqlite3`).
- Store chat history, generated code fragments, symbol indexes, and user preferences.
- Expose memory queries to agents (`memory.get`, `memory.set`).

## 🖥️ Visual TUI Dashboard
- Refactor the TUI to a **dashboard layout** with panes:
  - File tree (left)
  - Editor / diff viewer (center)
  - Chat pane (right)
  - Git status bar (bottom)
- Use `ink-box` or `ink-spinner` for nice UI components.
- Add keyboard shortcuts for common actions (e.g., `c` for codegen, `t` for test, `g` for git).

## 📦 Additional Engineering Tasks
- Add proper linting (`eslint`) and formatting (`prettier`).
- Write comprehensive integration tests for agent orchestration.
- CI pipeline enhancements (coverage, security scanning).
- Documentation: detailed developer guide, plugin authoring guide, MCP spec.
- Release automation (version bump, changelog generation).

---

**Recommended next steps**
- Implement function‑calling bridge in `ChatAgent` (dispatch to `MasterAgent.runCommand('call'…)`).
- Build the multi‑file editing UI & JSON‑diff flow.
- Upgrade `MemoryStore` to SQLite (`better-sqlite3`).
- Add plugin marketplace CLI (`plugins list/install/remove`).
- Polish the TUI dashboard (chat pane, status pane, shortcuts).
- Add unit tests for provider factory, command registration, and function‑calling.
- (Optional) flesh‑out Anthropic / Gemini providers when API keys are available.


These items form the backbone of the next major release of KodigoCode.  Each can be prioritized and broken into smaller tickets as the project progresses.
