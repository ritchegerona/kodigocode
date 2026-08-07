export interface FunctionCallRequest {
  /** The name of the function/tool to invoke */
  name: string;
  /** Arguments for the function call */
  arguments: Record<string, any>;
}

export interface FunctionCallResponse {
  /** The result of the function call */
  result: any;
  /** Optional error message if the call failed */
  error?: string;
}

/** A single message in a multi-turn conversation */
export interface ChatMessage {
  role: 'system' | 'user' | 'assistant' | 'tool';
  content: string;
  /** Tool call ID (for tool results) */
  tool_call_id?: string;
  /** Tool calls made by the assistant */
  tool_calls?: Array<{
    id: string;
    type: 'function';
    function: { name: string; arguments: string };
  }>;
}

export interface LLMProvider {
  name: string;
  generate(prompt: string, options?: any): Promise<string>;
  /**
   * Optional function calling support. Implementations may choose to ignore this
   * method if they do not provide a function‑calling API. The method receives
   * the name of the function/tool to invoke and an argument object. It should
   * return a JSON‑serialisable result.
   */
  functionCalling?(functionName: string, args: any): Promise<any>;
  /**
   * Optional: expose available function definitions to the LLM.
   * Returns an array of function schemas the provider can use for tool-use.
   */
  getFunctionDefinitions?(): any[];
  /**
   * Optional: multi-turn chat with tool support.
   * Sends messages with tool definitions and returns either a text response
   * or a tool call request. The caller should execute the tool and call again
   * with the tool result appended to messages.
   */
  chat?(messages: ChatMessage[], tools?: any[]): Promise<{
    role: 'assistant';
    content: string | null;
    tool_calls?: Array<{
      id: string;
      type: 'function';
      function: { name: string; arguments: string };
    }>;
  }>;
}
