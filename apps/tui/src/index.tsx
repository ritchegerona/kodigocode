import React from 'react';
import { render, Box, Text } from 'ink';
import { FileTree } from '../../packages/terminal/src/fileTree';
import { DiffViewer } from '../../packages/terminal/src/diffViewer';

const Dashboard = () => (
  <Box flexDirection="column">
    <Text>KodigoCode TUI Dashboard (prototype)</Text>
    <Box marginTop={1}>
      <FileTree root="./" />
    </Box>
    <Box marginTop={2}>
      <DiffViewer />
    </Box>
  </Box>
);

render(<Dashboard />);
