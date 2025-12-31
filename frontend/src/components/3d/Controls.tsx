'use client';

import * as React from 'react';
import { OrbitControls, PerspectiveCamera, Environment } from '@react-three/drei';
import { useThree, useFrame } from '@react-three/fiber';
import * as THREE from 'three';

// ============================================
// CAMERA CONTROLS COMPONENT
// ============================================

export interface ControlsProps {
  enableOrbit?: boolean;
  enablePan?: boolean;
  enableZoom?: boolean;
  autoRotate?: boolean;
  autoRotateSpeed?: number;
  minDistance?: number;
  maxDistance?: number;
  minPolarAngle?: number;
  maxPolarAngle?: number;
  target?: [number, number, number];
  dampingFactor?: number;
}

export function Controls({
  enableOrbit = true,
  enablePan = true,
  enableZoom = true,
  autoRotate = false,
  autoRotateSpeed = 0.5,
  minDistance = 2,
  maxDistance = 20,
  minPolarAngle = 0,
  maxPolarAngle = Math.PI / 2,
  target = [0, 0, 0],
  dampingFactor = 0.05,
}: ControlsProps) {
  const controlsRef = React.useRef<any>(null);

  return (
    <OrbitControls
      ref={controlsRef}
      makeDefault
      enableRotate={enableOrbit}
      enablePan={enablePan}
      enableZoom={enableZoom}
      autoRotate={autoRotate}
      autoRotateSpeed={autoRotateSpeed}
      minDistance={minDistance}
      maxDistance={maxDistance}
      minPolarAngle={minPolarAngle}
      maxPolarAngle={maxPolarAngle}
      target={target}
      enableDamping
      dampingFactor={dampingFactor}
    />
  );
}

// ============================================
// CUSTOM CAMERA
// ============================================

export interface CameraProps {
  position?: [number, number, number];
  fov?: number;
  near?: number;
  far?: number;
  lookAt?: [number, number, number];
  makeDefault?: boolean;
}

export function Camera({
  position = [0, 2, 5],
  fov = 50,
  near = 0.1,
  far = 1000,
  lookAt = [0, 0, 0],
  makeDefault = true,
}: CameraProps) {
  const cameraRef = React.useRef<THREE.PerspectiveCamera>(null);

  React.useEffect(() => {
    if (cameraRef.current) {
      cameraRef.current.lookAt(...lookAt);
    }
  }, [lookAt]);

  return (
    <PerspectiveCamera
      ref={cameraRef}
      makeDefault={makeDefault}
      position={position}
      fov={fov}
      near={near}
      far={far}
    />
  );
}

// ============================================
// CAMERA ANIMATION CONTROLLER
// ============================================

export interface CameraAnimationProps {
  positions: [number, number, number][];
  duration?: number;
  loop?: boolean;
  onComplete?: () => void;
}

export function CameraAnimation({
  positions,
  duration = 2000,
  loop = false,
  onComplete,
}: CameraAnimationProps) {
  const { camera } = useThree();
  const startTime = React.useRef<number | null>(null);
  const currentIndex = React.useRef(0);

  useFrame((state) => {
    if (positions.length < 2) return;

    if (startTime.current === null) {
      startTime.current = state.clock.elapsedTime * 1000;
    }

    const elapsed = state.clock.elapsedTime * 1000 - startTime.current;
    const progress = Math.min(elapsed / duration, 1);

    const fromIndex = currentIndex.current;
    const toIndex = (currentIndex.current + 1) % positions.length;

    const from = positions[fromIndex];
    const to = positions[toIndex];

    // Smooth easing
    const eased = easeInOutCubic(progress);

    camera.position.x = from[0] + (to[0] - from[0]) * eased;
    camera.position.y = from[1] + (to[1] - from[1]) * eased;
    camera.position.z = from[2] + (to[2] - from[2]) * eased;

    if (progress >= 1) {
      startTime.current = null;
      currentIndex.current = toIndex;

      if (!loop && currentIndex.current === positions.length - 1) {
        onComplete?.();
      }
    }
  });

  return null;
}

function easeInOutCubic(t: number): number {
  return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2;
}

// ============================================
// SCENE ENVIRONMENT
// ============================================

export interface SceneEnvironmentProps {
  preset?: 'sunset' | 'dawn' | 'night' | 'warehouse' | 'forest' | 'apartment' | 'studio' | 'city' | 'park' | 'lobby';
  background?: boolean;
  blur?: number;
}

export function SceneEnvironment({
  preset = 'city',
  background = false,
  blur = 0,
}: SceneEnvironmentProps) {
  return (
    <Environment
      preset={preset}
      background={background}
      blur={blur}
    />
  );
}

// ============================================
// FLOOR / GROUND
// ============================================

export interface FloorProps {
  size?: number;
  color?: string;
  receiveShadow?: boolean;
  grid?: boolean;
  gridColor?: string;
}

export function Floor({
  size = 20,
  color = '#111',
  receiveShadow = true,
  grid = false,
  gridColor = '#333',
}: FloorProps) {
  return (
    <group>
      <mesh
        rotation={[-Math.PI / 2, 0, 0]}
        position={[0, 0, 0]}
        receiveShadow={receiveShadow}
      >
        <planeGeometry args={[size, size]} />
        <meshStandardMaterial color={color} />
      </mesh>
      
      {grid && (
        <gridHelper
          args={[size, size, gridColor, gridColor]}
          position={[0, 0.01, 0]}
        />
      )}
    </group>
  );
}

// ============================================
// SCREEN SHAKE EFFECT
// ============================================

export interface ScreenShakeProps {
  intensity?: number;
  decay?: boolean;
  decayRate?: number;
}

export function useScreenShake({ intensity = 0.1, decay = true, decayRate = 0.95 }: ScreenShakeProps = {}) {
  const { camera } = useThree();
  const originalPosition = React.useRef(new THREE.Vector3());
  const shakeIntensity = React.useRef(0);

  const shake = React.useCallback((power = 1) => {
    shakeIntensity.current = intensity * power;
    originalPosition.current.copy(camera.position);
  }, [camera, intensity]);

  useFrame(() => {
    if (shakeIntensity.current > 0.001) {
      camera.position.x = originalPosition.current.x + (Math.random() - 0.5) * shakeIntensity.current;
      camera.position.y = originalPosition.current.y + (Math.random() - 0.5) * shakeIntensity.current;
      camera.position.z = originalPosition.current.z + (Math.random() - 0.5) * shakeIntensity.current;

      if (decay) {
        shakeIntensity.current *= decayRate;
      }
    }
  });

  return shake;
}

export default Controls;
