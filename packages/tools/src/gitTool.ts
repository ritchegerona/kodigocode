import { Tool } from './tool';
import simpleGit from 'simple-git';

export class GitTool implements Tool {
  name = 'git';
  description = 'Git operations (status, add, commit, diff, checkout)';
  private git = simpleGit();

  async run([action, ...args]: any[]): Promise<any> {
    switch (action) {
      case 'status':
        return this.git.status();
      case 'add':
        await this.git.add(args[0]);
        return 'added';
      case 'commit':
        await this.git.commit(args[0] || 'auto commit');
        return 'committed';
      case 'diff':
        return this.git.diff();
      case 'checkout':
        await this.git.checkout(args[0]);
        return 'checked out';
      default:
        throw new Error(`Unsupported git action ${action}`);
    }
  }
}
