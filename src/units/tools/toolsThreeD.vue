<script setup lang="ts">
import {onMounted, ref, watch} from "vue";
import {
  BlockPos,
  Structure, StructureRenderer,
} from "deepslate";
import {CameraState, InteractiveCanvas} from "../../modules/deepslateInit.ts";
import {fetchSchematicData} from "../../modules/schematic_data.ts";
import {schematic_id} from "../../modules/tools_data.ts";
import {blocks_resources} from "../../modules/deepslateInit.ts";
import {toast} from "../../modules/others.ts";
const structure_l = ref<Structure>();
const size_l = ref<BlockPos>();
const camera_l = ref<CameraState>()
const loading = ref(false);
const currentLayer = ref(0);
const layers = ref<Record<number, Array<{pos: BlockPos, block: {id: string, properties: any}}>>>({});
const structureRenderer = ref<StructureRenderer | null>(null);
const interactiveCanvas = ref<InteractiveCanvas | null>(null);
const loadStructure = async () => {
  const schematic_data = await fetchSchematicData(schematic_id.value)
  const schematic_size = schematic_data.size
  const structure = new Structure([schematic_size.width, schematic_size.height, schematic_size.length])
  const blocks = schematic_data.blocks
  const size = structure.getSize()
  let minX = Infinity;
  let minY = Infinity;
  let minZ = Infinity;
  const validElements = [];

  for (const element of blocks.elements) {
    const pos = element.pos;
    if (!element.block) {
      console.warn('Element has no block:', element);
      continue;
    }
    if (typeof element.block.id === 'string' && element.block.id.toLowerCase() === 'minecraft:air') {
      continue;
    }
    if (typeof pos.x !== 'number' || typeof pos.y !== 'number' || typeof pos.z !== 'number') {
      continue;
    }
    const x = Math.round(pos.x);
    const y = Math.round(pos.y);
    const z = Math.round(pos.z);
    validElements.push(element);
    if (x < minX) minX = x;
    if (y < minY) minY = y;
    if (z < minZ) minZ = z;
  }
  layers.value = {};
  for (const element of validElements) {
    const pos = element.pos;
    const x = Math.round(pos.x - minX);
    const y = Math.round(pos.y - minY);
    const z = Math.round(pos.z - minZ);

    if (!layers.value[y]) {
      layers.value[y] = [];
    }
    layers.value[y].push({
      pos: [x, y, z],
      block: {
        id: element.block.id,
        properties: element.block.properties || {}
      }
    });
    structure.addBlock(
        [Math.round(pos.x - minX), Math.round(pos.y - minY), Math.round(pos.z - minZ)],
        element.block.id,
        element.block.properties || {}
    );

    structure_l.value = structure
    size_l.value = size
  }
}
const updateStructure = (targetLayer: number) => {
  if (!size_l.value) return;

  const newStructure = new Structure([...size_l.value]);
  for (let y = 0; y <= targetLayer; y++) {
    if (layers.value[y]) {
      layers.value[y].forEach(block => {
        newStructure.addBlock(block.pos, block.block.id, block.block.properties);
      });
    }
  }
  structure_l.value = newStructure;
}
const reloadRenderer = async () => {
  const structureCanvas = document.getElementById('structure-display') as HTMLCanvasElement
  const structureGl = structureCanvas.getContext('webgl')!
  if (interactiveCanvas.value) {
    camera_l.value = {
      xRotation: (interactiveCanvas.value as any).xRotation,
      yRotation: (interactiveCanvas.value as any).yRotation,
      viewDist: (interactiveCanvas.value as any).viewDist
    }
  }
  structureRenderer.value = new StructureRenderer(structureGl, structure_l.value, blocks_resources.value, {
    facesPerBuffer: 1000,
    chunkSize: 16,
    useInvisibleBlockBuffer: false}
  )

  interactiveCanvas.value = new InteractiveCanvas(structureCanvas,camera_l.value, view => {
    structureRenderer.value.drawStructure(view)
    structureRenderer.value.drawGrid(view)
  }, [size_l.value[0] / 2, size_l.value[1] / 2, size_l.value[2] / 2])
}
onMounted(async () => {
  try {
    loading.value = true;
    await loadStructure();
    await reloadRenderer();
    currentLayer.value = size_l.value[1] - 1;
  }catch (e) {
    toast.error(`发生了一个错误:${e}`, {timeout: 3000});
  }finally {
    loading.value = false;
  }
})

watch(currentLayer, (newVal) => {
  updateStructure(newVal);
  reloadRenderer();
});
</script>

<template>
  <div class="container">
    <canvas class="gpu-canvas" id="structure-display" width="1350" height="800"></canvas>
    <div class="slider-container">
      <input
          type="range"
          class="vertical-slider"
          v-model="currentLayer"
          :min="0"
          :max="size_l ? size_l[1] - 1 : 0"
          orient="vertical"
      />
      <div class="layer-indicator">当前层: {{ currentLayer }}</div>
    </div>
    <div v-if="loading" class="loading-overlay">
      <div class="loader">
        <div class="spinner"></div>
        <p>加载结构中...</p>
      </div>
    </div>

  </div>
</template>

<style scoped>
.container {
  position: relative;
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  width: 100%;
  margin: 0;
  padding: 0;
  transform: translateZ(0);
}

.gpu-canvas {
  image-rendering: -moz-crisp-edges;
  image-rendering: crisp-edges;
  -ms-interpolation-mode: nearest-neighbor;

  touch-action: none;
  transform-style: preserve-3d;
  will-change: transform;
}

#structure-display {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  box-shadow: 0 0 10px rgba(0,0,0,0.1);
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(255, 255, 255, 0.8);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 999;
}

.loader {
  text-align: center;
}

.spinner {
  width: 40px;
  height: 40px;
  margin: 0 auto;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #3498db;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.loader p {
  margin-top: 10px;
  color: #333;
  font-weight: bold;
}

.slider-container {
  position: absolute;
  right: 20px;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 15px;
  z-index: 100;
}

.vertical-slider {
  writing-mode: bt-lr;
  -webkit-appearance: slider-vertical;
  width: 8px;
  height: 200px;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.layer-indicator {
  background: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 5px 10px;
  border-radius: 4px;
  font-size: 14px;
}

</style>