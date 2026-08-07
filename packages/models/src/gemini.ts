import { LLMProvider, ChatMessage } from './provider';
import { getAllToolSchemas } from '../../tools/src/toolSchemas';

export class GeminiProvider implements LLMProvider {
  name = 'gemini';
  private readonly apiUrl = 'https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent';
  constructor(private apiKey: string) {}

  async generate(prompt: string, systemPrompt: string = 'You are a helpful coding assistant.'): Promise<string> {
    const body = {
      system_instruction: { parts: [{ text: systemPrompt }] },
      contents: [{ parts: [{ text: prompt }] }],
    };
    const res = await fetch(`${this.apiUrl}?key=${this.apiKey}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) {
      const txt = await res.text();
      throw new Error(`Gemini API error ${res.status}: ${txt}`);
    }
    const data = await res.json();
    const text = data?.candidates?.[0]?.content?.parts?.[0]?.text;
    return typeof text === 'string' ? text.trim() : '';
  }

  getFunctionDefinitions(): any[] {
    return getAllToolSchemas().map((schema) => ({
      name: schema.name,
      description: schema.description,
      parameters: {
        type: 'object',
        properties: schema.parameters.properties,
        required: schema.parameters.required,
      },
    }));
  }

  /**
   * Multi-turn chat with tool support for Gemini.
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
    // Build Gemini-formatted contents from ChatMessage history
    const systemMessages = messages.filter((m) => m.role === 'system');
    const systemInstruction = systemMessages.length > 0
      ? { parts: systemMessages.map((m) => ({ text: m.content })) }
      : { parts: [{ text: 'You are a helpful coding assistant with access to tools.' }] };

    const geminiContents: any[] = [];
    for (const msg of messages) {
      if (msg.role === 'system') continue;

      if (msg.role === 'user') {
        geminiContents.push({ role: 'user', parts: [{ text: msg.content }] });
      } else if (msg.role === 'assistant') {
        const parts: any[] = [];
        if (msg.content) {
          parts.push({ text: msg.content });
        }
        if (msg.tool_calls && msg.tool_calls.length > 0) {
          for (const tc of msg.tool_calls) {
            parts.push({
              functionCall: {
                name: tc.function.name,
                args: JSON.parse(tc.function.arguments),
              },
            });
          }
        }
        geminiContents.push({ role: 'model', parts: parts.length > 0 ? parts : [{ text: '' }] });
      } else if (msg.role === 'tool') {
        geminiContents.push({
          role: 'function',
          parts: [{
            functionResponse: {
              name: msg.tool_call_id ?? 'unknown',
              response: { result: msg.content },
            },
          }],
        });
      }
    }

    const body: any = {
      system_instruction: systemInstruction,
      contents: geminiContents,
    };

    if (tools && tools.length > 0) {
      body.tools = [{ functionDeclarations: tools }];
    }

    const res = await fetch(`${this.apiUrl}?key=${this.apiKey}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (!res.ok) {
      const txt = await res.text();
      throw new Error(`Gemini chat error ${res.status}: ${txt}`);
    }

    const data = await res.json();
    const candidate = data?.candidates?.[0];
    const parts = candidate?.content?.parts ?? [];

    // Check for function calls
    const functionCalls: Array<{
      id: string;
      type: 'function';
      function: { name: string; arguments: string };
    }> = [];

    for (const part of parts) {
      if (part.functionCall) {
        functionCalls.push({
          id: part.functionCall.name + '_' + Date.now(),
          type: 'function' as const,
          function: {
            name: part.functionCall.name,
            arguments: JSON.stringify(part.functionCall.args || {}),
          },
        });
      }
    }

    if (functionCalls.length > 0) {
      return { role: 'assistant', content: null, tool_calls: functionCalls };
    }

    const text = parts
      .filter((p: any) => p.text)
      .map((p: any) => p.text)
      .join('\n');

    return { role: 'assistant', content: text || null };
  }

  async functionCalling(functionName: string, args: any): Promise<any> {
    const schemas = getAllToolSchemas();
    const targetSchema = schemas.find((s) => s.name === functionName);

    const toolConfig: any = { functionDeclarations: this.getFunctionDefinitions() };

    const body: any = {
      system_instruction: {
        parts: [{ text: 'You are a coding assistant with access to tools. Use the provided function to complete the task.' }],
      },
      contents: [
        {
          parts: [{ text: `Call the function "${functionName}" with these arguments: ${JSON.stringify(args)}` }],
        },
      ],
      tools: [toolConfig],
    };

    if (targetSchema) {
      body.tool_config = {
        function_calling_config: {
          mode: 'ANY',
          allowed_function_names: [functionName],
        },
      };
    }

    const res = await fetch(`${this.apiUrl}?key=${this.apiKey}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (!res.ok) {
      const txt = await res.text();
      throw new Error(`Gemini function-calling error ${res.status}: ${txt}`);
    }

    const data = await res.json();
    const parts = data?.candidates?.[0]?.content?.parts ?? [];
    for (const part of parts) {
      if (part.functionCall) {
        return part.functionCall.args;
      }
    }

    // Fallback: return text
    for (const part of parts) {
      if (part.text) return part.text.trim();
    }
    return '';
  }
}
