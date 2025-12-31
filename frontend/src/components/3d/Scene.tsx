'use client';

import * as React from 'react';
import { Canvas, type CanvasProps } from '@react-three/fiber';
import { Preload, AdaptiveDpr, AdaptiveEvents } from '@react-three/drei';

// ============================================
// SCENE WRAPPER COMPONENT
// ============================================

export interface SceneProps extends Partial<CanvasProps> {
  children: React.ReactNode;
  className?: string;
  enablePostProcessing?: boolean;
  backgroundColor?: string;
  fog?: {
    color: string;
    near: number;
    far: number;
  };
}

export function Scene({
  children,
  className,
  enablePostProcessing = false,
  backgroundColor,
  fog,
  ...canvasProps
}: SceneProps) {
  return (
    <div className={`relative h-full w-full ${className || ''}`}>
      <Canvas
        // Camera settings
        camera={{
          position: [0, 2, 5],
          fov: 50,
          near: 0.1,
          far: 1000,
        }}
        // WebGL settings
        gl={{
          antialias: true,
          alpha: true,
          powerPreference: 'high-performance',
          stencil: false,
          depth: true,
        }}
        // Performance
        dpr={[1, 2]}
        // Shadows
        shadows
        // Event settings
        eventSource={undefined}
        eventPrefix="client"
        {...canvasProps}
      >
        {/* Performance optimizations */}
        <AdaptiveDpr pixelated />
        <AdaptiveEvents />
        
        {/* Background color */}
        {backgroundColor && <color attach="background" args={[backgroundColor]} />}
        
        {/* Fog */}
        {fog && <fog attach="fog" args={[fog.color, fog.near, fog.far]} />}
        
        {/* Default lighting */}
        <SceneLighting />
        
        {/* Children (3D content) */}
        <React.Suspense fallback={<LoadingFallback />}>
          {children}
        </React.Suspense>
        
        {/* Preload assets */}
        <Preload all />
      </Canvas>
      
      {/* Loading overlay */}
      <SceneLoadingOverlay />
    </div>
  );
}

// ============================================
// DEFAULT SCENE LIGHTING
// ============================================

function SceneLighting() {
  return (
    <>
      {/* Ambient light for base illumination */}
      <ambientLight intensity={0.4} />
      
      {/* Main directional light (sun-like) */}
      <directionalLight
        position={[10, 10, 5]}
        intensity={1}
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-camera-far={50}
        shadow-camera-left={-10}
        shadow-camera-right={10}
        shadow-camera-top={10}
        shadow-camera-bottom={-10}
      />
      
      {/* Fill light from opposite side */}
      <directionalLight
        position={[-5, 5, -5]}
        intensity={0.3}
      />
      
      {/* Accent light with cyan color */}
      <pointLight
        position={[0, 5, 0]}
        color="#06b6d4"
        intensity={0.5}
      />
    </>
  );
}

// ============================================
// LOADING FALLBACK (Inside Canvas)
// ============================================

function LoadingFallback() {
  return (
    <mesh>
      <boxGeometry args={[1, 1, 1]} />
      <meshStandardMaterial color="#06b6d4" wireframe />
    </mesh>
  );
}

// ============================================
// LOADING OVERLAY (HTML overlay)
// ============================================

function SceneLoadingOverlay() {
  const [isLoading, setIsLoading] = React.useState(true);

  React.useEffect(() => {
    // Give time for Three.js to initialize
    const timer = setTimeout(() => setIsLoading(false), 500);
    return () => clearTimeout(timer);
  }, []);

  if (!isLoading) return null;

  return (
    <div className="absolute inset-0 flex items-center justify-center bg-black/80 backdrop-blur-sm">
      <div className="flex flex-col items-center gap-4">
        <div className="h-12 w-12 animate-spin rounded-full border-4 border-cyan-500 border-t-transparent" />
        <p className="text-sm text-zinc-400">Loading 3D Scene...</p>
      </div>
    </div>
  );
}

// ============================================
// SIMPLE SCENE (Pre-configured for quick use)
// ============================================

export interface SimpleSceneProps {
  children: React.ReactNode;
  className?: string;
  dark?: boolean;
}

export function SimpleScene({ children, className, dark = true }: SimpleSceneProps) {
  return (
    <Scene
      className={className}
      backgroundColor={dark ? '#09090b' : '#fafafa'}
      fog={dark ? { color: '#09090b', near: 5, far: 30 } : undefined}
    >
      {children}
    </Scene>
  );
}

export default Scene;
