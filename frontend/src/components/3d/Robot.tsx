'use client';

import * as React from 'react';
import { useGLTF, useAnimations } from '@react-three/drei';
import { useFrame } from '@react-three/fiber';
import * as THREE from 'three';
import type { GLTF } from 'three-stdlib';

// ============================================
// ROBOT MODEL COMPONENT
// ============================================

export interface RobotProps {
  modelPath?: string;
  position?: [number, number, number];
  rotation?: [number, number, number];
  scale?: number | [number, number, number];
  autoRotate?: boolean;
  autoRotateSpeed?: number;
  playAnimation?: string;
  onClick?: () => void;
  onLoad?: () => void;
}

type GLTFResult = GLTF & {
  nodes: Record<string, THREE.Mesh>;
  materials: Record<string, THREE.Material>;
};

export function Robot({
  modelPath = '/assets/models/robot.glb',
  position = [0, 0, 0],
  rotation = [0, 0, 0],
  scale = 1,
  autoRotate = false,
  autoRotateSpeed = 0.5,
  playAnimation,
  onClick,
  onLoad,
}: RobotProps) {
  const groupRef = React.useRef<THREE.Group>(null);
  
  // Load the GLTF model
  const { scene, animations } = useGLTF(modelPath) as GLTFResult;
  
  // Set up animations
  const { actions, mixer } = useAnimations(animations, groupRef);

  // Clone the scene to allow multiple instances
  const clonedScene = React.useMemo(() => scene.clone(), [scene]);

  // Play animation when specified
  React.useEffect(() => {
    if (playAnimation && actions[playAnimation]) {
      actions[playAnimation]?.reset().fadeIn(0.5).play();
    }
    
    return () => {
      if (playAnimation && actions[playAnimation]) {
        actions[playAnimation]?.fadeOut(0.5);
      }
    };
  }, [playAnimation, actions]);

  // Callback when model loads
  React.useEffect(() => {
    onLoad?.();
  }, [onLoad]);

  // Auto-rotation animation
  useFrame((state, delta) => {
    if (autoRotate && groupRef.current) {
      groupRef.current.rotation.y += delta * autoRotateSpeed;
    }
  });

  // Setup shadows for all meshes
  React.useEffect(() => {
    clonedScene.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.castShadow = true;
        child.receiveShadow = true;
      }
    });
  }, [clonedScene]);

  const scaleArray = typeof scale === 'number' ? [scale, scale, scale] : scale;

  return (
    <group
      ref={groupRef}
      position={position}
      rotation={rotation.map((r) => (r * Math.PI) / 180) as [number, number, number]}
      scale={scaleArray as [number, number, number]}
      onClick={onClick}
      onPointerOver={() => (document.body.style.cursor = onClick ? 'pointer' : 'default')}
      onPointerOut={() => (document.body.style.cursor = 'default')}
    >
      <primitive object={clonedScene} />
    </group>
  );
}

// ============================================
// PLACEHOLDER ROBOT (When no model available)
// ============================================

export interface PlaceholderRobotProps {
  position?: [number, number, number];
  color?: string;
  wireframe?: boolean;
  animate?: boolean;
}

export function PlaceholderRobot({
  position = [0, 0, 0],
  color = '#06b6d4',
  wireframe = false,
  animate = true,
}: PlaceholderRobotProps) {
  const groupRef = React.useRef<THREE.Group>(null);
  const headRef = React.useRef<THREE.Mesh>(null);

  useFrame((state) => {
    if (animate && groupRef.current) {
      // Subtle floating animation
      groupRef.current.position.y = position[1] + Math.sin(state.clock.elapsedTime) * 0.1;
      
      // Head rotation
      if (headRef.current) {
        headRef.current.rotation.y = Math.sin(state.clock.elapsedTime * 0.5) * 0.3;
      }
    }
  });

  return (
    <group ref={groupRef} position={position}>
      {/* Body */}
      <mesh position={[0, 0.5, 0]} castShadow>
        <boxGeometry args={[0.8, 1, 0.5]} />
        <meshStandardMaterial color={color} wireframe={wireframe} metalness={0.8} roughness={0.2} />
      </mesh>

      {/* Head */}
      <mesh ref={headRef} position={[0, 1.3, 0]} castShadow>
        <boxGeometry args={[0.5, 0.5, 0.5]} />
        <meshStandardMaterial color={color} wireframe={wireframe} metalness={0.8} roughness={0.2} />
      </mesh>

      {/* Eyes */}
      <mesh position={[-0.12, 1.35, 0.26]}>
        <sphereGeometry args={[0.06, 16, 16]} />
        <meshStandardMaterial color="#fff" emissive="#06b6d4" emissiveIntensity={2} />
      </mesh>
      <mesh position={[0.12, 1.35, 0.26]}>
        <sphereGeometry args={[0.06, 16, 16]} />
        <meshStandardMaterial color="#fff" emissive="#06b6d4" emissiveIntensity={2} />
      </mesh>

      {/* Antenna */}
      <mesh position={[0, 1.7, 0]}>
        <cylinderGeometry args={[0.02, 0.02, 0.3, 8]} />
        <meshStandardMaterial color="#666" metalness={0.9} roughness={0.1} />
      </mesh>
      <mesh position={[0, 1.9, 0]}>
        <sphereGeometry args={[0.05, 16, 16]} />
        <meshStandardMaterial color="#ef4444" emissive="#ef4444" emissiveIntensity={1} />
      </mesh>

      {/* Arms */}
      <mesh position={[-0.55, 0.5, 0]} castShadow>
        <boxGeometry args={[0.15, 0.6, 0.15]} />
        <meshStandardMaterial color={color} wireframe={wireframe} metalness={0.8} roughness={0.2} />
      </mesh>
      <mesh position={[0.55, 0.5, 0]} castShadow>
        <boxGeometry args={[0.15, 0.6, 0.15]} />
        <meshStandardMaterial color={color} wireframe={wireframe} metalness={0.8} roughness={0.2} />
      </mesh>

      {/* Legs */}
      <mesh position={[-0.2, -0.3, 0]} castShadow>
        <boxGeometry args={[0.2, 0.6, 0.2]} />
        <meshStandardMaterial color={color} wireframe={wireframe} metalness={0.8} roughness={0.2} />
      </mesh>
      <mesh position={[0.2, -0.3, 0]} castShadow>
        <boxGeometry args={[0.2, 0.6, 0.2]} />
        <meshStandardMaterial color={color} wireframe={wireframe} metalness={0.8} roughness={0.2} />
      </mesh>
    </group>
  );
}

// Preload models for better performance
// useGLTF.preload('/assets/models/robot.glb');

export default Robot;
