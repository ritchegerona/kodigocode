export type CommandHandler = (...args: any[]) => Promise<any> | any;

export class CommandRegistry {
  private commands = new Map<string, CommandHandler>();

  register(name: string, handler: CommandHandler) {
    this.commands.set(name, handler);
  }

  async execute(name: string, ...args: any[]) {
    const cmd = this.commands.get(name);
    if (!cmd) throw new Error(`Command ${name} not found`);
    return await cmd(...args);
  }

  list() {
    return Array.from(this.commands.keys());
  }
}
