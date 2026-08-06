import { Tool } from './tool';
import { readdirSync, readFileSync, statSync } from 'fs';
import { join, extname } from 'path';

/**
 * Very lightweight symbol search implementation.
 * It walks the workspace (default "src") and extracts lines that look like
 * exported symbols (functions, classes, const/let) using a simple RegExp.
 * This is a pragmatic placeholder – a full tree‑sitter integration can be
 * swapped in later without changing the public Tool interface.
 */
export class SymbolSearchTool implements Tool {
  name = 'symbol-search';
  description = 'Search exported symbols in TypeScript/JavaScript files';

  private readonly exportRegex = /export\s+(?:const|let|var|function|class)\s+(\w+)/g;

  async run([query = '', root = 'src'] : any[]): Promise<any> {
    const symbols: string[] = [];
    const walk = (dir: string) => {
      for (const entry of readdirSync(dir)) {
        const full = join(dir, entry);
        const stats = statSync(full);
        if (stats.isDirectory()) {
          // ignore node_modules and .git
          if (entry === 'node_modules' || entry.startsWith('.')) continue;
          walk(full);
        } else if (stats.isFile() && ['.ts', '.tsx', '.js', '.jsx'].includes(extname(entry))) {
          const content = readFileSync(full, 'utf8');
          let match: RegExpExecArray | null;
          while ((match = this.exportRegex.exec(content))) {
            symbols.push(`${match[1]} (${full})`);
          }
        }
      }
    };
    const start = join(process.cwd(), root);
    try { walk(start); } catch { /* ignore missing dirs */ }
    if (!query) return symbols;
    const lower = query.toLowerCase();
    return symbols.filter(s => s.toLowerCase().includes(lower));
  }
}
