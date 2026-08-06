export interface Tool {
  name: string;
  description: string;
  run(args: any[]): Promise<any>;
}
