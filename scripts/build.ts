import { execSync } from 'child_process';
import { readdirSync } from 'fs';
import { join } from 'path';

import { execSync } from 'child_process';
import { readdirSync } from 'fs';
import { join } from 'path';

const workspaceRoots = ['packages', 'apps'];
for (const root of workspaceRoots) {
  const rootPath = join(process.cwd(), root);
  const dirs = readdirSync(rootPath, { withFileTypes: true })
    .filter(d => d.isDirectory())
    .map(d => d.name);
  for (const dir of dirs) {
    const pkgPath = join(rootPath, dir);
    // Attempt to run tsc -b if tsconfig exists
    try {
      execSync('bun', { cwd: pkgPath, stdio: 'ignore' }); // ensure bun is available
      execSync('bun x tsc -b', { cwd: pkgPath, stdio: 'inherit' });
    } catch {
      // No TypeScript build for this package – ignore
    }
  }
}
console.log('Build complete');
