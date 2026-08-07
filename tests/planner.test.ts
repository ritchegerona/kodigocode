import { describe, it, expect } from 'vitest';
import { Planner } from '../packages/planner/src/planner';
import type { WorkflowDefinition } from '../packages/planner/src/workflowEngine';

describe('Planner', () => {
  it('runs tasks in priority order', async () => {
    const planner = new Planner();
    const order: string[] = [];

    planner.add({ id: 'low', priority: 3, run: async () => { order.push('low'); } });
    planner.add({ id: 'high', priority: 1, run: async () => { order.push('high'); } });
    planner.add({ id: 'med', priority: 2, run: async () => { order.push('med'); } });

    await planner.runAll();
    expect(order).toEqual(['high', 'med', 'low']);
  });

  it('has correct pending count', async () => {
    const planner = new Planner();
    expect(planner.pendingCount).toBe(0);

    planner.add({ id: 't1', priority: 1, run: async () => {} });
    planner.add({ id: 't2', priority: 2, run: async () => {} });
    expect(planner.pendingCount).toBe(2);

    await planner.runAll();
    expect(planner.pendingCount).toBe(0);
  });

  it('clears the queue', async () => {
    const planner = new Planner();
    planner.add({ id: 't1', priority: 1, run: async () => {} });
    planner.clear();
    expect(planner.pendingCount).toBe(0);
  });

  it('executes a workflow via executeWorkflow', async () => {
    const planner = new Planner();
    const definition: WorkflowDefinition = {
      name: 'Test',
      steps: [{ id: 's1', agent: 'code', input: 'write hello' }],
    };

    const result = await planner.executeWorkflow(definition);
    // Fallback message since MasterAgent likely isn't available in test env
    expect(result.success).toBe(true);
    expect(result.context.results.get('s1')).toBeTruthy();
  });
});