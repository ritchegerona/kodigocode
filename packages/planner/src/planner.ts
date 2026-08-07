import { WorkflowEngine, WorkflowDefinition, WorkflowResult } from './workflowEngine.js';

type Task = {
  id: string;
  priority: number;
  run: () => Promise<void>;
};

/**
 * Planner that integrates with the WorkflowEngine and MasterAgent.
 *
 * Supports:
 * - Priority-based task queuing
 * - Workflow definition execution via WorkflowEngine
 * - Single-task execution with automatic agent routing
 */
export class Planner {
  private queue: Task[] = [];
  private engine: WorkflowEngine;

  constructor() {
    this.engine = new WorkflowEngine(async (agent: string, prompt: string) => {
      return this.runAgent(agent, prompt);
    });
  }

  /** Add a task to the priority queue */
  add(task: Task) {
    this.queue.push(task);
    this.queue.sort((a, b) => a.priority - b.priority);
  }

  /** Run all queued tasks in priority order */
  async runAll() {
    for (const task of this.queue) {
      await task.run();
    }
  }

  /** Execute a declarative workflow definition */
  async executeWorkflow(definition: WorkflowDefinition): Promise<WorkflowResult> {
    return this.engine.execute(definition);
  }

  /** Clear the task queue */
  clear() {
    this.queue = [];
  }

  /** Number of pending tasks */
  get pendingCount(): number {
    return this.queue.length;
  }

  /**
   * Route an agent command through the MasterAgent.
   * Falls back gracefully if MasterAgent is not available.
   */
  private async runAgent(agent: string, prompt: string): Promise<string> {
    try {
      const { MasterAgent } = await import('../../core/src/masterAgent.js');
      const ma = new MasterAgent();
      await ma.init();
      const result = await ma.runCommand(agent, { prompt });
      return typeof result === 'string' ? result : JSON.stringify(result);
    } catch {
      // Fallback: if MasterAgent is unavailable, return the prompt as-is
      return `[Planner] Could not run agent "${agent}". Prompt: ${prompt}`;
    }
  }
}