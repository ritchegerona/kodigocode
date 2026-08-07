import { WebSocketServer } from 'ws';
import { createServer } from 'http';

// Simple MCP (Model Context Protocol) WebSocket server placeholder.
// Listens on port 8765 and echoes received JSON messages.

const httpServer = createServer();
const wss = new WebSocketServer({ server: httpServer });

wss.on('connection', (ws) => {
  ws.on('message', (data) => {
    try {
      const msg = JSON.parse(data.toString());
      // Echo back with a simple acknowledgment.
      const reply = { type: 'ack', received: msg };
      ws.send(JSON.stringify(reply));
    } catch (e) {
      ws.send(JSON.stringify({ error: 'Invalid JSON' }));
    }
  });
});

const PORT = process.env.PORT ? Number(process.env.PORT) : 8765;
httpServer.listen(PORT, () => {
  console.log(`MCP server listening on http://localhost:${PORT}`);
});
