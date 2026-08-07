import { FsTool } from './fsTool';
import { GitTool } from './gitTool';
import { SearchTool } from './searchTool';
import { ShellTool } from './shellTool';
import { BrowserTool } from './browserTool';
import { SymbolSearchTool } from './symbolSearchTool';
import { Tool } from './tool';
import { toolSchemas, validateToolArgs, getAllToolSchemas, type ToolSchema } from './toolSchemas';

/**
 * Central registry that maps tool names to their instances and JSON schemas.
 * Used by MasterAgent for the `call` command and by function-calling dispatch.
 */
export class ToolRegistry {
  private tools: Map<string, Tool> = new Map();

  constructor() {
    this.tools.set('fs', new FsTool());
    this.tools.set('git', new GitTool());
    this.tools.set('search', new SearchTool());
    this.tools.set('shell', new ShellTool());
    this.tools.set('browser', new BrowserTool());
    this.tools.set('symbol-search', new SymbolSearchTool());
  }

  /** Get a tool instance by name */
  get(name: string): Tool | undefined {
    return this.tools.get(name);
  }

  /** List all registered tool names */
  list(): string[] {
    return Array.from(this.tools.keys());
  }

  /** Get the JSON schema for a specific tool */
  getSchema(name: string): ToolSchema | undefined {
    return toolSchemas[name];
  }

  /** Get all tool schemas (for exposing to LLM function-calling APIs) */
  getAllSchemas(): ToolSchema[] {
    return getAllToolSchemas();
  }

  /**
   * Validate and execute a tool call.
   * @param toolName - The tool to invoke
   * @param args - Arguments matching the tool's schema
   * @returns The tool's result
   */
  async call(toolName: string, args: Record<string, any>): Promise<any> {
    // Validate
    const error = validateToolArgs(toolName, args);
    if (error) throw new Error(`Validation error: ${error}`);

    const tool = this.tools.get(toolName);
    if (!tool) throw new Error(`Tool not found: ${toolName}`);

    // Convert args object to positional array based on schema order
    const schema = toolSchemas[toolName];
    const argArray = schema.parameters.required.map((key) => args[key]);
    // Append any non-required args that were provided
    for (const [key, value] of Object.entries(args)) {
      if (!schema.parameters.required.includes(key) && value !== undefined) {
        argArray.push(value);
      }
    }

    return tool.run(argArray);
  }
}