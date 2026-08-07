/**
 * Minimal multi‑agent workflow manager.
 * Allows defining a sequence of agent commands with optional arguments.
 * Example usage (in a script):
 *   const wm = new WorkflowManager();
 *   wm.addStep('code', { prompt: 'Generate a utility function' });
 *   wm.addStep('test');
 *   await wm.run();
 */
export class WorkflowManager {
  private steps: { name: string; args?: any }[] = [];

  addStep(name: string, args?: any) {
    this.steps.push({ name, args });
  }

  async run() {
    const { MasterAgent } = await import('../../core/src/masterAgent.js');
    const agent = new MasterAgent();
    await agent.init();
    const results: any[] = [];
    for (const step of this.steps) {
      const res = await agent.runCommand(step.name, step.args ?? {});
      results.push(res);
    }
    return results;
  }
}
