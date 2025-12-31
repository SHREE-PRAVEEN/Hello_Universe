'use client';

import * as React from 'react';
import type { AIMessage, AIStreamChunk } from '@/types/index.d';
import { generateId } from '@/lib/utils';

// ============================================
// USE AI HOOK
// ============================================

export interface UseAIOptions {
  endpoint?: string;
  model?: string;
  systemPrompt?: string;
  maxTokens?: number;
  temperature?: number;
}

export interface UseAIReturn {
  // State
  messages: AIMessage[];
  isLoading: boolean;
  isStreaming: boolean;
  error: string | null;
  
  // Actions
  sendMessage: (content: string) => Promise<void>;
  clearMessages: () => void;
  stopStreaming: () => void;
  regenerateLastResponse: () => Promise<void>;
  
  // Current response (during streaming)
  currentResponse: string;
}

const defaultOptions: UseAIOptions = {
  endpoint: '/api/ai/chat',
  model: 'gpt-4',
  systemPrompt: 'You are Hello Universe, an AI assistant specialized in robotics and blockchain technology.',
  maxTokens: 2048,
  temperature: 0.7,
};

export function useAI(options: UseAIOptions = {}): UseAIReturn {
  const config = { ...defaultOptions, ...options };
  
  const [messages, setMessages] = React.useState<AIMessage[]>([]);
  const [isLoading, setIsLoading] = React.useState(false);
  const [isStreaming, setIsStreaming] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);
  const [currentResponse, setCurrentResponse] = React.useState('');
  
  const abortControllerRef = React.useRef<AbortController | null>(null);

  // Send a message and get AI response
  const sendMessage = React.useCallback(async (content: string) => {
    if (!content.trim()) return;
    
    setError(null);
    setIsLoading(true);
    
    // Add user message
    const userMessage: AIMessage = {
      id: generateId(),
      role: 'user',
      content: content.trim(),
      timestamp: new Date(),
    };
    
    setMessages((prev) => [...prev, userMessage]);
    
    // Create abort controller for cancellation
    abortControllerRef.current = new AbortController();
    
    try {
      const response = await fetch(config.endpoint!, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          messages: [...messages, userMessage].map((m) => ({
            role: m.role,
            content: m.content,
          })),
          model: config.model,
          systemPrompt: config.systemPrompt,
          maxTokens: config.maxTokens,
          temperature: config.temperature,
          stream: true,
        }),
        signal: abortControllerRef.current.signal,
      });
      
      if (!response.ok) {
        throw new Error(`API error: ${response.statusText}`);
      }
      
      // Handle streaming response
      if (response.body) {
        setIsStreaming(true);
        setCurrentResponse('');
        
        const reader = response.body.getReader();
        const decoder = new TextDecoder();
        let fullResponse = '';
        
        while (true) {
          const { done, value } = await reader.read();
          
          if (done) break;
          
          const chunk = decoder.decode(value, { stream: true });
          const lines = chunk.split('\n').filter((line) => line.trim() !== '');
          
          for (const line of lines) {
            if (line.startsWith('data: ')) {
              const data = line.slice(6);
              
              if (data === '[DONE]') continue;
              
              try {
                const parsed: AIStreamChunk = JSON.parse(data);
                
                if (parsed.content) {
                  fullResponse += parsed.content;
                  setCurrentResponse(fullResponse);
                }
                
                if (parsed.error) {
                  throw new Error(parsed.error);
                }
              } catch (e) {
                // Non-JSON data, treat as plain text
                fullResponse += data;
                setCurrentResponse(fullResponse);
              }
            }
          }
        }
        
        // Add assistant message when streaming is complete
        const assistantMessage: AIMessage = {
          id: generateId(),
          role: 'assistant',
          content: fullResponse,
          timestamp: new Date(),
          metadata: {
            model: config.model!,
            tokens: 0, // Would need to track this from the API
            latency: 0,
          },
        };
        
        setMessages((prev) => [...prev, assistantMessage]);
        setCurrentResponse('');
      }
    } catch (err) {
      if (err instanceof Error && err.name === 'AbortError') {
        // User cancelled the request
        setError('Request cancelled');
      } else {
        setError(err instanceof Error ? err.message : 'Failed to get AI response');
      }
    } finally {
      setIsLoading(false);
      setIsStreaming(false);
      abortControllerRef.current = null;
    }
  }, [messages, config]);

  // Clear all messages
  const clearMessages = React.useCallback(() => {
    setMessages([]);
    setError(null);
    setCurrentResponse('');
  }, []);

  // Stop streaming
  const stopStreaming = React.useCallback(() => {
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
      abortControllerRef.current = null;
    }
  }, []);

  // Regenerate last response
  const regenerateLastResponse = React.useCallback(async () => {
    if (messages.length < 2) return;
    
    // Remove the last assistant message
    const lastUserMessageIndex = messages.findLastIndex((m) => m.role === 'user');
    
    if (lastUserMessageIndex === -1) return;
    
    const lastUserMessage = messages[lastUserMessageIndex];
    
    // Remove messages after the last user message
    setMessages((prev) => prev.slice(0, lastUserMessageIndex));
    
    // Resend the message
    await sendMessage(lastUserMessage.content);
  }, [messages, sendMessage]);

  return {
    messages,
    isLoading,
    isStreaming,
    error,
    sendMessage,
    clearMessages,
    stopStreaming,
    regenerateLastResponse,
    currentResponse,
  };
}

// ============================================
// USE CHAT SCROLL HOOK
// ============================================

export function useChatScroll(messages: AIMessage[]) {
  const containerRef = React.useRef<HTMLDivElement>(null);

  React.useEffect(() => {
    if (containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [messages]);

  return containerRef;
}

// ============================================
// USE SPEECH TO TEXT HOOK
// ============================================

export function useSpeechToText() {
  const [isListening, setIsListening] = React.useState(false);
  const [transcript, setTranscript] = React.useState('');
  const recognitionRef = React.useRef<SpeechRecognition | null>(null);

  React.useEffect(() => {
    if (typeof window !== 'undefined') {
      const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
      
      if (SpeechRecognition) {
        recognitionRef.current = new SpeechRecognition();
        recognitionRef.current.continuous = true;
        recognitionRef.current.interimResults = true;
        
        recognitionRef.current.onresult = (event) => {
          const current = event.resultIndex;
          const result = event.results[current];
          setTranscript(result[0].transcript);
        };
        
        recognitionRef.current.onerror = () => {
          setIsListening(false);
        };
        
        recognitionRef.current.onend = () => {
          setIsListening(false);
        };
      }
    }
    
    return () => {
      if (recognitionRef.current) {
        recognitionRef.current.stop();
      }
    };
  }, []);

  const startListening = React.useCallback(() => {
    if (recognitionRef.current) {
      setTranscript('');
      recognitionRef.current.start();
      setIsListening(true);
    }
  }, []);

  const stopListening = React.useCallback(() => {
    if (recognitionRef.current) {
      recognitionRef.current.stop();
      setIsListening(false);
    }
  }, []);

  return {
    isListening,
    transcript,
    startListening,
    stopListening,
    isSupported: !!recognitionRef.current,
  };
}

export default useAI;
