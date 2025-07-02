
![[/prototype.png]]


# Crayon 🖍️

A collaborative 3D visualization application built with Rust and React, designed to replicate the infrastructure and user experience of Figma but for 3D content. C
## 🏗️ Architecture

### Frontend (React/Next.js)
- **Framework**: Next.js 15 with React 19
- **UI Components**: shadcn/ui with Radix UI primitives
- **Styling**: Tailwind CSS with custom design system
- **Collaboration**: Liveblocks for real-time features
- **Authentication**: Clerk for user management
- **State Management**: React Context + Tanstack Query

### 3D Canvas (Rust/Bevy/WebGPU)
- **Engine**: Bevy 0.15 game engine
- **Rendering**: WebGPU for high-performance graphics
- **Compilation**: Rust to WebAssembly for web deployment
- **Custom Systems**: Interactive meshes, camera controls, material systems
- **Event System**: Bidirectional communication with React frontend

### Backend Services
- **API Server**: Axum-based REST API
- **Database**: SurrealDB for flexible data modeling
- **File Storage**: AWS S3 for asset management
- **Request Client**: reqwest for HTTP communications




## 📁 Project Structure

```
crayon/
├── app/                    # Bevy 3D application (compiles to WASM)
│   ├── src/
│   │   ├── plugins/        # Bevy plugins for tools, interaction
│   │   ├── materials/      # Custom WebGPU materials and shaders
│   │   ├── meshes/         # 3D geometry and mesh handling
│   │   └── event/          # Event system for React ↔ Rust communication
│   └── assets/             # 3D assets, shaders, textures
├── www/                    # React/Next.js frontend
│   ├── src/
│   │   ├── components/     # React components with shadcn/ui
│   │   ├── providers/      # React contexts including WASM integration
│   │   └── bindings/       # Auto-generated TypeScript types from Rust
│   └── public/             # Static assets
├── server/                 # Axum REST API server
│   └── src/
│       ├── handlers/       # HTTP request handlers
│       ├── database/       # SurrealDB integration
│       └── file_storage/   # S3 integration
├── crates/                 # Custom Rust crates
│   ├── crayon_mcp/         # Model Context Protocol server
│   ├── bevy_*/            # Custom Bevy plugins and extensions
│   ├── crayon_core/       # Shared types and utilities
│   └── block3d_*/         # 3D block manipulation algorithms
└── cli/                   # Command-line utilities
```


## 🔧 Environment Variables

Create a `.env` file in the project root:

```env
# Server Configuration
AWS_ACCESS_KEY_ID=your_aws_key
AWS_SECRET_ACCESS_KEY=your_aws_secret
S3_REGION=us-east-1
AWS_BUCKET=your-bucket-name
SURREAL_URL=ws://localhost:8000

# Frontend Configuration (www/.env.local)
NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY=your_clerk_key
CLERK_SECRET_KEY=your_clerk_secret
NEXT_PUBLIC_LIVEBLOCKS_PUBLIC_KEY=your_liveblocks_key
LIVEBLOCKS_SECRET_KEY=your_liveblocks_secret
```


Start a surrealdb server
```bash
surreal start --log trace --user root --pass root memory
```

Start the server: 
```bash
cargo run --bin server
```

Run the nextjs application:
```bash
cd www
npm i
npm run dev
```

## 🙏 Acknowledgments

- [Bevy](https://bevyengine.org/) - Fantastic Rust game engine
- [shadcn/ui](https://ui.shadcn.com/) - Beautiful React components
- [Liveblocks](https://liveblocks.io/) - Real-time collaboration infrastructure
- [MCP](https://modelcontextprotocol.io/) - Model Context Protocol specification

---
