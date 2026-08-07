import { Agent } from './agent';
import { Logger } from '../../core/src/logger';
import { loadConfig } from '../../core/src/config';
import { createProvider } from '../../models/src/providerFactory';
import { SQLiteMemoryStore } from '../../memory/src/sqliteStore';
import { ToolRegistry } from '../../tools/src/toolRegistry';
import type { ChatMessage, LLMProvider } from '../../models/src/provider';

const MAX_TOOL_ITERATIONS = 10;

export class ChatAgent extends Agent {
  private logger = new Logger('[Chat]');
  private history: ChatMessage[] = [];
  private memory = new SQLiteMemoryStore();
  private memoryLoaded = false;
  private toolRegistry = new ToolRegistry();

  async plan(_context: any): Promise<void> {
    this.logger.info('Planning chat interaction');
  }

  /**
   * Execute a chat turn with multi-turn function-calling support.
   *
   * Flow:
   * 1. Send user message + tool definitions to the LLM
   * 2. If the LLM responds with tool_calls, execute them via ToolRegistry
   * 3. Feed tool results back to the LLM
   * 4. Repeat until the LLM produces a final text response (or max iterations)
   */
  async execute(msg: string = ''): Promise<string> {
    if (!msg) {
      return 'Please provide a message to chat with.';
    }

    // Load persisted history once per session
    if (!this.memoryLoaded) {
      await this.memory.load();
      const saved = this.memory.get('chatHistory');
      if (Array.isArray(saved)) this.history = saved;
      this.memoryLoaded = true;
    }

    // Append user message to history
    this.history.push({ role: 'user', content: msg });

    // Load provider config
    const { provider, providerName } = await this.loadProvider();

    // Build tool definitions for the LLM
    const tools = this.buildToolDefinitions(provider);

    // Multi-turn function-calling loop
    let finalResponse: string | null = null;
    let iterations = 0;

    while (iterations < MAX_TOOL_ITERATIONS) {
      iterations++;

      // Call the LLM with current history + tools
      const response = await this.callLLM(provider, tools);

      // If the LLM returned tool calls, execute them
      if (response.tool_calls && response.tool_calls.length > 0) {
        this.logger.info(
          `Tool calls requested: ${response.tool_calls.map((tc) => tc.function.name).join(', ')}`,
        );

        // Record the assistant's tool call request in history
        this.history.push({
          role: 'assistant',
          content: response.content ?? '',
          tool_calls: response.tool_calls,
        });

        // Execute each tool call and append results
        for (const toolCall of response.tool_calls) {
          const toolResult = await this.executeToolCall(
            toolCall.function.name,
            toolCall.function.arguments,
          );

          this.history.push({
            role: 'tool',
            content: JSON.stringify(toolResult),
            tool_call_id: toolCall.id,
          });
        }

        // Continue loop — the LLM will process tool results
        continue;
      }

      // No tool calls — this is the final text response
      finalResponse = response.content ?? '';
      this.history.push({ role: 'assistant', content: finalResponse });
      break;
    }

    if (iterations >= MAX_TOOL_ITERATIONS && finalResponse === null) {
      finalResponse =
        'I tried to complete your request but it required too many tool calls. Please try a more specific query.';
      this.history.push({ role: 'assistant', content: finalResponse });
    }

    // Persist updated history
    this.memory.set('chatHistory', this.history);
    await this.memory.save();

    return finalResponse ?? 'No response generated.';
  }

  /**
   * Load the configured LLM provider.
   */
  private async loadProvider(): Promise<{ provider: LLMProvider; providerName: string }> {
    let providerName = 'openai';
    try {
      const cfg = await loadConfig('kodigo.toml');
      if (cfg?.model?.provider) providerName = cfg.model.provider;
    } catch (e) {
      this.logger.warn('Failed to load config, using default provider');
    }
    const provider = createProvider(providerName);
    return { provider, providerName };
  }

  /**
   * Build tool definitions in the format expected by the LLM.
   * Uses the provider's own getFunctionDefinitions() if available,
   * otherwise falls back to raw schemas.
   */
  private buildToolDefinitions(provider: LLMProvider): any[] {
    if (provider.getFunctionDefinitions) {
      return provider.getFunctionDefinitions();
    }
    // Fallback: use raw schemas
    return this.toolRegistry.getAllSchemas().map((schema) => ({
      type: 'function',
      function: {
        name: schema.name,
        description: schema.description,
        parameters: schema.parameters,
      },
    }));
  }

  /**
   * Execute a single tool call and return the result.
   */
  private async executeToolCall(
    toolName: string,
    argumentsJson: string,
  ): Promise<{ success: boolean; result?: any; error?: string }> {
    try {
      const args = JSON.parse(argumentsJson);
      const result = await this.toolRegistry.call(toolName, args);
      return { success: true, result };
    } catch (e: any) {
      this.logger.error(`Tool call failed: ${toolName} — ${e.message}`);
      return { success: false, error: e.message };
    }
  }

  /**
   * Call the LLM. Falls back to generate() if chat() is not available.
   */
  private async callLLM(
    provider: LLMProvider,
    tools: any[],
  ): Promise<{
    role: 'assistant';
    content: string | null;
    tool_calls?: Array<{
      id: string;
      type: 'function';
      function: { name: string; arguments: string };
    }>;
  }> {
    if (provider.chat) {
      return provider.chat(this.history, tools);
    }

    // Fallback for providers without chat() support (e.g., Gemini stub)
    const prompt = this.history.map((m) => `${m.role}: ${m.content}`).join('\n');
    const text = await provider.generate(prompt);
    return { role: 'assistant', content: text };
  }
}
