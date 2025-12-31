'use client';

import * as React from 'react';
import { cn } from '@/lib/utils';

// ============================================
// INPUT COMPONENT
// ============================================

export interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
  hint?: string;
  leftIcon?: React.ReactNode;
  rightIcon?: React.ReactNode;
  rightElement?: React.ReactNode;
}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  (
    {
      className,
      type,
      label,
      error,
      hint,
      leftIcon,
      rightIcon,
      rightElement,
      disabled,
      id,
      ...props
    },
    ref
  ) => {
    const inputId = id || React.useId();

    return (
      <div className="w-full">
        {label && (
          <label
            htmlFor={inputId}
            className="mb-2 block text-sm font-medium text-zinc-700 dark:text-zinc-300"
          >
            {label}
          </label>
        )}
        <div className="relative">
          {leftIcon && (
            <div className="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-zinc-400">
              {leftIcon}
            </div>
          )}
          <input
            type={type}
            id={inputId}
            className={cn(
              // Base styles
              'flex h-11 w-full rounded-xl border bg-white px-4 py-2 text-sm transition-all duration-200',
              // Default border
              'border-zinc-200 dark:border-zinc-700',
              // Background
              'dark:bg-zinc-900',
              // Text color
              'text-zinc-900 dark:text-zinc-50',
              // Placeholder
              'placeholder:text-zinc-400 dark:placeholder:text-zinc-500',
              // Focus styles
              'focus:border-cyan-500 focus:outline-none focus:ring-2 focus:ring-cyan-500/20',
              // Error styles
              error && 'border-red-500 focus:border-red-500 focus:ring-red-500/20',
              // Disabled styles
              'disabled:cursor-not-allowed disabled:opacity-50 disabled:bg-zinc-100 dark:disabled:bg-zinc-800',
              // Icon padding
              leftIcon && 'pl-10',
              (rightIcon || rightElement) && 'pr-10',
              className
            )}
            ref={ref}
            disabled={disabled}
            aria-invalid={!!error}
            aria-describedby={error ? `${inputId}-error` : hint ? `${inputId}-hint` : undefined}
            {...props}
          />
          {rightIcon && (
            <div className="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-zinc-400">
              {rightIcon}
            </div>
          )}
          {rightElement && (
            <div className="absolute right-2 top-1/2 -translate-y-1/2">
              {rightElement}
            </div>
          )}
        </div>
        {error && (
          <p id={`${inputId}-error`} className="mt-1.5 text-sm text-red-500">
            {error}
          </p>
        )}
        {hint && !error && (
          <p id={`${inputId}-hint`} className="mt-1.5 text-sm text-zinc-500 dark:text-zinc-400">
            {hint}
          </p>
        )}
      </div>
    );
  }
);

Input.displayName = 'Input';

// ============================================
// PASSWORD INPUT
// ============================================

const PasswordInput = React.forwardRef<HTMLInputElement, Omit<InputProps, 'type' | 'rightElement'>>(
  (props, ref) => {
    const [showPassword, setShowPassword] = React.useState(false);

    return (
      <Input
        {...props}
        ref={ref}
        type={showPassword ? 'text' : 'password'}
        rightElement={
          <button
            type="button"
            onClick={() => setShowPassword(!showPassword)}
            className="flex h-8 w-8 items-center justify-center rounded-lg text-zinc-400 hover:bg-zinc-100 hover:text-zinc-600 dark:hover:bg-zinc-800 dark:hover:text-zinc-300"
            aria-label={showPassword ? 'Hide password' : 'Show password'}
          >
            {showPassword ? <EyeOffIcon /> : <EyeIcon />}
          </button>
        }
      />
    );
  }
);

PasswordInput.displayName = 'PasswordInput';

// ============================================
// TEXTAREA COMPONENT
// ============================================

export interface TextareaProps extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {
  label?: string;
  error?: string;
  hint?: string;
}

const Textarea = React.forwardRef<HTMLTextAreaElement, TextareaProps>(
  ({ className, label, error, hint, disabled, id, ...props }, ref) => {
    const textareaId = id || React.useId();

    return (
      <div className="w-full">
        {label && (
          <label
            htmlFor={textareaId}
            className="mb-2 block text-sm font-medium text-zinc-700 dark:text-zinc-300"
          >
            {label}
          </label>
        )}
        <textarea
          id={textareaId}
          className={cn(
            // Base styles
            'flex min-h-[120px] w-full rounded-xl border bg-white px-4 py-3 text-sm transition-all duration-200',
            // Default border
            'border-zinc-200 dark:border-zinc-700',
            // Background
            'dark:bg-zinc-900',
            // Text color
            'text-zinc-900 dark:text-zinc-50',
            // Placeholder
            'placeholder:text-zinc-400 dark:placeholder:text-zinc-500',
            // Focus styles
            'focus:border-cyan-500 focus:outline-none focus:ring-2 focus:ring-cyan-500/20',
            // Error styles
            error && 'border-red-500 focus:border-red-500 focus:ring-red-500/20',
            // Disabled styles
            'disabled:cursor-not-allowed disabled:opacity-50 disabled:bg-zinc-100 dark:disabled:bg-zinc-800',
            // Resize
            'resize-none',
            className
          )}
          ref={ref}
          disabled={disabled}
          aria-invalid={!!error}
          aria-describedby={error ? `${textareaId}-error` : hint ? `${textareaId}-hint` : undefined}
          {...props}
        />
        {error && (
          <p id={`${textareaId}-error`} className="mt-1.5 text-sm text-red-500">
            {error}
          </p>
        )}
        {hint && !error && (
          <p id={`${textareaId}-hint`} className="mt-1.5 text-sm text-zinc-500 dark:text-zinc-400">
            {hint}
          </p>
        )}
      </div>
    );
  }
);

Textarea.displayName = 'Textarea';

// ============================================
// CHECKBOX COMPONENT
// ============================================

export interface CheckboxProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'type'> {
  label?: string;
  error?: string;
}

const Checkbox = React.forwardRef<HTMLInputElement, CheckboxProps>(
  ({ className, label, error, id, ...props }, ref) => {
    const checkboxId = id || React.useId();

    return (
      <div className="flex items-start gap-3">
        <input
          type="checkbox"
          id={checkboxId}
          ref={ref}
          className={cn(
            'h-5 w-5 rounded border-zinc-300 text-cyan-500 transition-colors',
            'focus:ring-2 focus:ring-cyan-500/20 focus:ring-offset-0',
            'dark:border-zinc-600 dark:bg-zinc-800',
            error && 'border-red-500',
            className
          )}
          {...props}
        />
        {label && (
          <label
            htmlFor={checkboxId}
            className={cn(
              'text-sm text-zinc-700 dark:text-zinc-300',
              error && 'text-red-500'
            )}
          >
            {label}
          </label>
        )}
      </div>
    );
  }
);

Checkbox.displayName = 'Checkbox';

// ============================================
// ICONS
// ============================================

function EyeIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="18"
      height="18"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" />
      <circle cx="12" cy="12" r="3" />
    </svg>
  );
}

function EyeOffIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="18"
      height="18"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M9.88 9.88a3 3 0 1 0 4.24 4.24" />
      <path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68" />
      <path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61" />
      <line x1="2" x2="22" y1="2" y2="22" />
    </svg>
  );
}

export { Input, PasswordInput, Textarea, Checkbox };
