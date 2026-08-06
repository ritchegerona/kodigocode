import { Agent } from './agent';
import { Logger } from '../../core/src/logger';
import { GitTool } from '../../tools/src/gitTool';

export class GitAgent extends Agent {
  private logger = new Logger('[Git]');
  private gitTool = new GitTool();
  async plan(_context: any): Promise<void> {
    this.logger.info('Planning git task');
  }
  async execute(action: string = 'status'): Promise<any> {
    this.logger.info(`Executing git ${action}`);
    switch (action) {
      case 'status':
        return this.gitTool.run(['status']);
      case 'diff':
        return this.gitTool.run(['diff']);
      case 'add':
        return this.gitTool.run(['add', '.']);
      case 'commit':
        return this.gitTool.run(['commit', '-m', 'Automated commit']);
      default:
        return `Unsupported git action: ${action}`;
    }
  }
}
