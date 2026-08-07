import { LLMProvider, ChatMessage } from './provider';
import { getAllToolSchemas } from '../../tools/src/toolSchemas';

export class AnthropicProvider implements LLMProvider {
  name = 'anthropic';
  private readonly apiUrl = 'https://api.anthropic.com/v1/messages';
  private model: string;
  constructor(private apiKey: string, model: string = 'claude-3-5-sonnet-20241022') {
    this.model = model;
  }

  async generate(prompt: string, systemPrompt: string = 'You are a helpful coding assistant.'): Promise<string> {
    const body = {
      model: this.model,
      max_tokens: 4096,
      system: systemPrompt,
      messages: [{ role: 'user', content: prompt }],
    };
    const res = await fetch(this.apiUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': this.apiKey,
        'anthropic-version': '2023-06-01',
      },
      body: JSON.stringify(body),
    });
    if (!res.ok) {
      const txt = await res.text();
      throw new Error(`Anthropic API error ${res.status}: ${txt}`);
    }
    const data = await res.json();
    const text = data?.content?.[0]?.text;
    return typeof text === 'string' ? text.trim() : '';
  }

  getFunctionDefinitions(): any[] {
    return getAllToolSchemas().map((schema) => ({
      name: schema.name,
      description: schema.description,
      input_schema: {
        type: 'object',
        properties: schema.parameters.properties,
        required: schema.parameters.required,
      },
    }));
  }

  /**
   * Multi-turn chat with tool support.
   * Converts OpenAI-style messages to Anthropic format and back.
   */
  async chat(
    messages: ChatMessage[],
    tools?: any[],
  ): Promise<{
    role: 'assistant';
    content: string | null;
    tool_calls?: Array<{
      id: string;
      type: 'function';
      function: { name: string; arguments: string };
    }>;
  }> {
    // Extract system message and convert messages to Anthropic format
    let systemPrompt = 'You are a helpful coding assistant with access to tools.';
    const anthropicMessages: any[] = [];

    for (const msg of messages) {
      if (msg.role === 'system') {
        systemPrompt = msg.content;
      } else if (msg.role === 'user') {
        anthropicMessages.push({ role: 'user', content: msg.content });
      } else if (msg.role === 'assistant') {
        if (msg.tool_calls && msg.tool_calls.length > 0) {
          // Convert assistant tool_calls to Anthropic tool_use blocks
          const toolUseBlocks = msg.tool_calls.map((tc) => ({
            type: 'tool_use',
            id: tc.id,
            name: tc.function.name,
            input: JSON.parse(tc.function.arguments),
          }));
          anthropicMessages.push({
            role: 'assistant',
            content: toolUseBlocks,
          });
        } else if (msg.content) {
          anthropicMessages.push({ role: 'assistant', content: msg.content });
        }
      } else if (msg.role === 'tool') {
        // Convert tool result to Anthropic tool_result block
        anthropicMessages.push({
          role: 'user',
          content: [
            {
              type: 'tool_result',
              tool_use_id: msg.tool_call_id,
              content: msg.content,
            },
          ],
        });
      }
    }

    const body: any = {
      model: this.model,
      max_tokens: 4096,
      system: systemPrompt,
      messages: anthropicMessages,
    };

    if (tools && tools.length > 0) {
      body.tools = tools;
    }

    const res = await fetch(this.apiUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': this.apiKey,
        'anthropic-version': '2023-06-01',
      },
      body: JSON.stringify(body),
    });

    if (!res.ok) {
      const txt = await res.text();
      throw new Error(`Anthropic chat error ${res.status}: ${txt}`);
    }

    const data = await res.json();

    // Check for tool_use blocks
    const toolUseBlocks = (data?.content ?? []).filter((b: any) => b.type === 'tool_use');
    if (toolUseBlocks.length > 0) {
      return {
        role: 'assistant',
        content: null,
        tool_calls: toolUseBlocks.map((b: any) => ({
          id: b.id,
          type: 'function' as const,
          function: {
            name: b.name,
            arguments: JSON.stringify(b.input),
          },
        })),
      };
    }

    // Return text content
    const textBlocks = (data?.content ?? []).filter((b: any) => b.type === 'text');
    const text = textBlocks.map((b: any) => b.text).join('\n');
    return { role: 'assistant', content: text || null };
  }

  async functionCalling(functionName: string, args: any): Promise<any> {
    const schemas = getAllToolSchemas();
    const targetSchema = schemas.find((s) => s.name === functionName);

    const body: any = {
      model: this.model,
      max_tokens: 4096,
      system: 'You are a coding assistant with access to tools. Use the provided tool to complete the task.',
      messages: [
        {
          role: 'user',
          content: `Use the tool "${functionName}" with these arguments: ${JSON.stringify(args)}`,
        },
      ],
      tools: this.getFunctionDefinitions(),
    };

    if (targetSchema) {
      body.tool_choice = { type: 'tool', name: functionName };
    }

    const res = await fetch(this.apiUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': this.apiKey,
        'anthropic-version': '2023-06-01',
      },
      body: JSON.stringify(body),
    });

    if (!res.ok) {
      const txt = await res.text();
      throw new Error(`Anthropic function-calling error ${res.status}: ${txt}`);
    }

    const data = await res.json();
    // Extract tool_use blocks from the response
    for (const block of data?.content ?? []) {
      if (block.type === 'tool_use') {
        return block.input;
      }
    }

    // Fallback: return text content
    for (const block of data?.content ?? []) {
      if (block.type === 'text') return block.text.trim();
    }
    return '';
  }
}
