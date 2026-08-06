import { describe, it, expect } from 'vitest';
import { MasterAgent } from '../packages/core/src/masterAgent';

describe('MasterAgent', () => {
  it('loads plugins and runs hello command', async () => {
    const agent = new MasterAgent();
    await agent.init();
    const res = await agent.runCommand('hello');
    expect(res).toContain('Hello');
  });
});
