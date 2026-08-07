import { LLMProvider } from './provider';
import { OpenAIProvider } from './openai';
import { AnthropicProvider } from './anthropic';
import { GeminiProvider } from './gemini';

export function createProvider(name: string, apiKey?: string): LLMProvider {
  const key = apiKey ?? process.env.OPENAI_API_KEY ?? process.env.ANTHROPIC_API_KEY ?? process.env.GEMINI_API_KEY;
  if (!key) throw new Error('API key not configured – set the appropriate env variable (OPENAI_API_KEY, ANTHROPIC_API_KEY, or GEMINI_API_KEY)');

  switch (name.toLowerCase()) {
    case 'openai':
      return new OpenAIProvider(key);
    case 'anthropic':
      return new AnthropicProvider(key);
    case 'gemini':
      return new GeminiProvider(key);
    default:
      throw new Error(`Unsupported LLM provider: ${name}`);
  }
}
