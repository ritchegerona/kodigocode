import { LLMProvider, ChatMessage } from './provider';
import { getAllToolSchemas } from '../../tools/src/toolSchemas';

export class OpenAIProvider implements LLMProvider {
  name = 'openai';
  private readonly apiUrl = 'https://api.openai.com/v1/chat/completions';
  private model: string;
  constructor(private apiKey: string, model: string = 'gpt-4o-mini') {
    this.model = model;
  }

  async generate(prompt: string, systemPrompt: string = 'You are a helpful coding assistant.') {
    const body = {
      model: this.model,
      messages: [
        { role: 'system', content: systemPrompt },
        { role: 'user', content: prompt },
      ],
      temperature: 0.2,
    };
    const res = await fetch(this.apiUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${this.apiKey}`,
      },
      body: JSON.stringify(body),
    });
    if (!res.ok) {
      const txt = await res.text();
      throw new Error(`OpenAI API error ${res.status}: ${txt}`);
    }
    const data = await res.json();
    // Extract the assistant message content
    const message = data?.choices?.[0]?.message?.content;
    return typeof message === 'string' ? message.trim() : '';
  }

  /** Expose tool schemas as OpenAI-compatible function definitions */
  getFunctionDefinitions(): any[] {
    return getAllToolSchemas().map((schema) => ({
      type: 'function',
      function: {
        name: schema.name,
        description: schema.description,
        parameters: schema.parameters,
      },
    }));
  }

  /**
   * Multi-turn chat with tool support.
   * Sends messages with tool definitions and returns either a text response
   * or a tool call request.
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
    const body: any = {
      model: this.model,
      messages,
      temperature: 0.2,
    };

    if (tools && tools.length > 0) {
      body.tools = tools;
      body.tool_choice = 'auto';
    }

    const res = await fetch(this.apiUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${this.apiKey}`,
      },
      body: JSON.stringify(body),
    });

    if (!res.ok) {
      const txt = await res.text();
      throw new Error(`OpenAI chat error ${res.status}: ${txt}`);
    }

    const data = await res.json();
    const choice = data?.choices?.[0]?.message;
    if (!choice) throw new Error('No response from OpenAI');

    return {
      role: 'assistant',
      content: choice.content ?? null,
      tool_calls: choice.tool_calls ?? undefined,
    };
  }

  /**
   * Execute a function call by sending a chat completion request with
   * the function definitions and a prompt that triggers the tool use.
   */
  async functionCalling(functionName: string, args: any): Promise<any> {
    const schemas = getAllToolSchemas();
    const targetSchema = schemas.find((s) => s.name === functionName);

    const body: any = {
      model: this.model,
      messages: [
        {
          role: 'system',
          content: 'You are a coding assistant with access to tools. Use the provided function to complete the task.',
        },
        {
          role: 'user',
          content: `Call the function "${functionName}" with these arguments: ${JSON.stringify(args)}`,
        },
      ],
      tools: this.getFunctionDefinitions(),
      tool_choice: targetSchema ? { type: 'function', function: { name: functionName } } : 'auto',
      temperature: 0.2,
    };

    const res = await fetch(this.apiUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${this.apiKey}`,
      },
      body: JSON.stringify(body),
    });

    if (!res.ok) {
      const txt = await res.text();
      throw new Error(`OpenAI function-calling error ${res.status}: ${txt}`);
    }

    const data = await res.json();
    const toolCalls = data?.choices?.[0]?.message?.tool_calls;
    if (toolCalls && toolCalls.length > 0) {
      const tc = toolCalls[0];
      try {
        return JSON.parse(tc.function.arguments);
      } catch {
        return tc.function.arguments;
      }
    }

    // Fallback: return the text response
    const message = data?.choices?.[0]?.message?.content;
    return typeof message === 'string' ? message.trim() : '';
  }
}
