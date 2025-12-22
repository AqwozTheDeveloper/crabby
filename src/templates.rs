use anyhow::{Result, Context};
use std::path::Path;
use std::fs;
use crate::ui;

pub struct Template {
    pub name: &'static str,
    pub description: &'static str,
}

pub const TEMPLATES: &[Template] = &[
    // Backend Templates - TypeScript
    Template { name: "express-ts", description: "Express.js server with TypeScript" },
    Template { name: "fastify-ts", description: "Fastify framework with TypeScript" },
    
    // Backend Templates - JavaScript
    Template { name: "express", description: "Express.js server with JavaScript" },
    
    // Frontend Templates - React
    Template { name: "vite-react-ts", description: "Vite + React + TypeScript" },
    Template { name: "vite-react", description: "Vite + React + JavaScript" },
    Template { name: "next-app", description: "Next.js 14 App Router + TypeScript" },
    
    // Frontend Templates - Vue
    Template { name: "vite-vue-ts", description: "Vite + Vue 3 + TypeScript" },
    Template { name: "vite-vue", description: "Vite + Vue 3 + JavaScript" },
    
    // Frontend Templates - Svelte
    Template { name: "vite-svelte-ts", description: "Vite + Svelte + TypeScript" },
    Template { name: "vite-svelte", description: "Vite + Svelte + JavaScript" },
    
    // Frontend Templates - Vanilla
    Template { name: "vite-vanilla-ts", description: "Vite + TypeScript (no framework)" },
    Template { name: "vite-vanilla", description: "Vite + JavaScript (no framework)" },
    
    // Simple/Console
    Template { name: "simple-ts", description: "Basic TypeScript console app" },
    Template { name: "simple-js", description: "Basic JavaScript console app" },
];

pub fn create_project(template_name: &str, project_name: &str) -> Result<()> {
    let target_dir = Path::new(project_name);
    if target_dir.exists() {
        anyhow::bail!("Directory '{}' already exists", project_name);
    }

    fs::create_dir_all(target_dir)?;

    match template_name {
        "express-ts" => scaffold_express_ts(target_dir, project_name)?,
        "fastify-ts" => scaffold_fastify_ts(target_dir, project_name)?,
        "vite-react-ts" => scaffold_vite_react_ts(target_dir, project_name)?,
        "next-app" => scaffold_next_app(target_dir, project_name)?,
        "vite-vue-ts" => scaffold_vite_vue_ts(target_dir, project_name)?,
        "vite-svelte-ts" => scaffold_vite_svelte_ts(target_dir, project_name)?,
        "vite-vanilla-ts" => scaffold_vite_vanilla_ts(target_dir, project_name)?,
        "simple-ts" => scaffold_simple_ts(target_dir, project_name)?,
        "express" => scaffold_express(target_dir, project_name)?,
        "vite-react" => scaffold_vite_react(target_dir, project_name)?,
        "vite-vue" => scaffold_vite_vue(target_dir, project_name)?,
        "vite-svelte" => scaffold_vite_svelte(target_dir, project_name)?,
        "vite-vanilla" => scaffold_vite_vanilla(target_dir, project_name)?,
        "simple-js" => scaffold_simple_js(target_dir, project_name)?,
        _ => anyhow::bail!("Template '{}' not found", template_name),
    }

    Ok(())
}

fn scaffold_express_ts(dir: &Path, name: &str) -> Result<()> {
    ui::print_step("🏗️", "Scaffolding Express TypeScript project...");
    
    fs::create_dir_all(dir.join("src"))?;
    
    let pkg_json = serde_json::json!({
        "name": name,
        "version": "1.0.0",
        "description": "Express server created with Crabby",
        "scripts": {
            "dev": "crabby run src/index.ts --listen",
            "build": "tsc",
            "start": "node dist/index.js"
        },
        "dependencies": {
            "express": "^4.18.2"
        },
        "devDependencies": {
            "@types/express": "^4.17.17",
            "@types/node": "^20.0.0",
            "typescript": "^5.0.0",
            "tsx": "^4.0.0"
        }
    });

    fs::write(dir.join("package.json"), serde_json::to_string_pretty(&pkg_json)?)?;
    
    let index_ts = r#"import express from 'express';

const app = express();
const port = 3000;

app.get('/', (req, res) => {
  res.send('Hello from Crabby Express! 🦀');
});

app.listen(port, () => {
  console.log(`🚀 Server ready at http://localhost:${port}`);
});
"#;
    fs::write(dir.join("src/index.ts"), index_ts)?;

    let tsconfig = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "moduleResolution": "node",
    "esModuleInterop": true,
    "strict": true,
    "skipLibCheck": true,
    "outDir": "./dist",
    "typeRoots": ["./node_modules/@types"]
  },
  "include": ["src/**/*.ts"]
}"#;
    fs::write(dir.join("tsconfig.json"), tsconfig)?;

    Ok(())
}

fn scaffold_fastify_ts(dir: &Path, name: &str) -> Result<()> {
    ui::print_step("🏗️", "Scaffolding Fastify TypeScript project...");
    
    fs::create_dir_all(dir.join("src"))?;
    
    let pkg_json = serde_json::json!({
        "name": name,
        "version": "1.0.0",
        "description": "Fastify server created with Crabby",
        "scripts": {
            "dev": "crabby run src/index.ts --listen",
            "build": "tsc",
            "start": "node dist/index.js"
        },
        "dependencies": {
            "fastify": "^4.25.0"
        },
        "devDependencies": {
            "@types/node": "^20.0.0",
            "typescript": "^5.0.0",
            "tsx": "^4.0.0"
        }
    });

    fs::write(dir.join("package.json"), serde_json::to_string_pretty(&pkg_json)?)?;
    
    let index_ts = r#"import Fastify from 'fastify';

const fastify = Fastify({
  logger: true
});

fastify.get('/', async (request, reply) => {
  return { message: 'Hello from Crabby Fastify! 🦀⚡' };
});

const start = async () => {
  try {
    await fastify.listen({ port: 3000 });
    console.log('🚀 Server ready at http://localhost:3000');
  } catch (err) {
    fastify.log.error(err);
    process.exit(1);
  }
};

start();
"#;
    fs::write(dir.join("src/index.ts"), index_ts)?;

    let tsconfig = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "moduleResolution": "node",
    "esModuleInterop": true,
    "strict": true,
    "skipLibCheck": true,
    "outDir": "./dist"
  },
  "include": ["src/**/*.ts"]
}"#;
    fs::write(dir.join("tsconfig.json"), tsconfig)?;

    Ok(())
}

fn scaffold_simple_ts(dir: &Path, name: &str) -> Result<()> {
    ui::print_step("🏗️", "Scaffolding Simple TypeScript project...");
    
    fs::create_dir_all(dir.join("src"))?;
    
    let pkg_json = serde_json::json!({
        "name": name,
        "version": "1.0.0",
        "scripts": {
            "start": "crabby run src/index.ts"
        },
        "devDependencies": {
            "@types/node": "^20.0.0",
            "typescript": "^5.0.0",
            "tsx": "^4.0.0"
        }
    });

    fs::write(dir.join("package.json"), serde_json::to_string_pretty(&pkg_json)?)?;
    fs::write(dir.join("src/index.ts"), "console.log('Hello from Crabby! 🦀');\n")?;

    Ok(())
}

fn scaffold_vite_react_ts(dir: &Path, name: &str) -> Result<()> {
    ui::print_step("🏗️", "Scaffolding Vite React TypeScript project...");
    
    let pkg_json = serde_json::json!({
        "name": name,
        "version": "1.0.0",
        "type": "module",
        "scripts": {
            "dev": "vite",
            "build": "tsc && vite build",
            "preview": "vite preview"
        },
        "dependencies": {
            "react": "^18.2.0",
            "react-dom": "^18.2.0"
        },
        "devDependencies": {
            "@types/react": "^18.2.0",
            "@types/react-dom": "^18.2.0",
            "@vitejs/plugin-react": "^4.2.0",
            "typescript": "^5.0.0",
            "vite": "^5.0.0"
        }
    });

    fs::write(dir.join("package.json"), serde_json::to_string_pretty(&pkg_json)?)?;
    
    fs::write(dir.join("index.html"), r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Crabby Vite React App</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
"#)?;

    fs::create_dir_all(dir.join("src"))?;
    fs::write(dir.join("src/main.tsx"), r#"import React from 'react'
import ReactDOM from 'react-dom/client'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <div style={{ textAlign: 'center', marginTop: '50px' }}>
      <h1>Hello from Crabby Vite + React! 🦀⚛️</h1>
      <p>Edit src/main.tsx and save to test hot reload</p>
    </div>
  </React.StrictMode>,
)
"#)?;
    
    fs::write(dir.join("src/index.css"), r#"body {
  font-family: system-ui, -apple-system, sans-serif;
  background: #0d1117;
  color: #c9d1d9;
  margin: 0;
  padding: 0;
  min-height: 100vh;
}

h1 {
  color: #58a6ff;
}
"#)?;

    fs::write(dir.join("vite.config.ts"), r#"import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
})
"#)?;

    fs::write(dir.join("tsconfig.json"), r#"{
  "compilerOptions": {
    "target": "ES2020",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"]
}
"#)?;

    Ok(())
}

fn scaffold_next_app(dir: &Path, name: &str) -> Result<()> {
    ui::print_step("🏗️", "Scaffolding Next.js App Router project...");
    
    let pkg_json = serde_json::json!({
        "name": name,
        "version": "1.0.0",
        "scripts": {
            "dev": "next dev",
            "build": "next build",
            "start": "next start",
            "lint": "next lint"
        },
        "dependencies": {
            "react": "^18.2.0",
            "react-dom": "^18.2.0",
            "next": "^14.0.0"
        },
        "devDependencies": {
            "@types/node": "^20.0.0",
            "@types/react": "^18.2.0",
            "@types/react-dom": "^18.2.0",
            "typescript": "^5.0.0"
        }
    });

    fs::write(dir.join("package.json"), serde_json::to_string_pretty(&pkg_json)?)?;
    
    fs::create_dir_all(dir.join("app"))?;
    fs::write(dir.join("app/page.tsx"), r#"export default function Home() {
  return (
    <main style={{ textAlign: 'center', marginTop: '100px' }}>
      <h1>Hello from Crabby Next.js! 🦀</h1>
      <p>Edit app/page.tsx and save to see your changes</p>
    </main>
  )
}
"#)?;

    fs::write(dir.join("app/layout.tsx"), r#"import type { Metadata } from 'next'

export const metadata: Metadata = {
  title: 'Crabby Next.js App',
  description: 'Created with Crabby package manager',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
"#)?;

    fs::write(dir.join("app/globals.css"), r#"body {
  font-family: system-ui, -apple-system, sans-serif;
  background: #0d1117;
  color: #c9d1d9;
  margin: 0;
  padding: 0;
}
"#)?;

    fs::write(dir.join("next.config.js"), r#"/** @type {import('next').NextConfig} */
const nextConfig = {}

module.exports = nextConfig
"#)?;

    fs::write(dir.join("tsconfig.json"), r#"{
  "compilerOptions": {
    "target": "ES2017",
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "plugins": [{ "name": "next" }],
    "paths": {
      "@/*": ["./*"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}
"#)?;

    Ok(())
}

fn scaffold_vite_vue_ts(dir: &Path, name: &str) -> Result<()> {
    ui::print_step("🏗️", "Scaffolding Vite Vue TypeScript project...");
    
    let pkg_json = serde_json::json!({
        "name": name,
        "version": "1.0.0",
        "type": "module",
        "scripts": {
            "dev": "vite",
            "build": "vue-tsc && vite build",
            "preview": "vite preview"
        },
        "dependencies": {
            "vue": "^3.3.0"
        },
        "devDependencies": {
            "@vitejs/plugin-vue": "^5.0.0",
            "typescript": "^5.0.0",
            "vue-tsc": "^1.8.0",
            "vite": "^5.0.0"
        }
    });

    fs::write(dir.join("package.json"), serde_json::to_string_pretty(&pkg_json)?)?;
    
    fs::write(dir.join("index.html"), r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Crabby Vite Vue App</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
"#)?;

    fs::create_dir_all(dir.join("src"))?;
    fs::write(dir.join("src/main.ts"), r#"import { createApp } from 'vue'
import './style.css'
import App from './App.vue'

createApp(App).mount('#app')
"#)?;

    fs::write(dir.join("src/App.vue"), r#"<script setup lang="ts">
import { ref } from 'vue'

const count = ref(0)
</script>

<template>
  <div class="container">
    <h1>Hello from Crabby Vite + Vue! 🦀💚</h1>
    <p>Edit src/App.vue and save to test hot reload</p>
    <button @click="count++">Count: {{ count }}</button>
  </div>
</template>

<style scoped>
.container {
  text-align: center;
  margin-top: 50px;
}

button {
  background: #42b883;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 5px;
  cursor: pointer;
  font-size: 16px;
}

button:hover {
  background: #35a372;
}
</style>
"#)?;

    fs::write(dir.join("src/style.css"), r#"body {
  font-family: system-ui, -apple-system, sans-serif;
  background: #0d1117;
  color: #c9d1d9;
  margin: 0;
  padding: 0;
}

h1 {
  color: #42b883;
}
"#)?;

    fs::write(dir.join("vite.config.ts"), r#"import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
})
"#)?;

    fs::write(dir.join("tsconfig.json"), r#"{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "preserve",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src/**/*.ts", "src/**/*.vue"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
"#)?;

    Ok(())
}

fn scaffold_vite_svelte_ts(dir: &Path, name: &str) -> Result<()> {
    ui::print_step("🏗️", "Scaffolding Vite Svelte TypeScript project...");
    
    let pkg_json = serde_json::json!({
        "name": name,
        "version": "1.0.0",
        "type": "module",
        "scripts": {
            "dev": "vite",
            "build": "vite build",
            "preview": "vite preview",
            "check": "svelte-check --tsconfig ./tsconfig.json"
        },
        "devDependencies": {
            "@sveltejs/vite-plugin-svelte": "^3.0.0",
            "svelte": "^4.2.0",
            "svelte-check": "^3.6.0",
            "tslib": "^2.6.0",
            "typescript": "^5.0.0",
            "vite": "^5.0.0"
        }
    });

    fs::write(dir.join("package.json"), serde_json::to_string_pretty(&pkg_json)?)?;
    
    fs::write(dir.join("index.html"), r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Crabby Vite Svelte App</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
"#)?;

    fs::create_dir_all(dir.join("src"))?;
    fs::write(dir.join("src/main.ts"), r#"import './app.css'
import App from './App.svelte'

const app = new App({
  target: document.getElementById('app')!,
})

export default app
"#)?;

    fs::write(dir.join("src/App.svelte"), r#"<script lang="ts">
  let count: number = 0
</script>

<main>
  <h1>Hello from Crabby Vite + Svelte! 🦀🧡</h1>
  <p>Edit src/App.svelte and save to test hot reload</p>
  <button on:click={() => count++}>
    Count: {count}
  </button>
</main>

<style>
  main {
    text-align: center;
    margin-top: 50px;
  }

  button {
    background: #ff3e00;
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 5px;
    cursor: pointer;
    font-size: 16px;
  }

  button:hover {
    background: #e63900;
  }
</style>
"#)?;

    fs::write(dir.join("src/app.css"), r#"body {
  font-family: system-ui, -apple-system, sans-serif;
  background: #0d1117;
  color: #c9d1d9;
  margin: 0;
  padding: 0;
}

h1 {
  color: #ff3e00;
}
"#)?;

    fs::write(dir.join("vite.config.ts"), r#"import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

export default defineConfig({
  plugins: [svelte()],
})
"#)?;

    fs::write(dir.join("svelte.config.js"), r#"import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

export default {
  preprocess: vitePreprocess(),
}
"#)?;

    fs::write(dir.join("tsconfig.json"), r#"{
  "extends": "@tsconfig/svelte/tsconfig.json",
  "compilerOptions": {
    "target": "ESNext",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "resolveJsonModule": true,
    "allowJs": true,
    "checkJs": true,
    "isolatedModules": true
  },
  "include": ["src/**/*.d.ts", "src/**/*.ts", "src/**/*.js", "src/**/*.svelte"]
}
"#)?;

    Ok(())
}

fn scaffold_vite_vanilla_ts(dir: &Path, name: &str) -> Result<()> {
    ui::print_step("🏗️", "Scaffolding Vite Vanilla TypeScript project...");
    
    let pkg_json = serde_json::json!({
        "name": name,
        "version": "1.0.0",
        "type": "module",
        "scripts": {
            "dev": "vite",
            "build": "tsc && vite build",
            "preview": "vite preview"
        },
        "devDependencies": {
            "typescript": "^5.0.0",
            "vite": "^5.0.0"
        }
    });

    fs::write(dir.join("package.json"), serde_json::to_string_pretty(&pkg_json)?)?;
    
    fs::write(dir.join("index.html"), r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Crabby Vite App</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
"#)?;

    fs::create_dir_all(dir.join("src"))?;
    fs::write(dir.join("src/main.ts"), r#"import './style.css'

const app = document.querySelector<HTMLDivElement>('#app')!

app.innerHTML = `
  <div class="container">
    <h1>Hello from Crabby Vite! 🦀</h1>
    <p>Edit src/main.ts and save to test hot reload</p>
    <button id="counter">Count: 0</button>
  </div>
`

let count = 0
const button = document.querySelector<HTMLButtonElement>('#counter')!
button.addEventListener('click', () => {
  count++
  button.textContent = `Count: ${count}`
})
"#)?;

    fs::write(dir.join("src/style.css"), r#"body {
  font-family: system-ui, -apple-system, sans-serif;
  background: #0d1117;
  color: #c9d1d9;
  margin: 0;
  padding: 0;
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
}

.container {
  text-align: center;
}

h1 {
  color: #fbbf24;
  font-size: 3em;
  margin-bottom: 0.5em;
}

button {
  background: #3b82f6;
  color: white;
  border: none;
  padding: 12px 24px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
  transition: background 0.2s;
}

button:hover {
  background: #2563eb;
}
"#)?;

    fs::write(dir.join("tsconfig.json"), r#"{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"]
}
"#)?;

    Ok(())
}
fn scaffold_express(dir: &Path, name: &str) -> Result<()> { anyhow::bail!("JS templates coming soon! Use 'express-ts' for now.") }
fn scaffold_vite_react(dir: &Path, name: &str) -> Result<()> { anyhow::bail!("JS templates coming soon! Use 'vite-react-ts' for now.") }
fn scaffold_vite_vue(dir: &Path, name: &str) -> Result<()> { anyhow::bail!("JS templates coming soon! Use 'vite-vue-ts' for now.") }
fn scaffold_vite_svelte(dir: &Path, name: &str) -> Result<()> { anyhow::bail!("JS templates coming soon! Use 'vite-svelte-ts' for now.") }
fn scaffold_vite_vanilla(dir: &Path, name: &str) -> Result<()> { anyhow::bail!("JS templates coming soon! Use 'vite-vanilla-ts' for now.") }
fn scaffold_simple_js(dir: &Path, name: &str) -> Result<()> { anyhow::bail!("JS templates coming soon! Use 'simple-ts' for now.") }
