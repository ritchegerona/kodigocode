import React from 'react';
import { render, Text, Box } from 'ink';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import { MasterAgent } from '../../packages/core/src/masterAgent';

const argv = yargs(hideBin(process.argv))
  .command('run', 'Start the daemon')
  .command('hello', 'Run hello plugin command')
  .command('help', 'Show available commands')
  .help()
  .argv;

const App = () => {
  const [output, setOutput] = React.useState('');
  const [ready, setReady] = React.useState(false);
  React.useEffect(() => {
    (async () => {
      const agent = new MasterAgent();
      await agent.init();
      if (argv._.length > 0) {
        const cmd = argv._[0] as string;
        try {
          const res = await agent.runCommand(cmd);
          setOutput(String(res));
        } catch (e: any) {
          setOutput(`Error: ${e.message}`);
        }
      } else {
        setOutput('No command provided. Use --help.');
      }
      setReady(true);
    })();
  }, []);

  return (
    <Box flexDirection="column">
      <Text>KodigoCode CLI – version {process.env.npm_package_version || '0.1.0'}</Text>
      {ready ? <Text>{output}</Text> : <Text>Initializing...</Text>}
    </Box>
  );
};

render(<App />);
