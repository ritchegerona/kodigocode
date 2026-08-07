import { createServer } from 'http';
import { WebSocketServer } from 'ws';
import { Logger } from '../../core/src/logger';
import { MasterAgent } from '../../core/src/masterAgent';

const logger = new Logger('[Daemon]');
const port = process.env.KODO_DAEMON_PORT ? parseInt(process.env.KODO_DAEMON_PORT) : 8080;
// Daemon entry point – starts the MCP WebSocket server.
import './server';

async function start() {
  const agent = new MasterAgent();
  await agent.init();

  const server = createServer();
  const wss = new WebSocketServer({ server });

  wss.on('connection', (ws) => {
    logger.info('MCP client connected');
    ws.on('message', async (data) => {
      try {
        const msg = JSON.parse(data.toString());
        const { command, args } = msg;
        logger.info(`MCP request: ${command}`);
        const result = await agent.runCommand(command, ...(args || []));
        ws.send(JSON.stringify({ success: true, result }));
      } catch (e: any) {
        logger.error(`MCP handling error: ${e.message}`);
        ws.send(JSON.stringify({ success: false, error: e.message }));
      }
    });
    ws.on('close', () => logger.info('MCP client disconnected'));
  });

  server.listen(port, () => {
    logger.info(`MCP WebSocket server listening on ws://localhost:${port}`);
  });
}

start().catch((e) => logger.error(`Daemon start failed: ${e.message}`));
