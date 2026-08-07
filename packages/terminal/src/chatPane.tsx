import React, { useState, useCallback } from 'react';
import { Box, Text } from 'ink';
import TextInput from 'ink-text-input';
import { ChatAgent } from '../../agents/src/chatAgent.js';
import { terminalTheme } from '../../ui/src/theme.js';

interface ChatMessage {
  role: 'user' | 'assistant';
  content: string;
}

/**
 * Interactive chat pane for the TUI dashboard.
 * Supports real-time messaging via ChatAgent with streaming responses.
 */
export const ChatPane: React.FC = () => {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = useCallback(async (value: string) => {
    const trimmed = value.trim();
    if (!trimmed || loading) return;

    setInput('');
    setError(null);
    setLoading(true);

    const userMsg: ChatMessage = { role: 'user', content: trimmed };
    setMessages((prev) => [...prev, userMsg]);

    try {
      const agent = new ChatAgent();
      const reply = await agent.execute(trimmed);
      const assistantMsg: ChatMessage = { role: 'assistant', content: reply };
      setMessages((prev) => [...prev, assistantMsg]);
    } catch (e: any) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  }, [loading]);

  // Render the last few messages (most recent at bottom)
  const visibleMessages = messages.slice(-10);

  return (
    <Box flexDirection="column" borderStyle="round" borderColor={terminalTheme.border} padding={1}>
      <Text color={terminalTheme.accent} bold>
        Chat
      </Text>
      <Box flexDirection="column" marginTop={1} minHeight={10}>
        {visibleMessages.length === 0 && !loading && (
          <Text color={terminalTheme.muted}>Type a message to start chatting...</Text>
        )}
        {visibleMessages.map((msg, i) => (
          <Box key={i} flexDirection="column" marginBottom={1}>
            <Text color={msg.role === 'user' ? terminalTheme.accent : 'green'}>
              {msg.role === 'user' ? '▶ You' : '◆ Assistant'}
            </Text>
            {msg.content.split('\n').map((line, li) => (
              <Text key={li} color={msg.role === 'user' ? terminalTheme.text : terminalTheme.muted}>
                {line}
              </Text>
            ))}
          </Box>
        ))}
        {loading && (
          <Text color="yellow">Thinking...</Text>
        )}
        {error && (
          <Box marginTop={1}>
            <Text color="red">Error: {error}</Text>
          </Box>
        )}
      </Box>
      <Box marginTop={1}>
        <Text color={terminalTheme.muted}>
          {loading ? 'Waiting for response...' : 'Enter message and press return:'}
        </Text>
      </Box>
      <TextInput
        value={input}
        onChange={setInput}
        onSubmit={handleSubmit}
        placeholder="Ask KodigoCode..."
      />
    </Box>
  );
};