import React from 'react';
import { Box, Text } from 'ink';
import { terminalTheme } from '../../ui/src/theme';

export const Spinner = ({ label }: { label: string }) => (
  <Box>
    <Text color={terminalTheme.accent}>{label} ...</Text>
  </Box>
);
