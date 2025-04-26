/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}
// src/env.d.ts
declare module '../../router' {
  import { Router } from 'vue-router';
  const router: Router;
  export default router;
}
