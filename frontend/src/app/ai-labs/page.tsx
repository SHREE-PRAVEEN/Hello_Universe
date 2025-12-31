'use client';

import * as React from 'react';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/input';
import { useAI } from '@/hooks/useAI';

export default function AILabsPage() {
  const [prompt, setPrompt] = React.useState('');
  const [selectedModel, setSelectedModel] = React.useState('gpt-4');
  const { messages, isLoading, sendMessage, clearConversation } = useAI();

  const models = [
    { id: 'gpt-4', name: 'GPT-4', description: 'Most capable model' },
    { id: 'gpt-3.5-turbo', name: 'GPT-3.5', description: 'Fast and efficient' },
    { id: 'claude-3', name: 'Claude 3', description: 'Balanced reasoning' },
    { id: 'gemini-pro', name: 'Gemini Pro', description: 'Google AI' },
  ];

  const presets = [
    { name: 'Robot Control', prompt: 'Help me write a control algorithm for a robotic arm with 6 degrees of freedom.' },
    { name: 'Path Planning', prompt: 'Design an optimal path planning algorithm for autonomous navigation.' },
    { name: 'Computer Vision', prompt: 'Create an object detection pipeline for a robot to identify and grasp items.' },
    { name: 'NLP Interface', prompt: 'Build a natural language interface for controlling robots with voice commands.' },
  ];

  const handleSend = async () => {
    if (!prompt.trim() || isLoading) return;
    const currentPrompt = prompt;
    setPrompt('');
    await sendMessage(currentPrompt);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <div className="min-h-screen bg-zinc-950 pt-20">
      <div className="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-white">AI Labs</h1>
          <p className="mt-1 text-zinc-400">
            Experiment with AI models to build intelligent robot behaviors.
          </p>
        </div>

        <div className="grid gap-8 lg:grid-cols-4">
          {/* Sidebar */}
          <div className="space-y-6">
            {/* Model Selector */}
            <Card variant="elevated">
              <CardHeader>
                <CardTitle className="text-white">Model</CardTitle>
              </CardHeader>
              <CardContent className="space-y-2">
                {models.map((model) => (
                  <button
                    key={model.id}
                    onClick={() => setSelectedModel(model.id)}
                    className={`w-full rounded-lg border p-3 text-left transition-all ${
                      selectedModel === model.id
                        ? 'border-cyan-500 bg-cyan-500/10'
                        : 'border-zinc-800 hover:border-zinc-700'
                    }`}
                  >
                    <p
                      className={`font-medium ${
                        selectedModel === model.id ? 'text-cyan-400' : 'text-white'
                      }`}
                    >
                      {model.name}
                    </p>
                    <p className="text-xs text-zinc-500">{model.description}</p>
                  </button>
                ))}
              </CardContent>
            </Card>

            {/* Quick Presets */}
            <Card variant="elevated">
              <CardHeader>
                <CardTitle className="text-white">Quick Presets</CardTitle>
              </CardHeader>
              <CardContent className="space-y-2">
                {presets.map((preset) => (
                  <Button
                    key={preset.name}
                    variant="ghost"
                    size="sm"
                    className="w-full justify-start text-left"
                    onClick={() => setPrompt(preset.prompt)}
                  >
                    <SparkleIcon className="mr-2 h-4 w-4 text-cyan-400" />
                    {preset.name}
                  </Button>
                ))}
              </CardContent>
            </Card>
          </div>

          {/* Chat Area */}
          <div className="lg:col-span-3">
            <Card variant="elevated" className="flex h-[calc(100vh-220px)] flex-col">
              <CardHeader className="flex flex-row items-center justify-between border-b border-zinc-800">
                <CardTitle className="text-white">
                  Chat with {models.find((m) => m.id === selectedModel)?.name}
                </CardTitle>
                <Button variant="ghost" size="sm" onClick={clearConversation}>
                  Clear Chat
                </Button>
              </CardHeader>

              {/* Messages */}
              <div className="flex-1 overflow-y-auto p-6">
                {messages.length === 0 ? (
                  <div className="flex h-full flex-col items-center justify-center text-center">
                    <div className="mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gradient-to-br from-cyan-500 to-blue-600">
                      <BrainIcon className="h-8 w-8 text-white" />
                    </div>
                    <h3 className="text-lg font-medium text-white">
                      Start a conversation
                    </h3>
                    <p className="mt-1 max-w-sm text-sm text-zinc-400">
                      Ask the AI to help you build robot behaviors, control algorithms,
                      or analyze sensor data.
                    </p>
                  </div>
                ) : (
                  <div className="space-y-6">
                    {messages.map((message) => (
                      <div
                        key={message.id}
                        className={`flex ${
                          message.role === 'user' ? 'justify-end' : 'justify-start'
                        }`}
                      >
                        <div
                          className={`max-w-[80%] rounded-2xl px-4 py-3 ${
                            message.role === 'user'
                              ? 'bg-cyan-500 text-white'
                              : 'bg-zinc-800 text-zinc-100'
                          }`}
                        >
                          <p className="whitespace-pre-wrap">{message.content}</p>
                          <p
                            className={`mt-1 text-xs ${
                              message.role === 'user'
                                ? 'text-cyan-200'
                                : 'text-zinc-500'
                            }`}
                          >
                            {new Date(message.timestamp).toLocaleTimeString()}
                          </p>
                        </div>
                      </div>
                    ))}
                    {isLoading && (
                      <div className="flex justify-start">
                        <div className="max-w-[80%] rounded-2xl bg-zinc-800 px-4 py-3">
                          <div className="flex items-center gap-2">
                            <div className="h-2 w-2 animate-bounce rounded-full bg-cyan-400" />
                            <div
                              className="h-2 w-2 animate-bounce rounded-full bg-cyan-400"
                              style={{ animationDelay: '0.1s' }}
                            />
                            <div
                              className="h-2 w-2 animate-bounce rounded-full bg-cyan-400"
                              style={{ animationDelay: '0.2s' }}
                            />
                          </div>
                        </div>
                      </div>
                    )}
                  </div>
                )}
              </div>

              {/* Input */}
              <div className="border-t border-zinc-800 p-4">
                <div className="flex gap-4">
                  <Textarea
                    value={prompt}
                    onChange={(e) => setPrompt(e.target.value)}
                    onKeyDown={handleKeyDown}
                    placeholder="Type your message..."
                    className="min-h-[60px] resize-none"
                  />
                  <Button
                    variant="primary"
                    onClick={handleSend}
                    disabled={!prompt.trim() || isLoading}
                    className="shrink-0"
                  >
                    <SendIcon className="h-4 w-4" />
                  </Button>
                </div>
                <p className="mt-2 text-xs text-zinc-500">
                  Press Enter to send, Shift + Enter for new line
                </p>
              </div>
            </Card>
          </div>
        </div>

        {/* Capabilities Section */}
        <div className="mt-12">
          <h2 className="mb-6 text-2xl font-bold text-white">AI Capabilities</h2>
          <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
            {[
              {
                icon: <RobotIcon className="h-6 w-6" />,
                title: 'Robot Control',
                description: 'Generate control code for various robot types and configurations.',
              },
              {
                icon: <EyeIcon className="h-6 w-6" />,
                title: 'Computer Vision',
                description: 'Build vision pipelines for object detection and scene understanding.',
              },
              {
                icon: <RouteIcon className="h-6 w-6" />,
                title: 'Path Planning',
                description: 'Create intelligent navigation algorithms for autonomous movement.',
              },
              {
                icon: <ChatIcon className="h-6 w-6" />,
                title: 'NLP Interface',
                description: 'Design natural language interfaces for intuitive robot control.',
              },
            ].map((capability, index) => (
              <Card key={index} variant="elevated" className="transition-all hover:border-cyan-500/50">
                <CardContent className="p-6">
                  <div className="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-gradient-to-br from-cyan-500 to-blue-600 text-white">
                    {capability.icon}
                  </div>
                  <h3 className="font-medium text-white">{capability.title}</h3>
                  <p className="mt-1 text-sm text-zinc-400">{capability.description}</p>
                </CardContent>
              </Card>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

// Icons
function BrainIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
    </svg>
  );
}

function SparkleIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
    </svg>
  );
}

function SendIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
    </svg>
  );
}

function RobotIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
    </svg>
  );
}

function EyeIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
    </svg>
  );
}

function RouteIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 20l-5.447-2.724A1 1 0 013 16.382V5.618a1 1 0 011.447-.894L9 7m0 13l6-3m-6 3V7m6 10l4.553 2.276A1 1 0 0021 18.382V7.618a1 1 0 00-.553-.894L15 4m0 13V4m0 0L9 7" />
    </svg>
  );
}

function ChatIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
    </svg>
  );
}
