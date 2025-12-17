import { useRef, useEffect, useState, useCallback } from 'react';
import * as THREE from 'three';
// FIX: Use 'examples/jsm' path which is more stable across different build systems/bundlers
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

// Define materials globally to avoid recreating them on every render
const MATERIALS = {
  wood: new THREE.MeshStandardMaterial({ color: 0x8B4513, roughness: 0.8 }),
  concrete: new THREE.MeshStandardMaterial({ color: 0xA9A9A9, roughness: 0.9 }),
  steel: new THREE.MeshStandardMaterial({ color: 0x6A6A6A, metalness: 0.6, roughness: 0.4 }),
  soil: new THREE.MeshStandardMaterial({ color: 0x3d2b1f, roughness: 1 }), 
  default: new THREE.MeshStandardMaterial({ color: 0xcccccc })
};

/**
 * ThreeDView Component for rendering the 3D visualization.
 * Includes error boundaries for WebGL context failures.
 */
const ThreeDView = ({ dimensions, theme }) => {
  const mountRef = useRef(null);
  const sceneRef = useRef(null);
  const rendererRef = useRef(null);
  const cameraRef = useRef(null);
  const controlsRef = useRef(null);
  const objectRef = useRef(null);
  const animationFrameRef = useRef(null);
  
  const [loading, setLoading] = useState(true);
  const [hasError, setHasError] = useState(false);
  const [errorMessage, setErrorMessage] = useState('');

  // 1. Scene Initialization
  useEffect(() => {
    const currentMount = mountRef.current;
    if (!currentMount) return;

    // cleanup previous instances if strict mode runs twice
    if (rendererRef.current) return;

    try {
      // -- Scene --
      const scene = new THREE.Scene();
      sceneRef.current = scene;

      // -- Camera --
      const camera = new THREE.PerspectiveCamera(75, currentMount.clientWidth / currentMount.clientHeight, 0.1, 1000);
      camera.position.set(4, 4, 4);
      cameraRef.current = camera;

      // -- Renderer --
      // FIX: Wrapped in try-catch because new WebGLRenderer() throws if WebGL is unavailable
      const renderer = new THREE.WebGLRenderer({ 
        antialias: true, 
        alpha: true,
        powerPreference: "high-performance" // Try to force hardware acceleration
      });
      
      renderer.setSize(currentMount.clientWidth, currentMount.clientHeight);
      renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2)); // Limit pixel ratio for performance
      currentMount.appendChild(renderer.domElement);
      rendererRef.current = renderer;

      // -- Controls --
      const controls = new OrbitControls(camera, renderer.domElement);
      controls.enableDamping = true;
      controls.dampingFactor = 0.05;
      controlsRef.current = controls;

      // -- Lighting --
      const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
      scene.add(ambientLight);
      
      const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
      directionalLight.position.set(5, 10, 7.5);
      scene.add(directionalLight);

      setLoading(false);

    } catch (error) {
      console.error("Three.js Initialization Failed:", error);
      setLoading(false);
      setHasError(true);
      setErrorMessage(error.message || "WebGL is not supported on this device.");
    }

    // Cleanup on unmount
    return () => {
      cancelAnimationFrame(animationFrameRef.current);
      
      if (rendererRef.current) {
        // Dispose of the renderer context
        rendererRef.current.dispose();
        
        // Remove canvas from DOM
        if (currentMount && rendererRef.current.domElement) {
          if (currentMount.contains(rendererRef.current.domElement)) {
            currentMount.removeChild(rendererRef.current.domElement);
          }
        }
        rendererRef.current = null;
      }

      if (controlsRef.current) {
        controlsRef.current.dispose();
      }
    };
  }, []);

  // 2. Animation Loop
  const animate = useCallback(() => {
    if (!hasError && rendererRef.current && sceneRef.current && cameraRef.current) {
      if (controlsRef.current) controlsRef.current.update();
      rendererRef.current.render(sceneRef.current, cameraRef.current);
      animationFrameRef.current = requestAnimationFrame(animate);
    }
  }, [hasError]);

  useEffect(() => {
    if (!hasError && !loading) {
      animationFrameRef.current = requestAnimationFrame(animate);
    }
    return () => cancelAnimationFrame(animationFrameRef.current);
  }, [animate, hasError, loading]);

  // 3. Update Object Geometry based on Dimensions
  useEffect(() => {
    const scene = sceneRef.current;
    if (loading || hasError || !scene) return;

    // Clean up old object geometry/mesh
    if (objectRef.current) {
      scene.remove(objectRef.current);
      if (objectRef.current.geometry) objectRef.current.geometry.dispose();
      // We do not dispose materials here as they are const globals
    }

    const { width, length, height, material } = dimensions;
    
    // Safety parsing
    const geoWidth = Math.max(0.1, parseFloat(width) || 1);
    const geoLength = Math.max(0.1, parseFloat(length) || 1);
    const geoHeight = Math.max(0.01, parseFloat(height) || 0.1);
    
    // Select Material
    const selectedMaterial = MATERIALS[material] || MATERIALS.default;
    
    // Create Mesh
    const geometry = new THREE.BoxGeometry(geoWidth, geoHeight, geoLength);
    const mesh = new THREE.Mesh(geometry, selectedMaterial);
    
    // Center it on top of the "floor" (y=0)
    mesh.position.set(0, geoHeight / 2, 0);

    scene.add(mesh);
    objectRef.current = mesh;

    // Optional: Auto-adjust camera target to center of object
    if (controlsRef.current) {
        controlsRef.current.target.set(0, geoHeight / 2, 0);
    }

  }, [dimensions, loading, hasError]);

  // 4. Handle Window Resize
  useEffect(() => {
    const handleResize = () => {
      const currentMount = mountRef.current;
      if (currentMount && rendererRef.current && cameraRef.current) {
        const width = currentMount.clientWidth;
        const height = currentMount.clientHeight;
        
        cameraRef.current.aspect = width / height;
        cameraRef.current.updateProjectionMatrix();
        
        rendererRef.current.setSize(width, height);
      }
    };

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);
  
  // 5. Theme Sync
  useEffect(() => {
    if (sceneRef.current) {
        const color = theme === 'dark' ? 0x121212 : 0xfafafa;
        sceneRef.current.background = new THREE.Color(color);
    }
  }, [theme, loading, hasError]);

  // -- Render Phase --

  if (hasError) {
    return (
      <div className="w-full h-full min-h-[300px] flex flex-col items-center justify-center rounded-2xl bg-sand-100 dark:bg-charcoal-900 border border-red-200 dark:border-red-900/30 p-4 text-center">
        <p className="text-red-600 dark:text-red-400 font-semibold mb-2">3D Visualization Unavailable</p>
        <p className="text-sm text-charcoal-500 dark:text-steel-400 max-w-xs">
           Your browser or device was unable to initialize the graphics engine.
        </p>
        {/* Debug info hidden in summary detail if needed */}
        {import.meta.env.DEV && (
            <details className="mt-4 text-xs text-left w-full overflow-hidden text-gray-400">
                <summary>Error Details</summary>
                {errorMessage}
            </details>
        )}
      </div>
    );
  }

  return (
    <div 
        ref={mountRef} 
        className="relative w-full h-full min-h-[300px] md:min-h-[500px] rounded-2xl overflow-hidden bg-sand-100 dark:bg-charcoal-900 transition-colors"
    >
      {loading && (
        <div className="absolute inset-0 flex items-center justify-center bg-sand-100/50 dark:bg-charcoal-900/50 z-10">
          <span className="text-charcoal-500 dark:text-steel-400 animate-pulse">
            Initializing Engine...
          </span>
        </div>
      )}
    </div>
  );
};

export default ThreeDView;