import React, { useState } from 'react';
import { render, Box, Text, useInput } from 'ink';
import { FileTree } from '../../../packages/terminal/src/fileTree.tsx';
import { DiffViewer } from '../../../packages/terminal/src/diffViewer.tsx';
import { ChatPane } from '../../../packages/terminal/src/chatPane.tsx';
import { GitStatusBar } from '../../../packages/terminal/src/gitStatus.tsx';
import { MultiFileEditor } from '../../../packages/terminal/src/multiFileEditor.tsx';
import { terminalTheme } from '../../../packages/ui/src/theme.ts';
import { WorkflowPanel } from '../../../packages/terminal/src/workflowPanel.tsx';
import { AgentControls } from '../../../packages/terminal/src/agentControls.tsx';

type ActivePane = 'files' | 'editor' | 'chat' | 'diff';

const ShortcutBar = () => (
  <Box flexDirection="row" borderStyle="round" borderColor={terminalTheme.border} padding={1}>
    <Text color={terminalTheme.accent} bold>Shortcuts: </Text>
    <Text color={terminalTheme.muted}>1</Text>
    <Text color={terminalTheme.text}> FileTree </Text>
    <Text color={terminalTheme.muted}>2</Text>
    <Text color={terminalTheme.text}> Editor </Text>
    <Text color={terminalTheme.muted}>3</Text>
    <Text color={terminalTheme.text}> Chat </Text>
    <Text color={terminalTheme.muted}>4</Text>
    <Text color={terminalTheme.text}> Diff </Text>
    <Text color={terminalTheme.muted}>q</Text>
    <Text color={terminalTheme.text}> Quit </Text>
    <Text color={terminalTheme.muted}>?</Text>
    <Text color={terminalTheme.text}> Help </Text>
  </Box>
);

const Dashboard = () => {
  const [activePane, setActivePane] = useState<ActivePane>('chat');
  const [showHelp, setShowHelp] = useState(false);
  const [editorPrompt, setEditorPrompt] = useState<string | undefined>();

  useInput((input, key) => {
    if (input === '1') {
      setActivePane('chat');
      setShowHelp(false);
    } else if (input === '2') {
      setActivePane('diff');
      setShowHelp(false);
    } else if (input === '3') {
      setEditorPrompt('Help me implement a utility module');
      setActivePane('chat');
      setShowHelp(false);
    } else if (input === '4') {
      setActivePane('diff');
      setShowHelp(false);
    } else if (input === 'g') {
      setActivePane('chat');
    } else if (input === 'c') {
      setEditorPrompt('Generate code for me');
    } else if (input === 'd') {
      setActivePane('diff');
    } else if (input === 'q') {
      process.exit(0);
    } else if (input === '?') {
      setShowHelp((prev) => !prev);
    } else if (key.escape) {
      setShowHelp(false);
    }
  });

  if (showHelp) {
    return (
      <Box flexDirection="column" padding={1}>
        <Box borderStyle="round" borderColor={terminalTheme.border} padding={1}>
          <Text color={terminalTheme.accent} bold>KodigoCode Help</Text>
        </Box>
        <Box flexDirection="column" marginTop={1} borderStyle="round" borderColor={terminalTheme.border} padding={1}>
          <Text color={terminalTheme.accent}>Keyboard Shortcuts</Text>
          <Text color={terminalTheme.text}>━━━━━━━━━━━━━━━━━━━━━━━━</Text>
          <Text color={terminalTheme.muted}>1  </Text><Text color={terminalTheme.text}>Focus Chat pane</Text>
          <Text color={terminalTheme.muted}>2  </Text><Text color={terminalTheme.text}>Focus Editor pane</Text>
          <Text color={terminalTheme.muted}>3  </Text><Text color={terminalTheme.text}>Code generation prompt</Text>
          <Text color={terminalTheme.muted}>4  </Text><Text color={terminalTheme.text}>Focus Diff viewer</Text>
          <Text color={terminalTheme.muted}>c  </Text><Text color={terminalTheme.text}>Trigger code generation</Text>
          <Text color={terminalTheme.muted}>g  </Text><Text color={terminalTheme.text}>Focus Git status</Text>
          <Text color={terminalTheme.muted}>d  </Text><Text color={terminalTheme.text}>View Git diff</Text>
          <Text color={terminalTheme.muted}>?  </Text><Text color={terminalTheme.text}>Toggle help</Text>
          <Text color={terminalTheme.muted}>q  </Text><Text color={terminalTheme.text}>Quit</Text>
          <Text color={terminalTheme.muted}>Esc </Text><Text color={terminalTheme.text}>Close overlay</Text>
          <Box marginTop={1}>
            <Text color={terminalTheme.muted}>Press Esc or ? to return</Text>
          </Box>
        </Box>
      </Box>
    );
  }

  return (
    <Box flexDirection="column" padding={1}>
      <Box borderStyle="round" borderColor={terminalTheme.border} padding={1}>
        <Text color={terminalTheme.accent} bold>KodigoCode</Text>
        <Text color={terminalTheme.muted}> • agentic coding workspace</Text>
      </Box>

      <Box marginTop={1} flexDirection="row">
        <Box
          width="25%"
          borderStyle="round"
          borderColor={activePane === 'chat' ? terminalTheme.accent : terminalTheme.border}
          padding={1}
          marginRight={1}
        >
          <FileTree root="./" />
        </Box>
        <Box
          width="50%"
          borderStyle="round"
          borderColor={activePane === 'diff' ? terminalTheme.accent : terminalTheme.border}
          padding={1}
          marginRight={1}
          flexDirection="column"
        >
          <WorkflowPanel />
          {activePane === 'diff' ? <DiffViewer /> : <MultiFileEditor prompt={editorPrompt} />}
          <AgentControls />
        </Box>
        <Box
          width="25%"
          borderStyle="round"
          borderColor={activePane === 'chat' ? terminalTheme.accent : terminalTheme.border}
          padding={1}
        >
          <ChatPane />
        </Box>
      </Box>
      <Box marginTop={1}>
        <GitStatusBar />
      </Box>
      <Box marginTop={1}>
        <ShortcutBar />
      </Box>
    </Box>
  );
};

render(<Dashboard />);