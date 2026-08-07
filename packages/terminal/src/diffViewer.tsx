import React, { useEffect, useState } from 'react';
import { Box, Text } from 'ink';
import { GitTool } from '../../tools/src/gitTool';
import { terminalTheme } from '../../ui/src/theme';

export const DiffViewer: React.FC = () => {
  const [diff, setDiff] = useState<string>('');
  useEffect(() => {
    const run = async () => {
      const tool = new GitTool();
      const out = await tool.run(['diff']);
      setDiff(String(out));
    };
    run();
  }, []);
  return (
    <Box flexDirection="column">
      <Text color={terminalTheme.accent}>Changes</Text>
      <Box borderStyle="round" borderColor={terminalTheme.border} padding={1} marginTop={1}>
        <Text color={diff ? terminalTheme.text : terminalTheme.muted}>
          {diff || 'No pending changes'}
        </Text>
      </Box>
    </Box>
  );
};
