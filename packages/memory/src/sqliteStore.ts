import Database from 'better-sqlite3';
import path from 'path';
import { promises as fs } from 'fs';

/**
 * Simple SQLite‑backed memory store.
 * Stores key/value pairs as JSON strings.
 */
export class SQLiteMemoryStore {
  private db: Database.Database;

  constructor(dbPath?: string) {
    const defaultPath = path.join(process.cwd(), 'kodigo_memory.sqlite');
    this.db = new Database(dbPath ?? defaultPath);
    this.db.exec(`CREATE TABLE IF NOT EXISTS kv (key TEXT PRIMARY KEY, value TEXT);`);
  }

  async set(key: string, value: any): Promise<void> {
    const stmt = this.db.prepare('INSERT OR REPLACE INTO kv (key, value) VALUES (?, ?)');
    stmt.run(key, JSON.stringify(value));
  }

  /**
   * No-op: SQLite persists writes immediately in `set()`.
   * Provided so this store satisfies the same async API as the file-based
   * MemoryStore (`load()` / `save()`).
   */
  async load(): Promise<void> {}

  /** No-op: see `load()`. */
  async save(): Promise<void> {}

  async get<T = any>(key: string): Promise<T | undefined> {
    const row = this.db.prepare('SELECT value FROM kv WHERE key = ?').get(key) as
      | { value: string }
      | undefined;
    if (!row) return undefined;
    return JSON.parse(row.value) as T;
  }

  async delete(key: string): Promise<void> {
    this.db.prepare('DELETE FROM kv WHERE key = ?').run(key);
  }

  async clear(): Promise<void> {
    this.db.exec('DELETE FROM kv');
  }
}
