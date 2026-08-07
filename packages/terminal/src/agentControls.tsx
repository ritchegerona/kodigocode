import React from 'react';
import { Box, Text, useInput } from 'ink';
import { terminalTheme } from '../../ui/src/theme';
import { MasterAgent } from '../../core/src/masterAgent'; // Assuming MasterAgent exists for dispatch

// Simple AgentControls component with shortcut keys
export const AgentControls = () => {
  // We'll use a local Hook to capture key presses for agent shortcuts
  useInput((input, key) => {
    // Define mapping of keys to agent names
    const mapping: Record<string, string> = {
      c: 'code',
      t: 'test',
      d: 'doc',
      s: 'security',
      g: 'git',
    };
    const agent = mapping[input];
    if (agent) {
      // Fire off the agent via MasterAgent. Ignore errors in UI.
      const ma = new MasterAgent();
      ma.init().then(() => {
        ma.runCommand(agent, { prompt: '' }).catch(() => {});
      });
    }
  });

  return (
    <Box borderStyle="round" borderColor={terminalTheme.border} padding={1} marginTop={1}>
      <Text color={terminalTheme.accent} bold>Agent Controls: </Text>
      <Text>{`[c] Code  `}</Text>
      <Text>{`[t] Test  `}</Text>
      <Text>{`[d] Doc  `}</Text>
      <Text>{`[s] Security  `}</Text>
      <Text>{`[g] Git`}</Text>
    </Box>
  );
};
