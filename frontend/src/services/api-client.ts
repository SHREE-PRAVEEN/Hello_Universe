// Hello Universe - API Client
// Centralized API client for interacting with the Rust backend

import type { ApiResponse, PaginatedResponse, PaginationParams } from '@/types/index.d';

// ============================================
// CONFIGURATION
// ============================================

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';
const API_TIMEOUT = 30000; // 30 seconds

// ============================================
// API CLIENT CLASS
// ============================================

class ApiClient {
  private baseUrl: string;
  private defaultHeaders: HeadersInit;

  constructor(baseUrl: string = API_BASE_URL) {
    this.baseUrl = baseUrl;
    this.defaultHeaders = {
      'Content-Type': 'application/json',
    };
  }

  // Set authorization token
  setAuthToken(token: string | null) {
    if (token) {
      this.defaultHeaders = {
        ...this.defaultHeaders,
        Authorization: `Bearer ${token}`,
      };
    } else {
      const { Authorization, ...rest } = this.defaultHeaders as Record<string, string>;
      this.defaultHeaders = rest;
    }
  }

  // Generic fetch wrapper
  private async fetch<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<ApiResponse<T>> {
    const url = `${this.baseUrl}${endpoint}`;
    
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), API_TIMEOUT);

    try {
      const response = await fetch(url, {
        ...options,
        headers: {
          ...this.defaultHeaders,
          ...options.headers,
        },
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      const data = await response.json();

      if (!response.ok) {
        return {
          success: false,
          data: null as unknown as T,
          error: {
            code: `HTTP_${response.status}`,
            message: data.message || response.statusText,
            details: data.details,
          },
        };
      }

      return {
        success: true,
        data: data as T,
      };
    } catch (error) {
      clearTimeout(timeoutId);

      if (error instanceof Error && error.name === 'AbortError') {
        return {
          success: false,
          data: null as unknown as T,
          error: {
            code: 'TIMEOUT',
            message: 'Request timed out',
          },
        };
      }

      return {
        success: false,
        data: null as unknown as T,
        error: {
          code: 'NETWORK_ERROR',
          message: error instanceof Error ? error.message : 'Network error',
        },
      };
    }
  }

  // HTTP Methods
  async get<T>(endpoint: string, params?: Record<string, string>): Promise<ApiResponse<T>> {
    const url = params
      ? `${endpoint}?${new URLSearchParams(params).toString()}`
      : endpoint;

    return this.fetch<T>(url, { method: 'GET' });
  }

  async post<T>(endpoint: string, body?: unknown): Promise<ApiResponse<T>> {
    return this.fetch<T>(endpoint, {
      method: 'POST',
      body: body ? JSON.stringify(body) : undefined,
    });
  }

  async put<T>(endpoint: string, body?: unknown): Promise<ApiResponse<T>> {
    return this.fetch<T>(endpoint, {
      method: 'PUT',
      body: body ? JSON.stringify(body) : undefined,
    });
  }

  async patch<T>(endpoint: string, body?: unknown): Promise<ApiResponse<T>> {
    return this.fetch<T>(endpoint, {
      method: 'PATCH',
      body: body ? JSON.stringify(body) : undefined,
    });
  }

  async delete<T>(endpoint: string): Promise<ApiResponse<T>> {
    return this.fetch<T>(endpoint, { method: 'DELETE' });
  }

  // Paginated GET
  async getPaginated<T>(
    endpoint: string,
    params?: PaginationParams & Record<string, string>
  ): Promise<ApiResponse<PaginatedResponse<T>>> {
    const queryParams: Record<string, string> = {};

    if (params) {
      if (params.page) queryParams.page = params.page.toString();
      if (params.pageSize) queryParams.page_size = params.pageSize.toString();
      if (params.sortBy) queryParams.sort_by = params.sortBy;
      if (params.sortOrder) queryParams.sort_order = params.sortOrder;
      
      // Add any additional params
      Object.entries(params).forEach(([key, value]) => {
        if (!['page', 'pageSize', 'sortBy', 'sortOrder'].includes(key)) {
          queryParams[key] = value as string;
        }
      });
    }

    return this.get<PaginatedResponse<T>>(endpoint, queryParams);
  }

  // File upload
  async upload<T>(endpoint: string, file: File, fieldName = 'file'): Promise<ApiResponse<T>> {
    const formData = new FormData();
    formData.append(fieldName, file);

    return this.fetch<T>(endpoint, {
      method: 'POST',
      headers: {
        // Remove Content-Type to let browser set it with boundary
      },
      body: formData,
    });
  }
}

// ============================================
// API ENDPOINTS
// ============================================

export const api = new ApiClient();

// Auth endpoints
export const authApi = {
  login: (email: string, password: string) =>
    api.post<{ user: unknown; token: string }>('/auth/login', { email, password }),
  
  signup: (email: string, password: string, username: string) =>
    api.post<{ user: unknown; token: string }>('/auth/signup', { email, password, username }),
  
  logout: () => api.post('/auth/logout'),
  
  refreshToken: () => api.post<{ token: string }>('/auth/refresh'),
  
  forgotPassword: (email: string) =>
    api.post('/auth/forgot-password', { email }),
  
  resetPassword: (token: string, password: string) =>
    api.post('/auth/reset-password', { token, password }),
};

// User endpoints
export const userApi = {
  getProfile: () => api.get<unknown>('/user/profile'),
  
  updateProfile: (data: unknown) => api.patch<unknown>('/user/profile', data),
  
  updatePassword: (currentPassword: string, newPassword: string) =>
    api.post('/user/password', { currentPassword, newPassword }),
  
  uploadAvatar: (file: File) => api.upload<{ url: string }>('/user/avatar', file),
};

// Robot endpoints
export const robotApi = {
  list: (params?: PaginationParams) =>
    api.getPaginated<unknown>('/robots', params),
  
  get: (id: string) => api.get<unknown>(`/robots/${id}`),
  
  create: (data: unknown) => api.post<unknown>('/robots', data),
  
  update: (id: string, data: unknown) => api.patch<unknown>(`/robots/${id}`, data),
  
  delete: (id: string) => api.delete(`/robots/${id}`),
};

// AI endpoints
export const aiApi = {
  chat: (messages: unknown[], options?: unknown) =>
    api.post<unknown>('/ai/chat', { messages, ...options }),
  
  streamChat: async function* (messages: unknown[], options?: unknown) {
    const response = await fetch(`${API_BASE_URL}/ai/chat/stream`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ messages, ...options }),
    });

    if (!response.ok) {
      throw new Error('Failed to stream chat');
    }

    const reader = response.body?.getReader();
    if (!reader) throw new Error('No reader available');

    const decoder = new TextDecoder();

    while (true) {
      const { done, value } = await reader.read();
      if (done) break;

      const chunk = decoder.decode(value, { stream: true });
      yield chunk;
    }
  },
};

export default api;
