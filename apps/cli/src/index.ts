import React from 'react';
import { render, Text, Box } from 'ink';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';

const argv = yargs(hideBin(process.argv))
  .command('chat [msg]', 'Start chat', (y) => y.positional('msg', { type: 'string' }))
  .command('run', 'Run the daemon')
  .help()
  .argv;

const App = () => (
  <Box>
    <Text>KodigoCode CLI – version {process.env.npm_package_version || '0.1.0'}</Text>
  </Box>
);

render(<App />);
