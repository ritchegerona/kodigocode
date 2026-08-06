import React, { useEffect, useState } from 'react';
import { Box, Text } from 'ink';
import { GitTool } from '../../tools/src/gitTool';

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
      <Text>Git Diff:</Text>
      <Box borderStyle="round" padding={1} marginTop={1}>
        <Text>{diff || 'No changes'}</Text>
      </Box>
    </Box>
  );
};
