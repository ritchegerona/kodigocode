import { describe, it, expect } from 'vitest';
import { terminalTheme } from '../packages/ui/src/theme';

describe('terminal theme', () => {
  it('uses a Claude-inspired teal palette', () => {
    expect(terminalTheme.accent).toBe('#2DD4BF');
    expect(terminalTheme.border).toBe('#2DD4BF');
    expect(terminalTheme.text).toBe('#F8FAFC');
  });
});
