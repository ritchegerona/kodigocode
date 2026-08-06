import { Tool } from './tool';
import { exec } from 'child_process';
import { promisify } from 'util';
const execAsync = promisify(exec);

export class ShellTool implements Tool {
  name = 'shell';
  description = 'Run arbitrary shell commands';
  async run([cmd]: any[]): Promise<any> {
    const { stdout, stderr } = await execAsync(cmd);
    if (stderr) return stderr;
    return stdout;
  }
}
