"use client";

// Direct React port of
// https://gitlab.com/side_project/chill-component/-/blob/main/src/components/bg-sakura-fubuki/bg-sakura-fubuki.vue
// Uses @babylonjs/core (same rendering stack as the reference).

import { useEffect, useRef, useState } from "react";
import {
  ArcRotateCamera,
  Color3,
  Color4,
  DefaultRenderingPipeline,
  DepthOfFieldEffectBlurLevel,
  DynamicTexture,
  Engine,
  HemisphericLight,
  Matrix,
  MeshBuilder,
  Scene,
  StandardMaterial,
  Vector3,
} from "@babylonjs/core";

type Vec3 = Record<"x" | "y" | "z", number>;
type ParticleSize = Record<"width" | "height", number>;

type SakuraChildren =
  | React.ReactNode
  | ((ctx: { fps: number }) => React.ReactNode);

export interface BgSakuraFubukiProps {
  particleSrc?: string;
  particleSize?: ParticleSize;
  capacity?: number;
  velocity?: Vec3;
  className?: string;
  children?: SakuraChildren;
  /** When false, fallen petals are not respawned — the storm tapers off. */
  emit?: boolean;
  /** Fires once after emit=false and every petal has fallen off. */
  onComplete?: () => void;
}

const DEFAULT_PARTICLE_SRC = "/sakura-petal.png";
const DEFAULT_PARTICLE_SIZE: ParticleSize = { width: 0.7, height: 1 };
const DEFAULT_CAPACITY = 500;
const DEFAULT_VELOCITY: Vec3 = { x: 0.01, y: -0.02, z: 0.01 };

export default function BgSakuraFubuki({
  particleSrc = DEFAULT_PARTICLE_SRC,
  particleSize = DEFAULT_PARTICLE_SIZE,
  capacity = DEFAULT_CAPACITY,
  velocity = DEFAULT_VELOCITY,
  className,
  children,
  emit = true,
  onComplete,
}: BgSakuraFubukiProps) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [fps, setFps] = useState(0);
  // Kept as refs so we can flip them without re-running the Babylon init effect.
  const emitRef = useRef(emit);
  const onCompleteRef = useRef(onComplete);
  emitRef.current = emit;
  onCompleteRef.current = onComplete;

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const engine = new Engine(canvas, true, {
      antialias: true,
      adaptToDeviceRatio: true,
    });
    // Render at the device's real pixel density — kills the jagged edges.
    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    engine.setHardwareScalingLevel(1 / dpr);

    const scene = new Scene(engine);
    scene.createDefaultLight();
    const defaultLight = scene.lights.at(-1);
    if (defaultLight instanceof HemisphericLight) {
      defaultLight.intensity = 1;
      defaultLight.direction = new Vector3(0.5, 1, 0);
      defaultLight.diffuse = Color3.White();
      defaultLight.groundColor = Color3.White();
    }

    const camera = new ArcRotateCamera(
      "camera",
      Math.PI / 2,
      Math.PI / 2,
      50,
      new Vector3(0, 0, 0),
      scene,
    );

    scene.clearColor = new Color4(0, 0, 0, 0);

    // Rendering pipeline: DoF + bloom (reference uses these; tuned so petals
    // stay recognisable in the small card — the reference runs full-viewport).
    const pipeline = new DefaultRenderingPipeline(
      "defaultPipeline",
      false,
      scene,
      [camera],
    );
    pipeline.samples = 4; // MSAA x4 on the pipeline render target
    pipeline.fxaaEnabled = true;
    pipeline.depthOfFieldEnabled = true;
    pipeline.depthOfField.focusDistance = 37000;
    pipeline.depthOfField.focalLength = 5000;
    pipeline.depthOfField.fStop = 8;
    pipeline.depthOfFieldBlurLevel = DepthOfFieldEffectBlurLevel.Low;
    pipeline.bloomEnabled = true;
    pipeline.bloomThreshold = 0.9;
    pipeline.bloomWeight = 0.25;
    pipeline.bloomKernel = 32;
    pipeline.bloomScale = 0.5;

    // Particle field (thin instances).
    const box = MeshBuilder.CreateBox(
      "box",
      {
        ...particleSize,
        depth: 0.001,
      },
      scene,
    );

    const size = camera.radius * 1.5;
    const matricesData = new Float32Array(16 * capacity);
    // All petals start above the camera view, staggered vertically, so the
    // storm drifts in from the top rather than popping in all at once.
    // y spans [size/2, size/2 + size*2] — first petals cross the view quickly,
    // last ones arrive after the stagger height has fallen through.
    const staggerHeight = size * 2;
    const aliveFlags = new Uint8Array(capacity).fill(1);
    for (let i = 0; i < capacity; i += 1) {
      const m = Matrix.Translation(
        Math.random() * size - size / 2,
        size / 2 + Math.random() * staggerHeight,
        Math.random() * size - size / 2,
      );
      m.copyToArray(matricesData, i * 16);
    }
    box.thinInstanceSetBuffer("matrix", matricesData, 16);

    const material = new StandardMaterial("material", scene);
    material.specularColor = new Color3(0.1, 0.1, 0.1);

    const dynamicTexture = new DynamicTexture(
      "dynamicTexture",
      { width: 124, height: 180 },
      scene,
    );
    dynamicTexture.hasAlpha = true;
    dynamicTexture.anisotropicFilteringLevel = 16;

    let disposed = false;
    const img = new Image();
    img.onload = () => {
      if (disposed) return;
      const ctx = dynamicTexture.getContext();
      ctx.drawImage(img, 0, 0, 124, 180);
      dynamicTexture.update();
    };
    img.src = particleSrc;

    material.diffuseTexture = dynamicTexture;
    box.material = material;

    let completed = false;
    const beforeRender = () => {
      const time = performance.now() * 0.001;
      let aliveCount = 0;
      for (let i = 0; i < capacity; i += 1) {
        if (!aliveFlags[i]) continue;
        const offset = i * 16;
        const originalX = matricesData[offset + 12] ?? 0;
        const originalY = matricesData[offset + 13] ?? 0;
        const originalZ = matricesData[offset + 14] ?? 0;

        let y = originalY + velocity.y;
        let x = originalX + 0.005 * Math.sin(time + i * 0.01) + velocity.x;
        let z = originalZ + 0.005 * Math.cos(time + i * 0.01) + velocity.z;

        if (y < -size / 2) {
          if (emitRef.current) {
            // Respawn from the top, fresh horizontal position.
            y = size / 2;
            x = Math.random() * size - size / 2;
            z = Math.random() * size - size / 2;
          } else {
            // Park the petal far below and hide it for the rest of the run.
            aliveFlags[i] = 0;
            const hideMatrix = Matrix.Translation(0, -1e6, 0);
            hideMatrix.copyToArray(matricesData, offset);
            continue;
          }
        }

        aliveCount += 1;
        const angle = time * 0.5 + i * 0.1;
        const rotationMatrix = Matrix.RotationYawPitchRoll(angle, angle, angle);
        const translationMatrix = Matrix.Translation(x, y, z);
        const finalMatrix = rotationMatrix.multiply(translationMatrix);
        finalMatrix.copyToArray(matricesData, offset);
      }
      box.thinInstanceSetBuffer("matrix", matricesData, 16);

      if (!emitRef.current && aliveCount === 0 && !completed) {
        completed = true;
        onCompleteRef.current?.();
      }
    };
    scene.registerBeforeRender(beforeRender);

    engine.runRenderLoop(() => scene.render());

    const fpsInterval = window.setInterval(() => {
      setFps(Math.floor(engine.getFps() || 0));
    }, 100);

    const handleResize = () => engine.resize();
    window.addEventListener("resize", handleResize);

    return () => {
      disposed = true;
      window.removeEventListener("resize", handleResize);
      window.clearInterval(fpsInterval);
      scene.unregisterBeforeRender(beforeRender);
      scene.dispose();
      engine.dispose();
    };
  }, [
    capacity,
    particleSize.height,
    particleSize.width,
    particleSrc,
    velocity.x,
    velocity.y,
    velocity.z,
  ]);

  const classes = [
    "pointer-events-none absolute inset-0 overflow-hidden",
    className,
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <div className={classes}>
      <canvas ref={canvasRef} className="block h-full w-full" aria-hidden />
      {typeof children === "function" ? children({ fps }) : children}
    </div>
  );
}
