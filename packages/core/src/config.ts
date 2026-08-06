import { readFile } from 'fs/promises';
import { parse as tomlParse } from '@iarna/toml';
import { load as yamlLoad } from 'js-yaml';

export type Config = Record<string, any>;

export async function loadConfig(path: string): Promise<Config> {
  const data = await readFile(path, 'utf-8');
  if (path.endsWith('.toml')) {
    return tomlParse(data) as Config;
  }
  if (path.endsWith('.yml') || path.endsWith('.yaml')) {
    return yamlLoad(data) as Config;
  }
  throw new Error('Unsupported config format');
}
