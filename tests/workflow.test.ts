import { describe, it, expect } from 'vitest';
import { WorkflowEngine } from '../packages/planner/src/workflowEngine';
import type { WorkflowDefinition } from '../packages/planner/src/workflowEngine';

describe('WorkflowEngine', () => {
  it('executes a simple single-step workflow', async () => {
    const runner = async (agent: string, prompt: string): Promise<string> => {
      return `Result from ${agent}: ${prompt}`;
    };

    const engine = new WorkflowEngine(runner);
    const definition: WorkflowDefinition = {
      name: 'Simple',
      steps: [{ id: 'test-step', agent: 'test', input: 'run tests' }],
    };

    const result = await engine.execute(definition);
    expect(result.success).toBe(true);
    expect(result.context.results.get('test-step')).toBe('Result from test: run tests');
    expect(result.errors).toHaveLength(0);
  });

  it('executes sequential steps (then)', async () => {
    const runner = async (agent: string, prompt: string): Promise<string> => {
      return `[${agent}] ${prompt}`;
    };

    const engine = new WorkflowEngine(runner);
    const definition: WorkflowDefinition = {
      name: 'Sequential',
      steps: [
        {
          id: 'step1',
          agent: 'code',
          input: 'generate code',
          then: [
            {
              id: 'step2',
              agent: 'test',
              input: (ctx) => `test: ${ctx.results.get('step1')}`,
            },
          ],
        },
      ],
    };

    const result = await engine.execute(definition);
    expect(result.success).toBe(true);
    expect(result.context.results.get('step1')).toBe('[code] generate code');
    expect(result.context.results.get('step2')).toBe('[test] test: [code] generate code');
  });

  it('handles conditions', async () => {
    const runner = async (agent: string, prompt: string): Promise<string> => prompt;

    const engine = new WorkflowEngine(runner);
    const definition: WorkflowDefinition = {
      name: 'Conditional',
      steps: [
        {
          id: 'always-runs',
          agent: 'chat',
          input: 'hello',
        },
        {
          id: 'conditional',
          agent: 'chat',
          input: 'should not run',
          condition: () => false,
        },
      ],
    };

    const result = await engine.execute(definition);
    expect(result.success).toBe(true);
    expect(result.context.results.get('always-runs')).toBe('hello');
    expect(result.context.results.has('conditional')).toBe(false);
  });

  it('handles errors gracefully', async () => {
    const runner = async (agent: string, _prompt: string): Promise<string> => {
      if (agent === 'bad') throw new Error('Agent failed');
      return 'ok';
    };

    const engine = new WorkflowEngine(runner);
    const definition: WorkflowDefinition = {
      name: 'ErrorHandling',
      steps: [
        { id: 'good-step', agent: 'test', input: 'works' },
        { id: 'bad-step', agent: 'bad', input: 'fails' },
      ],
    };

    const result = await engine.execute(definition);
    expect(result.success).toBe(false);
    expect(result.errors).toHaveLength(1);
    expect(result.errors[0].stepId).toBe('bad-step');
    expect(result.context.results.get('good-step')).toBe('ok');
  });

  it('handles retry with success', async () => {
    let attempts = 0;
    const runner = async (): Promise<string> => {
      attempts++;
      if (attempts < 3) throw new Error('Temporary failure');
      return 'eventual success';
    };

    const engine = new WorkflowEngine(runner);
    const definition: WorkflowDefinition = {
      name: 'Retry',
      steps: [
        {
          id: 'retry-step',
          agent: 'test',
          input: 'retry me',
          retry: { maxAttempts: 3, delayMs: 10 },
        },
      ],
    };

    const result = await engine.execute(definition);
    expect(result.success).toBe(true);
    expect(attempts).toBe(3);
  });

  it('handles timeout', async () => {
    const runner = async (): Promise<string> => {
      return new Promise((resolve) => setTimeout(() => resolve('slow'), 100));
    };

    const engine = new WorkflowEngine(runner);
    const definition: WorkflowDefinition = {
      name: 'Timeout',
      steps: [
        {
          id: 'timed-step',
          agent: 'test',
          input: 'should time out',
          timeoutMs: 10,
        },
      ],
    };

    const result = await engine.execute(definition);
    expect(result.success).toBe(false);
    expect(result.errors[0].stepId).toBe('timed-step');
    expect(result.errors[0].error).toContain('timed out');
  });
});