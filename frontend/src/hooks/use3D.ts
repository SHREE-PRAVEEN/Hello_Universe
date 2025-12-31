'use client';

import * as React from 'react';
import { useThree, useFrame } from '@react-three/fiber';
import * as THREE from 'three';
import type { CameraConfig, SceneConfig } from '@/types/index.d';

// ============================================
// USE 3D HOOK - Main hook for 3D scene state
// ============================================

export interface Use3DReturn {
  // Scene state
  isLoaded: boolean;
  isAnimating: boolean;
  
  // Camera controls
  cameraPosition: THREE.Vector3;
  setCameraPosition: (position: [number, number, number]) => void;
  lookAt: (target: [number, number, number]) => void;
  
  // Animation controls
  play: () => void;
  pause: () => void;
  reset: () => void;
  
  // Performance
  fps: number;
}

export function use3D(): Use3DReturn {
  const [isLoaded, setIsLoaded] = React.useState(false);
  const [isAnimating, setIsAnimating] = React.useState(true);
  const [fps, setFps] = React.useState(60);
  
  const cameraPositionRef = React.useRef(new THREE.Vector3(0, 2, 5));
  const targetRef = React.useRef(new THREE.Vector3(0, 0, 0));

  // FPS tracking
  const frameCount = React.useRef(0);
  const lastTime = React.useRef(performance.now());

  const updateFPS = React.useCallback(() => {
    frameCount.current++;
    const now = performance.now();
    
    if (now - lastTime.current >= 1000) {
      setFps(frameCount.current);
      frameCount.current = 0;
      lastTime.current = now;
    }
  }, []);

  // Mark as loaded after mount
  React.useEffect(() => {
    const timer = setTimeout(() => setIsLoaded(true), 100);
    return () => clearTimeout(timer);
  }, []);

  const setCameraPosition = React.useCallback((position: [number, number, number]) => {
    cameraPositionRef.current.set(...position);
  }, []);

  const lookAt = React.useCallback((target: [number, number, number]) => {
    targetRef.current.set(...target);
  }, []);

  const play = React.useCallback(() => setIsAnimating(true), []);
  const pause = React.useCallback(() => setIsAnimating(false), []);
  const reset = React.useCallback(() => {
    cameraPositionRef.current.set(0, 2, 5);
    targetRef.current.set(0, 0, 0);
    setIsAnimating(true);
  }, []);

  return {
    isLoaded,
    isAnimating,
    cameraPosition: cameraPositionRef.current,
    setCameraPosition,
    lookAt,
    play,
    pause,
    reset,
    fps,
  };
}

// ============================================
// USE CAMERA ANIMATION HOOK
// ============================================

export interface CameraAnimationConfig {
  from: [number, number, number];
  to: [number, number, number];
  duration: number;
  easing?: (t: number) => number;
  onComplete?: () => void;
}

export function useCameraAnimation() {
  const { camera } = useThree();
  const animationRef = React.useRef<CameraAnimationConfig | null>(null);
  const progressRef = React.useRef(0);

  const animate = React.useCallback((config: CameraAnimationConfig) => {
    animationRef.current = config;
    progressRef.current = 0;
  }, []);

  useFrame((_, delta) => {
    if (!animationRef.current) return;

    const config = animationRef.current;
    const duration = config.duration / 1000; // Convert to seconds
    progressRef.current += delta / duration;

    if (progressRef.current >= 1) {
      camera.position.set(...config.to);
      config.onComplete?.();
      animationRef.current = null;
      return;
    }

    const easing = config.easing || easeInOutCubic;
    const t = easing(progressRef.current);

    camera.position.x = config.from[0] + (config.to[0] - config.from[0]) * t;
    camera.position.y = config.from[1] + (config.to[1] - config.from[1]) * t;
    camera.position.z = config.from[2] + (config.to[2] - config.from[2]) * t;
  });

  return { animate };
}

// ============================================
// USE MODEL ANIMATION HOOK
// ============================================

export interface ModelAnimationState {
  currentAnimation: string | null;
  isPlaying: boolean;
  progress: number;
}

export function useModelAnimation(mixer: THREE.AnimationMixer | null) {
  const [state, setState] = React.useState<ModelAnimationState>({
    currentAnimation: null,
    isPlaying: false,
    progress: 0,
  });

  const actionsRef = React.useRef<Map<string, THREE.AnimationAction>>(new Map());

  const playAnimation = React.useCallback((name: string, options?: {
    loop?: THREE.AnimationActionLoopStyles;
    clampWhenFinished?: boolean;
    crossFadeDuration?: number;
  }) => {
    if (!mixer) return;

    const action = actionsRef.current.get(name);
    if (!action) return;

    // Stop current animation with crossfade
    const currentAction = state.currentAnimation
      ? actionsRef.current.get(state.currentAnimation)
      : null;

    if (currentAction && options?.crossFadeDuration) {
      currentAction.fadeOut(options.crossFadeDuration);
      action.reset().fadeIn(options.crossFadeDuration).play();
    } else {
      if (currentAction) currentAction.stop();
      action.reset().play();
    }

    if (options?.loop !== undefined) {
      action.setLoop(options.loop, Infinity);
    }

    if (options?.clampWhenFinished) {
      action.clampWhenFinished = true;
    }

    setState((prev) => ({
      ...prev,
      currentAnimation: name,
      isPlaying: true,
    }));
  }, [mixer, state.currentAnimation]);

  const stopAnimation = React.useCallback(() => {
    if (!mixer) return;

    const action = state.currentAnimation
      ? actionsRef.current.get(state.currentAnimation)
      : null;

    if (action) {
      action.stop();
    }

    setState((prev) => ({
      ...prev,
      isPlaying: false,
    }));
  }, [mixer, state.currentAnimation]);

  const pauseAnimation = React.useCallback(() => {
    if (!mixer) return;

    const action = state.currentAnimation
      ? actionsRef.current.get(state.currentAnimation)
      : null;

    if (action) {
      action.paused = !action.paused;
    }

    setState((prev) => ({
      ...prev,
      isPlaying: !prev.isPlaying,
    }));
  }, [mixer, state.currentAnimation]);

  const registerAction = React.useCallback((name: string, action: THREE.AnimationAction) => {
    actionsRef.current.set(name, action);
  }, []);

  // Update mixer on each frame
  useFrame((_, delta) => {
    if (mixer && state.isPlaying) {
      mixer.update(delta);
    }
  });

  return {
    ...state,
    playAnimation,
    stopAnimation,
    pauseAnimation,
    registerAction,
  };
}

// ============================================
// USE HOVER EFFECT HOOK
// ============================================

export function useHoverEffect() {
  const [hovered, setHovered] = React.useState(false);
  const scaleRef = React.useRef(1);

  useFrame(() => {
    const targetScale = hovered ? 1.1 : 1;
    scaleRef.current = THREE.MathUtils.lerp(scaleRef.current, targetScale, 0.1);
  });

  const onPointerOver = React.useCallback(() => {
    setHovered(true);
    document.body.style.cursor = 'pointer';
  }, []);

  const onPointerOut = React.useCallback(() => {
    setHovered(false);
    document.body.style.cursor = 'default';
  }, []);

  return {
    hovered,
    scale: scaleRef.current,
    onPointerOver,
    onPointerOut,
  };
}

// ============================================
// USE SCROLL ANIMATION HOOK
// ============================================

export function useScrollAnimation(options?: {
  start?: number;
  end?: number;
  onProgress?: (progress: number) => void;
}) {
  const [scrollProgress, setScrollProgress] = React.useState(0);
  const { start = 0, end = 1, onProgress } = options || {};

  React.useEffect(() => {
    const handleScroll = () => {
      const scrollTop = window.scrollY;
      const docHeight = document.documentElement.scrollHeight - window.innerHeight;
      const progress = Math.max(0, Math.min(1, scrollTop / docHeight));
      
      // Map to start-end range
      const mappedProgress = (progress - start) / (end - start);
      const clampedProgress = Math.max(0, Math.min(1, mappedProgress));
      
      setScrollProgress(clampedProgress);
      onProgress?.(clampedProgress);
    };

    window.addEventListener('scroll', handleScroll, { passive: true });
    handleScroll(); // Initial call

    return () => window.removeEventListener('scroll', handleScroll);
  }, [start, end, onProgress]);

  return scrollProgress;
}

// ============================================
// EASING FUNCTIONS
// ============================================

export function easeInOutCubic(t: number): number {
  return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2;
}

export function easeOutExpo(t: number): number {
  return t === 1 ? 1 : 1 - Math.pow(2, -10 * t);
}

export function easeInOutQuad(t: number): number {
  return t < 0.5 ? 2 * t * t : 1 - Math.pow(-2 * t + 2, 2) / 2;
}

export function easeOutElastic(t: number): number {
  const c4 = (2 * Math.PI) / 3;
  return t === 0 ? 0 : t === 1 ? 1 : Math.pow(2, -10 * t) * Math.sin((t * 10 - 0.75) * c4) + 1;
}

export default use3D;
