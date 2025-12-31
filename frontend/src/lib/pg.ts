// Hello Universe - PostgreSQL Client
// Server-side only - Use in Server Components and Server Actions

import { Pool, type PoolConfig } from 'pg';

// Only initialize on server
const isServer = typeof window === 'undefined';

// Database configuration from environment variables
const poolConfig: PoolConfig = {
  connectionString: process.env.DATABASE_URL,
  max: 20, // Maximum number of clients in the pool
  idleTimeoutMillis: 30000, // Close idle clients after 30 seconds
  connectionTimeoutMillis: 2000, // Return an error after 2 seconds if connection not established
  ssl: process.env.NODE_ENV === 'production' ? { rejectUnauthorized: false } : false,
};

// Create a singleton pool instance
let pool: Pool | null = null;

function getPool(): Pool {
  if (!isServer) {
    throw new Error('Database pool should only be used on the server');
  }
  
  if (!pool) {
    pool = new Pool(poolConfig);
    
    // Handle pool errors
    pool.on('error', (err) => {
      console.error('Unexpected error on idle client', err);
    });
  }
  
  return pool;
}

/**
 * Execute a SQL query
 */
export async function query<T = unknown>(
  text: string,
  params?: unknown[]
): Promise<T[]> {
  const client = getPool();
  const start = Date.now();
  
  try {
    const result = await client.query(text, params);
    const duration = Date.now() - start;
    
    if (process.env.NODE_ENV === 'development') {
      console.log('Executed query', { text: text.substring(0, 50), duration, rows: result.rowCount });
    }
    
    return result.rows as T[];
  } catch (error) {
    console.error('Database query error:', error);
    throw error;
  }
}

/**
 * Execute a SQL query and return a single row
 */
export async function queryOne<T = unknown>(
  text: string,
  params?: unknown[]
): Promise<T | null> {
  const rows = await query<T>(text, params);
  return rows[0] ?? null;
}

/**
 * Execute a transaction with multiple queries
 */
export async function transaction<T>(
  callback: (client: {
    query: <R = unknown>(text: string, params?: unknown[]) => Promise<R[]>;
  }) => Promise<T>
): Promise<T> {
  const pool = getPool();
  const client = await pool.connect();
  
  try {
    await client.query('BEGIN');
    
    const result = await callback({
      query: async <R = unknown>(text: string, params?: unknown[]): Promise<R[]> => {
        const res = await client.query(text, params);
        return res.rows as R[];
      },
    });
    
    await client.query('COMMIT');
    return result;
  } catch (error) {
    await client.query('ROLLBACK');
    throw error;
  } finally {
    client.release();
  }
}

/**
 * Check database health
 */
export async function checkHealth(): Promise<boolean> {
  try {
    await query('SELECT 1');
    return true;
  } catch {
    return false;
  }
}

/**
 * Close the database pool (for graceful shutdown)
 */
export async function closePool(): Promise<void> {
  if (pool) {
    await pool.end();
    pool = null;
  }
}

// SQL helper for building queries safely
export const sql = {
  /**
   * Create a parameterized query
   */
  raw: (strings: TemplateStringsArray, ...values: unknown[]): { text: string; values: unknown[] } => {
    let text = '';
    const params: unknown[] = [];
    
    strings.forEach((string, i) => {
      text += string;
      if (i < values.length) {
        params.push(values[i]);
        text += `$${params.length}`;
      }
    });
    
    return { text, values: params };
  },
  
  /**
   * Join multiple values for IN clauses
   */
  in: (values: unknown[]): { placeholders: string; values: unknown[] } => {
    const placeholders = values.map((_, i) => `$${i + 1}`).join(', ');
    return { placeholders, values };
  },
};
