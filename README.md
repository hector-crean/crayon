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


## 🔄 Technical Challenges Solved

### 1. React ↔ Rust Communication
**Challenge**: Seamless bidirectional communication between React frontend and Rust canvas.

**Solution**: 
- Custom event system using `wasm-bindgen` for type-safe message passing
- TypeScript bindings auto-generated from Rust types using `ts-rs`
- Event queuing system for reliable message delivery
- WASM module lifecycle management with proper cleanup

```typescript
// Frontend sends events to Rust canvas
sendCrayonEvent({ type: 'ChangeTool', data: 'Transform' });

// Rust processes events and can send responses back
drainEventQueue(); // Gets events from Rust
```


### 2. Real-time Collaboration
**Challenge**: Synchronizing 3D scene state across multiple users.

**Solution**:
- Liveblocks for operational transformation
- Optimistic updates with conflict resolution
- Presence cursors in 3D space
- Event sourcing for reliable state synchronization



## 🚀 Getting Started

### Prerequisites

- **Rust** (latest stable) with `wasm32-unknown-unknown` target
- **Node.js** 18+ with npm/yarn
- **wasm-bindgen-cli**: `cargo install wasm-bindgen-cli`
- **wasm-opt** (optional): For WASM optimization

### Installation

1. **Clone the repository**
```bash
git clone https://github.com/yourusername/crayon.git
cd crayon
```

2. **Install Rust dependencies**
```bash
cargo build
```

3. **Install Node.js dependencies**
```bash
cd www
npm install
cd ..
```

4. **Build WASM module**
```bash
./build_wasm.sh
```

### Development

#### Start the development servers:

**Frontend (React)**:
```bash
cd www
npm run dev
```

**Backend (Rust)**:
```bash
cargo run -p server --bin server
```

**MCP Server**:
```bash
cargo run -p crayon_mcp --bin crayon_mcp_server
```

#### Build for production:
```bash
# Build optimized WASM
./build_wasm.sh

# Build frontend
cd www
npm run build
```

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

## 🎨 Tools & Interaction

Crayon provides several specialized tools for 3D scene manipulation:

- **Transform Tool**: Move, rotate, and scale 3D objects
- **Comment Tool**: Add contextual annotations to 3D elements
- **Markup Tool**: Draw or highlight areas in 3D space
- **Block Tool**: Create and manipulate voxel-based structures

Each tool can be controlled via:
- UI interactions in the React frontend
- Keyboard shortcuts and gestures


### Usage Example
```bash
# Start MCP server
cargo run -p crayon_mcp --bin crayon_mcp_server

# Connect your AI agent to use Crayon tools
# The MCP server exposes all tools with proper JSON schemas
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

## 🧪 Testing

```bash
# Run Rust tests
cargo test

# Run frontend tests
cd www
npm test

# Test WASM module
wasm-pack test --headless --firefox
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and ensure WASM builds successfully
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 📄 License

This project is licensed under the MIT OR Apache-2.0 License - see the [LICENSE](LICENSE) files for details.

## 🙏 Acknowledgments

- [Bevy](https://bevyengine.org/) - Fantastic Rust game engine
- [shadcn/ui](https://ui.shadcn.com/) - Beautiful React components
- [Liveblocks](https://liveblocks.io/) - Real-time collaboration infrastructure
- [MCP](https://modelcontextprotocol.io/) - Model Context Protocol specification

---

Built with ❤️ using Rust, React, and WebGPU 