import { Tool } from './tool';
import { exec } from 'child_process';
import { promisify } from 'util';
const execAsync = promisify(exec);

export class SearchTool implements Tool {
  name = 'search';
  description = 'Repository search using ripgrep';
  async run([pattern, path = '.'] : any[]): Promise<any> {
    const cmd = `rg "${pattern}" ${path}`;
    try {
      const { stdout } = await execAsync(cmd);
      return stdout;
    } catch (e: any) {
      return e.stdout || '';
    }
  }
}
