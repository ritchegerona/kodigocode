import React from 'react';
import { Box, Text } from 'ink';
import { terminalTheme } from '../../ui/src/theme';

// Placeholder WorkflowPanel – displays a static header. Later it will connect to the real workflow context.
export const WorkflowPanel = () => (
  <Box borderStyle="round" borderColor={terminalTheme.border} padding={1} marginBottom={1}>
    <Text color={terminalTheme.accent} bold>
      Workflow Panel (coming soon)
    </Text>
  </Box>
);
