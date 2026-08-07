/**
 * Multi-Agent Workflow Engine
 *
 * Defines a declarative workflow format where multiple agents can be chained
 * together with conditional branching, parallel execution, and data passing
 * between steps.
 */

export interface WorkflowStep {
  /** Unique step identifier */
  id: string;
  /** Agent name to invoke (e.g., 'code', 'test', 'doc', 'security', 'git', 'chat') */
  agent: string;
  /** Prompt or input to pass to the agent */
  input: string | ((ctx: WorkflowContext) => string);
  /** Condition that must be true for this step to run */
  condition?: (ctx: WorkflowContext) => boolean;
  /** Steps to run after this one completes (sequential) */
  then?: WorkflowStep[];
  /** Steps to run in parallel after this one completes */
  parallel?: WorkflowStep[];
  /** Retry configuration */
  retry?: { maxAttempts: number; delayMs: number };
  /** Timeout in milliseconds */
  timeoutMs?: number;
}

export interface WorkflowDefinition {
  name: string;
  description?: string;
  /** Entry point steps (run in parallel) */
  steps: WorkflowStep[];
  /** Error handler */
  onError?: (error: Error, ctx: WorkflowContext) => Promise<void>;
}

export interface WorkflowContext {
  /** Results from each step, keyed by step ID */
  results: Map<string, any>;
  /** Shared data between steps */
  data: Record<string, any>;
  /** Workflow start time */
  startTime: number;
}

export interface WorkflowResult {
  success: boolean;
  context: WorkflowContext;
  errors: Array<{ stepId: string; error: string }>;
  durationMs: number;
}

/**
 * Executes a workflow definition by traversing the step graph.
 */
export class WorkflowEngine {
  private agentRunner: (agent: string, prompt: string) => Promise<string>;

  constructor(agentRunner: (agent: string, prompt: string) => Promise<string>) {
    this.agentRunner = agentRunner;
  }

  async execute(definition: WorkflowDefinition): Promise<WorkflowResult> {
    const ctx: WorkflowContext = {
      results: new Map(),
      data: {},
      startTime: Date.now(),
    };
    const errors: Array<{ stepId: string; error: string }> = [];

    try {
      await this.runSteps(definition.steps, ctx, errors);
    } catch (e: any) {
      if (definition.onError) {
        await definition.onError(e, ctx);
      }
      errors.push({ stepId: '__workflow__', error: e.message });
    }

    return {
      success: errors.length === 0,
      context: ctx,
      errors,
      durationMs: Date.now() - ctx.startTime,
    };
  }

  private async runSteps(
    steps: WorkflowStep[],
    ctx: WorkflowContext,
    errors: Array<{ stepId: string; error: string }>,
  ): Promise<void> {
    // Run all steps in this level in parallel
    const promises = steps.map((step) => this.runStep(step, ctx, errors));
    await Promise.all(promises);
  }

  private async runStep(
    step: WorkflowStep,
    ctx: WorkflowContext,
    errors: Array<{ stepId: string; error: string }>,
  ): Promise<void> {
    // Check condition
    if (step.condition && !step.condition(ctx)) {
      return;
    }

    // Resolve input (can be a string or a function of context)
    const input = typeof step.input === 'function' ? step.input(ctx) : step.input;

    // Execute with retry
    let result: string | undefined;
    let lastError: Error | undefined;
    const maxAttempts = step.retry?.maxAttempts ?? 1;
    const delayMs = step.retry?.delayMs ?? 0;

    for (let attempt = 0; attempt < maxAttempts; attempt++) {
      try {
        const runPromise = this.agentRunner(step.agent, input);
        if (step.timeoutMs) {
          result = await this.timeout(runPromise, step.timeoutMs);
        } else {
          result = await runPromise;
        }
        lastError = undefined;
        break;
      } catch (e: any) {
        lastError = e;
        if (attempt < maxAttempts - 1 && delayMs > 0) {
          await this.sleep(delayMs);
        }
      }
    }

    if (lastError) {
      errors.push({ stepId: step.id, error: lastError.message });
      ctx.results.set(step.id, `ERROR: ${lastError.message}`);
      return;
    }

    ctx.results.set(step.id, result!);

    // Run sequential follow-up steps
    if (step.then && step.then.length > 0) {
      for (const nextStep of step.then) {
        await this.runStep(nextStep, ctx, errors);
      }
    }

    // Run parallel follow-up steps
    if (step.parallel && step.parallel.length > 0) {
      await this.runSteps(step.parallel, ctx, errors);
    }
  }

  private timeout<T>(promise: Promise<T>, ms: number): Promise<T> {
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => reject(new Error(`Step timed out after ${ms}ms`)), ms);
      promise.then(
        (val) => { clearTimeout(timer); resolve(val); },
        (err) => { clearTimeout(timer); reject(err); },
      );
    });
  }

  private sleep(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
}

/**
 * Predefined workflow templates for common development tasks.
 */
export const WorkflowTemplates = {
  /** Full code review: lint → security audit → test */
  codeReview: (target: string): WorkflowDefinition => ({
    name: 'Code Review',
    description: `Full review of ${target}`,
    steps: [
      {
        id: 'security-check',
        agent: 'security',
        input: `Review the following code for security vulnerabilities:\n${target}`,
        parallel: [
          {
            id: 'doc-check',
            agent: 'doc',
            input: `Generate documentation for:\n${target}`,
          },
        ],
      },
      {
        id: 'test-gen',
        agent: 'test',
        input: `Generate tests for:\n${target}`,
      },
    ],
  }),

  /** Feature implementation workflow */
  featureImpl: (description: string): WorkflowDefinition => ({
    name: 'Feature Implementation',
    description: `Implement: ${description}`,
    steps: [
      {
        id: 'plan',
        agent: 'chat',
        input: `Create an implementation plan for: ${description}`,
        then: [
          {
              id: 'implement',
              agent: 'code',
              input: (ctx) => `Implement the following feature based on this plan:\n${ctx.results.get('plan')}\n\nFeature: ${description}`,
            then: [
              {
                  id: 'test',
                  agent: 'test',
                  input: (ctx) => `Generate tests for this implementation:\n${ctx.results.get('implement')}`,
              },
              {
                  id: 'review',
                  agent: 'security',
                  input: (ctx) => `Security review of:\n${ctx.results.get('implement')}`,
              },
            ],
          },
        ],
      },
    ],
  }),

  /** Git commit workflow */
  commit: (message: string): WorkflowDefinition => ({
    name: 'Smart Commit',
    description: `Commit: ${message}`,
    steps: [
      {
        id: 'diff',
        agent: 'git',
        input: 'diff',
        then: [
          {
              id: 'review-diff',
              agent: 'chat',
              input: (ctx) => `Review this git diff and suggest improvements:\n${ctx.results.get('diff')}`,
          },
          {
              id: 'commit',
              agent: 'git',
              input: `commit ${message}`,
          },
        ],
      },
    ],
  }),
};