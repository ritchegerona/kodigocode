import { describe, it, expect } from 'vitest';
import { loadConfig } from '../packages/core/src/config';

describe('Config loader', () => {
  it('parses toml correctly', async () => {
    const cfg = await loadConfig('kodigo.toml');
    expect(cfg.model.provider).toBe('openai');
  });
});
