import { Agent } from './agent';
import { Logger } from '../../core/src/logger';
import { exec } from 'child_process';
import { promisify } from 'util';
const execAsync = promisify(exec);

export class TestingAgent extends Agent {
  private logger = new Logger('[Testing]');
  async plan(_context: any): Promise<void> {
    this.logger.info('Planning testing task');
  }
  async execute(_context?: any): Promise<string> {
    this.logger.info('Running tests');
    try {
      const { stdout } = await execAsync('npm run test');
      return stdout;
    } catch (e: any) {
      return e.stderr || e.message;
    }
  }
}
