#!/usr/bin/env node
import { createProvider } from '../packages/models/src/providerFactory';
import { MasterAgent } from '../packages/core/src/masterAgent';
import { Logger } from '../packages/core/src/logger';

const logger = new Logger('[kc]');

// Simple argument parser – looks for --model <name>
let model = 'openai';
for (let i = 2; i < process.argv.length; i++) {
  if (process.argv[i] === '--model' && i + 1 < process.argv.length) {
    model = process.argv[i + 1];
    i++;
  }
}

(async () => {
  try {
    const provider = createProvider(model);
    logger.info(`Using model provider: ${provider.name}`);
    const agent = new MasterAgent();
    await agent.init();
    // Example: run a default command, e.g., help
    const result = await agent.runCommand('help');
    console.log(result);
  } catch (e) {
    console.error('Error:', e);
    process.exit(1);
  }
})();
