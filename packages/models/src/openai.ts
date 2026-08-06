import { LLMProvider } from './provider';

export class OpenAIProvider implements LLMProvider {
  name = 'openai';
  constructor(private apiKey: string) {}
  async generate(prompt: string): Promise<string> {
    // Placeholder – real implementation would call OpenAI API.
    return `OpenAI response to: ${prompt}`;
  }
}
