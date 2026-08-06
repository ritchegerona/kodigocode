type Task = {
  id: string;
  priority: number;
  run: () => Promise<void>;
};

export class Planner {
  private queue: Task[] = [];

  add(task: Task) {
    this.queue.push(task);
    this.queue.sort((a, b) => a.priority - b.priority);
  }

  async runAll() {
    for (const task of this.queue) {
      await task.run();
    }
  }
}
