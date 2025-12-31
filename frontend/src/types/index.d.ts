// Hello Universe - Global Type Definitions

// ============================================
// USER TYPES
// ============================================

export interface User {
  id: string;
  email: string;
  username: string;
  walletAddress?: string;
  avatar?: string;
  createdAt: Date;
  updatedAt: Date;
  preferences: UserPreferences;
}

export interface UserPreferences {
  theme: 'light' | 'dark' | 'system';
  notifications: boolean;
  language: string;
  newsletter: boolean;
}

export interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
}

// ============================================
// API TYPES
// ============================================

export interface ApiResponse<T> {
  success: boolean;
  data: T;
  message?: string;
  error?: ApiError;
}

export interface ApiError {
  code: string;
  message: string;
  details?: Record<string, string[]>;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
  totalPages: number;
  hasNext: boolean;
  hasPrevious: boolean;
}

export interface PaginationParams {
  page?: number;
  pageSize?: number;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
}

// ============================================
// AI TYPES
// ============================================

export interface AIMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: Date;
  metadata?: AIMessageMetadata;
}

export interface AIMessageMetadata {
  model: string;
  tokens: number;
  latency: number;
  sources?: string[];
}

export interface AIConversation {
  id: string;
  title: string;
  messages: AIMessage[];
  createdAt: Date;
  updatedAt: Date;
  userId: string;
}

export interface AIStreamChunk {
  content: string;
  done: boolean;
  error?: string;
}

// ============================================
// ROBOT TYPES
// ============================================

export interface Robot {
  id: string;
  name: string;
  type: RobotType;
  status: RobotStatus;
  model3DUrl: string;
  thumbnailUrl: string;
  specifications: RobotSpecifications;
  capabilities: string[];
  createdAt: Date;
}

export type RobotType = 'humanoid' | 'quadruped' | 'wheeled' | 'drone' | 'industrial';

export type RobotStatus = 'idle' | 'active' | 'maintenance' | 'offline';

export interface RobotSpecifications {
  height: number;
  weight: number;
  batteryCapacity: number;
  maxSpeed: number;
  payload: number;
  sensors: string[];
}

// ============================================
// 3D SCENE TYPES
// ============================================

export interface SceneConfig {
  camera: CameraConfig;
  lighting: LightingConfig;
  environment: EnvironmentConfig;
  postProcessing: PostProcessingConfig;
}

export interface CameraConfig {
  position: [number, number, number];
  target: [number, number, number];
  fov: number;
  near: number;
  far: number;
}

export interface LightingConfig {
  ambient: { color: string; intensity: number };
  directional: { color: string; intensity: number; position: [number, number, number] };
  spotlights?: SpotlightConfig[];
}

export interface SpotlightConfig {
  color: string;
  intensity: number;
  position: [number, number, number];
  angle: number;
  penumbra: number;
}

export interface EnvironmentConfig {
  background: string | null;
  hdri?: string;
  fog?: { color: string; near: number; far: number };
}

export interface PostProcessingConfig {
  bloom: boolean;
  bloomIntensity?: number;
  chromaticAberration: boolean;
  vignette: boolean;
}

// ============================================
// FORM TYPES
// ============================================

export interface FormField {
  name: string;
  label: string;
  type: 'text' | 'email' | 'password' | 'number' | 'select' | 'textarea' | 'checkbox';
  placeholder?: string;
  required?: boolean;
  options?: SelectOption[];
  validation?: ValidationRule[];
}

export interface SelectOption {
  value: string;
  label: string;
  disabled?: boolean;
}

export interface ValidationRule {
  type: 'required' | 'min' | 'max' | 'pattern' | 'custom';
  value?: string | number | RegExp;
  message: string;
}

// ============================================
// NOTIFICATION TYPES
// ============================================

export type NotificationType = 'success' | 'error' | 'warning' | 'info';

export interface Notification {
  id: string;
  type: NotificationType;
  title: string;
  message: string;
  duration?: number;
  action?: NotificationAction;
}

export interface NotificationAction {
  label: string;
  onClick: () => void;
}

// ============================================
// UTILITY TYPES
// ============================================

export type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

export type Nullable<T> = T | null;

export type AsyncState<T> = {
  data: T | null;
  isLoading: boolean;
  error: Error | null;
};
