import { Tool } from './tool';

export class BrowserTool implements Tool {
  name = 'browser';
  description = 'Perform HTTP GET requests';
  async run([url]: any[]): Promise<any> {
    const res = await fetch(url);
    const text = await res.text();
    return text;
  }
}
