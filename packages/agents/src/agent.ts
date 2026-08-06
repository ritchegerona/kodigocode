export abstract class Agent {
  abstract plan(context: any): Promise<void>;
  abstract execute(context: any): Promise<void>;
}
