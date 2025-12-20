use anyhow::{Result, Context};
use std::path::Path;
use std::fs;
use crate::ui;

pub struct Template {
    pub name: &'static str,
    pub description: &'static str,
}

pub const TEMPLATES: &[Template] = &[
    Template {
        name: "express-ts",
        description: "Minimal Express.js server with TypeScript",
    },
    Template {
        name: "vite-react-ts",
        description: "Vite + React + TypeScript starter",
    },
    Template {
        name: "simple-ts",
        description: "Basic TypeScript console application",
    },
];

pub fn create_project(template_name: &str, project_name: &str) -> Result<()> {
    let target_dir = Path::new(project_name);
    if target_dir.exists() {
        anyhow::bail!("Directory '{}' already exists", project_name);
    }

    fs::create_dir_all(target_dir)?;

    match template_name {
        "express-ts" => scaffold_express_ts(target_dir, project_name)?,
        "vite-react-ts" => scaffold_vite_ts(target_dir, project_name)?,
        "simple-ts" => scaffold_simple_ts(target_dir, project_name)?,
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

fn scaffold_vite_ts(dir: &Path, name: &str) -> Result<()> {
    ui::print_step("🏗️", "Scaffolding Vite React TypeScript project...");
    
    let pkg_json = serde_json::json!({
        "name": name,
        "version": "1.0.0",
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
            "@vitejs/plugin-react": "^4.0.0",
            "typescript": "^5.0.0",
            "vite": "^4.4.0"
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
      <h1>Hello from Crabby Vite! 🦀</h1>
    </div>
  </React.StrictMode>,
)
"#)?;
    fs::write(dir.join("src/index.css"), "body { font-family: sans-serif; background: #121212; color: white; }\n")?;

    Ok(())
}
