import React, { useEffect, useState } from 'react';
import { Box, Text } from 'ink';
import { GitTool } from '../../tools/src/gitTool.js';
import { terminalTheme } from '../../ui/src/theme.js';

interface GitStatus {
  branch: string;
  status: string;
  ahead?: number;
  behind?: number;
  dirtyFiles: number;
  staged: number;
  untrackedFiles: number;
  behindCount?: number;
}

/**
 * Real-time Git status bar for the TUI dashboard.
 * Polls GitTool periodically to show branch, changes, and sync status.
 */
export const GitStatusBar: React.FC = () => {
  const [status, setStatus] = useState<GitStatus | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchStatus = async () => {
      try {
        const git = new GitTool();
        const branchResult = await git.run(['branch']);
        const branch = branchResult.replace('* ', '').trim().split('\n')[0] || 'main';

        const statusOutput = await git.run(['status', '--porcelain']);

        // Parse status output
        const lines = statusOutput.trim().split('\n').filter(Boolean);
        let staged = 0;
        let unstaged = 0;
        let untracked = 0;

        for (const line of lines) {
          const stagedChar = line[0];
          const unstagedChar = line[1];

          if (stagedChar !== ' ' && stagedChar !== '?') staged++;
          if (unstagedChar !== ' ' && unstagedChar !== '?') unstaged++;
          if (stagedChar === '?' && unstagedChar === '?') untracked++;
        }

        let statusLabel = 'clean';
        if (staged > 0 || unstaged > 0 || untracked > 0) {
          statusLabel = 'dirty';
        }

        // Check ahead/behind
        let ahead: number | undefined;
        try {
          const remoteResult = await git.run(['rev-list', '--count', `${branch}..origin/${branch}`]);
          ahead = parseInt(remoteResult.trim(), 10);
        } catch {
          // No remote configured or no upstream
        }

        // Check behind
        let behindCount: number | undefined;
        try {
          const behindResult = await git.run(['rev-list', '--count', `origin/${branch}..${branch}`]);
          behindCount = parseInt(behindResult.trim(), 10);
        } catch {
          // No remote configured
        }

        setStatus({
          branch,
          status: statusLabel,
          ahead: ahead && ahead > 0 ? ahead : undefined,
          behindCount: behindCount && behindCount > 0 ? behindCount : undefined,
          dirtyFiles: staged + unstaged + untracked,
          staged,
          untrackedFiles: untracked,
        });
        setError(null);
      } catch (e: any) {
        setError(e.message);
        setStatus(null);
      }
    };

    fetchStatus();
    const interval = setInterval(fetchStatus, 5000);
    return () => clearInterval(interval);
  }, []);

  if (error) {
    return (
      <Box borderStyle="round" borderColor={terminalTheme.border} padding={1}>
        <Text color={terminalTheme.accent} bold>Git</Text>
        <Text color="grey"> Not available</Text>
      </Box>
    );
  }

  if (!status) {
    return (
      <Box borderStyle="round" borderColor={terminalTheme.border} padding={1}>
        <Text color={terminalTheme.accent} bold>Git</Text>
        <Text color={terminalTheme.muted}> Loading...</Text>
      </Box>
    );
  }

  const statusColor = status.status === 'clean' ? 'green' : 'yellow';

  return (
    <Box borderStyle="round" borderColor={terminalTheme.border} padding={1}>
      <Text color={terminalTheme.accent} bold>Git</Text>
      <Text color={terminalTheme.text}>: </Text>
      <Text color={terminalTheme.accent}>{status.branch}</Text>
      <Text color={terminalTheme.text}> • </Text>
      <Text color={statusColor}>{status.status}</Text>
      {status.staged > 0 && (
        <>
          <Text color={terminalTheme.text}> • </Text>
          <Text color="green">+{status.staged} staged</Text>
        </>
      )}
      {status.dirtyFiles > 0 && status.staged !== status.dirtyFiles && (
        <>
          <Text color={terminalTheme.text}> • </Text>
          <Text color="yellow">~{status.dirtyFiles - status.staged} modified</Text>
        </>
      )}
      {status.untrackedFiles > 0 && (
        <>
          <Text color={terminalTheme.text}> • </Text>
          <Text color="red">?{status.untrackedFiles} new</Text>
        </>
      )}
      {status.ahead && (
        <>
          <Text color={terminalTheme.text}> • </Text>
          <Text color="cyan">↑{status.ahead}</Text>
        </>
      )}
      {status.behindCount && (
        <>
          <Text color={terminalTheme.text}> • </Text>
          <Text color="cyan">↓{status.behindCount}</Text>
        </>
      )}
    </Box>
  );
};