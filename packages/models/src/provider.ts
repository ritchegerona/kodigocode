export interface LLMProvider {
  name: string;
  generate(prompt: string, options?: any): Promise<string>;
}
