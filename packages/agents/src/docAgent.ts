import { Agent } from './agent';
import { Logger } from '../../core/src/logger';
import { readFile } from 'fs/promises';

export class DocAgent extends Agent {
  private logger = new Logger('[Doc]');
  async plan(_context: any): Promise<void> {
    this.logger.info('Planning documentation task');
  }
  async execute(_context: any): Promise<string> {
    this.logger.info('Generating documentation summary');
    try {
      const readme = await readFile('README.md', 'utf-8');
      const firstLines = readme.split('\n').slice(0, 5).join('\n');
      return `README excerpt:\n${firstLines}`;
    } catch {
      return 'README not found';
    }
  }
}
