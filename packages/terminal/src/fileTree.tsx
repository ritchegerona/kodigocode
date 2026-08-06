import React from 'react';
import { Box, Text } from 'ink';
import { readdirSync, statSync } from 'fs';
import { join } from 'path';

type FileNode = { name: string; path: string; children?: FileNode[]; isDir: boolean };

function buildTree(dir: string): FileNode[] {
  const entries = readdirSync(dir);
  return entries.map(entry => {
    const full = join(dir, entry);
    const isDir = statSync(full).isDirectory();
    return isDir
      ? { name: entry, path: full, isDir, children: buildTree(full) }
      : { name: entry, path: full, isDir };
  });
}

const renderNode = (node: FileNode, depth = 0): JSX.Element => (
  <Box key={node.path} marginLeft={depth * 2}>
    <Text>{node.isDir ? '📁' : '📄'} {node.name}</Text>
    {node.isDir && node.children?.map(child => renderNode(child, depth + 1))}
  </Box>
);

export const FileTree: React.FC<{ root?: string }> = ({ root = '.' }) => {
  const tree = React.useMemo(() => buildTree(root), [root]);
  return <Box flexDirection="column">{tree.map(node => renderNode(node))}</Box>;
};
