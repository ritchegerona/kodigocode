import { Agent } from './agent';
import { Logger } from '../../core/src/logger';
import { loadConfig } from '../../core/src/config';
import { createProvider } from '../../models/src/providerFactory';

export class CodingAgent extends Agent {
  private logger = new Logger('[Coding]');

  async plan(_context: any): Promise<void> {
    this.logger.info('Planning coding task');
  }

  // Accept a prompt string; if omitted, return a helpful message
  async execute(prompt: string = ''): Promise<string> {
    if (!prompt) {
      this.logger.warn('No prompt provided to CodingAgent');
      return 'Please provide a coding prompt.';
    }

    // Load provider configuration (defaults to openai)
    let providerName = 'openai';
    try {
      const cfg = await loadConfig('kodigo.toml');
      if (cfg?.model?.provider) providerName = cfg.model.provider;
    } catch (e) {
      this.logger.warn('Failed to load config, using default provider');
    }

    const provider = createProvider(providerName);
    this.logger.info(`Generating code via ${providerName}`);
    const result = await provider.generate(prompt);
    return result;
  }

  // Produce a JSON diff for multi‑file edits by asking the LLM
  async generateDiff(prompt: string = ''): Promise<any> {
    if (!prompt) {
      return { changes: [] };
    }

    // Load provider configuration (defaults to openai)
    let providerName = 'openai';
    try {
      const cfg = await loadConfig('kodigo.toml');
      if (cfg?.model?.provider) providerName = cfg.model.provider;
    } catch (e) {
      this.logger.warn('Failed to load config, using default provider');
    }

    const provider = createProvider(providerName);
    this.logger.info(`Generating diff via ${providerName}`);

    const systemPrompt = `You are a coding assistant that outputs multi-file edits as JSON.
Given a user prompt describing changes, return a JSON object with a "changes" array.
Each change has:
- "path": the file path (string)
- "action": one of "add", "edit", or "delete" (string)
- "content": the full file content after the change (string, not required for "delete")

Example response format:
{
  "changes": [
    { "path": "src/utils.ts", "action": "add", "content": "export function hello() { return 'world'; }" },
    { "path": "README.md", "action": "edit", "content": "# Updated README\\n..." }
  ]
}

Return ONLY the JSON object, no other text.`;

    const result = await provider.generate(prompt, systemPrompt);

    // Try to parse the LLM output as JSON
    try {
      // Strip markdown code fences if present
      const cleaned = result
        .replace(/^```json\s*/i, '')
        .replace(/^```\s*/i, '')
        .replace(/\s*```$/, '')
        .trim();

      const parsed = JSON.parse(cleaned);
      if (parsed.changes && Array.isArray(parsed.changes)) {
        return parsed;
      }
    } catch {
      this.logger.warn('Failed to parse diff JSON, returning raw output');
    }

    // Fallback: treat the whole output as a single file write
    return {
      changes: [
        {
          path: 'generated.txt',
          action: 'add',
          content: result,
        },
      ],
    };
  }
}
