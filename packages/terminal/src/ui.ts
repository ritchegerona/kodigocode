import React from 'react';
import { Box, Text } from 'ink';

export const Spinner = ({ label }: { label: string }) => (
  <Box>
    <Text>{label} ...</Text>
  </Box>
);
