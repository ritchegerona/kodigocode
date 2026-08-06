import { Tool } from './tool';
import { promises as fs } from 'fs';
import { join } from 'path';

export class FsTool implements Tool {
  name = 'fs';
  description = 'Filesystem operations (read, write, rename, delete, search)';
  async run([action, ...args]: any[]): Promise<any> {
    switch (action) {
      case 'read':
        return fs.readFile(args[0], 'utf-8');
      case 'write':
        await fs.writeFile(args[0], args[1], 'utf-8');
        return 'written';
      case 'rename':
        await fs.rename(args[0], args[1]);
        return 'renamed';
      case 'delete':
        await fs.rm(args[0], { recursive: true, force: true });
        return 'deleted';
      default:
        throw new Error(`Unsupported fs action ${action}`);
    }
  }
}
