import { promises as fs } from 'fs';
import { join } from 'path';

export class MemoryStore {
  private filePath: string;
  private data: Record<string, any> = {};

  constructor(private readonly dir: string = '.kodigocode') {
    this.filePath = join(this.dir, 'memory.json');
  }

  async load() {
    try {
      const raw = await fs.readFile(this.filePath, 'utf-8');
      this.data = JSON.parse(raw);
    } catch {
      this.data = {};
    }
  }

  async save() {
    await fs.mkdir(this.dir, { recursive: true });
    await fs.writeFile(this.filePath, JSON.stringify(this.data, null, 2), 'utf-8');
  }

  get(key: string) { return this.data[key]; }
  set(key: string, value: any) { this.data[key] = value; }
}
