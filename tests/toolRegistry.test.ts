import { describe, it, expect } from 'vitest';
import { ToolRegistry } from '../packages/tools/src/toolRegistry';

describe('ToolRegistry', () => {
  it('returns all registered tool names', () => {
    const registry = new ToolRegistry();
    const names = registry.list();
    expect(names.length).toBeGreaterThan(0);
    expect(names).toContain('fs');
    expect(names).toContain('shell');
    expect(names).toContain('git');
  });

  it('finds tools by name', () => {
    const registry = new ToolRegistry();
    const tool = registry.get('shell');
    expect(tool).toBeDefined();
  });

  it('returns undefined for unknown tools', () => {
    const registry = new ToolRegistry();
    const tool = registry.get('nonexistent');
    expect(tool).toBeUndefined();
  });

  it('returns schemas for tools', () => {
    const registry = new ToolRegistry();
    const schemas = registry.getAllSchemas();
    expect(schemas.length).toBeGreaterThan(0);
    expect(schemas[0]).toHaveProperty('name');
    expect(schemas[0]).toHaveProperty('parameters');
    expect(schemas[0]).toHaveProperty('description');
  });
});
