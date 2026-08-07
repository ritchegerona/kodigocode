/**
 * JSON Schema definitions for each tool, used for function-calling validation
 * and for exposing tool capabilities to LLM providers.
 */

export interface ToolSchema {
  name: string;
  description: string;
  parameters: {
    type: 'object';
    properties: Record<string, { type: string; description: string; enum?: string[] }>;
    required: string[];
  };
}

export const toolSchemas: Record<string, ToolSchema> = {
  fs: {
    name: 'fs',
    description: 'Filesystem operations: read, write, rename, delete files',
    parameters: {
      type: 'object',
      properties: {
        action: {
          type: 'string',
          description: 'The filesystem action to perform',
          enum: ['read', 'write', 'rename', 'delete'],
        },
        path: { type: 'string', description: 'The file path to operate on' },
        content: { type: 'string', description: 'Content to write (required for write action)' },
        newPath: { type: 'string', description: 'New path for rename action' },
      },
      required: ['action', 'path'],
    },
  },
  git: {
    name: 'git',
    description: 'Git operations: status, diff, log, commit, branch, checkout, pull, push',
    parameters: {
      type: 'object',
      properties: {
        action: {
          type: 'string',
          description: 'The git action to perform',
          enum: ['status', 'diff', 'log', 'commit', 'branch', 'checkout', 'pull', 'push'],
        },
        message: { type: 'string', description: 'Commit message (required for commit)' },
        branch: { type: 'string', description: 'Branch name (required for checkout/branch)' },
      },
      required: ['action'],
    },
  },
  search: {
    name: 'search',
    description: 'Search the codebase for files or text patterns',
    parameters: {
      type: 'object',
      properties: {
        query: { type: 'string', description: 'The search query or pattern' },
        type: {
          type: 'string',
          description: 'Type of search',
          enum: ['file', 'text', 'symbol'],
        },
      },
      required: ['query'],
    },
  },
  shell: {
    name: 'shell',
    description: 'Execute a shell command and return the output',
    parameters: {
      type: 'object',
      properties: {
        command: { type: 'string', description: 'The shell command to execute' },
        cwd: { type: 'string', description: 'Working directory for the command' },
      },
      required: ['command'],
    },
  },
  browser: {
    name: 'browser',
    description: 'Open a URL or perform browser-based actions',
    parameters: {
      type: 'object',
      properties: {
        action: {
          type: 'string',
          description: 'The browser action',
          enum: ['open', 'screenshot', 'click', 'type', 'read'],
        },
        url: { type: 'string', description: 'URL to navigate to' },
        selector: { type: 'string', description: 'CSS selector for click/type actions' },
        text: { type: 'string', description: 'Text to type' },
      },
      required: ['action'],
    },
  },
  'symbol-search': {
    name: 'symbol-search',
    description: 'Search for code symbols (functions, classes, variables) across the workspace',
    parameters: {
      type: 'object',
      properties: {
        query: { type: 'string', description: 'The symbol name or pattern to search for' },
      },
      required: ['query'],
    },
  },
};

/** Returns all tool schemas as an array (for LLM function-calling APIs) */
export function getAllToolSchemas(): ToolSchema[] {
  return Object.values(toolSchemas);
}

/** Validate arguments against a tool's JSON schema. Returns null if valid, or an error string. */
export function validateToolArgs(toolName: string, args: Record<string, any>): string | null {
  const schema = toolSchemas[toolName];
  if (!schema) return `Unknown tool: ${toolName}`;

  // Check required fields
  for (const required of schema.parameters.required) {
    if (!(required in args) || args[required] === undefined || args[required] === '') {
      return `Missing required parameter: ${required}`;
    }
  }

  // Check enum constraints
  for (const [key, prop] of Object.entries(schema.parameters.properties)) {
    if (prop.enum && args[key] !== undefined && !prop.enum.includes(args[key])) {
      return `Invalid value for ${key}: "${args[key]}". Allowed: ${prop.enum.join(', ')}`;
    }
  }

  return null; // valid
}