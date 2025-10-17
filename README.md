# 🎮 NeoForge Legends

**MVP Sprint Repository** - Open-world RPG sandbox blockchain game built with **Rust + Bevy**, targeting WebAssembly and desktop.

## 🚀 Quick Start

### Desktop Build
```bash
cargo run -p neoforge-game
```

### WASM Build
```bash
# Install wasm target
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli

# Build for web
cargo build -p neoforge-game --release --target wasm32-unknown-unknown --features wasm
wasm-bindgen --target web --no-typescript --out-dir web/dist target/wasm32-unknown-unknown/release/neoforge_game.wasm

# Serve locally
cd web && python3 -m http.server 8000
```

## 📁 Structure

```
├── crates/
│   ├── core/          # Core game engine (Vec2, systems, entities)
│   └── game/          # Main game logic (Bevy app, player, UI)
├── web/               # WASM web shell + Vite config
├── .github/workflows/ # CI for desktop + WASM builds
└── vercel.json        # Deploy config
```

## 🎯 MVP Features

✅ **Basic Game Loop** - Player movement with WASD/arrows  
✅ **Score System** - Points for movement  
✅ **Cross-Platform** - Desktop + WASM builds  
✅ **CI/CD Pipeline** - Auto-build and deploy  
🚧 **Blockchain Integration** - MultiversX SDK (next sprint)  
🚧 **Game World** - Procedural terrain and NPCs  
🚧 **Inventory System** - Items and crafting  

## 🛠 Development

- **Engine**: Bevy 0.14 with ECS architecture
- **Language**: Rust (stable toolchain)
- **Targets**: Desktop (native) + Web (WASM)
- **Deploy**: Vercel (web) + GitHub Releases (desktop)
- **CI**: GitHub Actions with caching

---

**Live Demo**: [Deployed on Vercel](https://neoforge-legends.vercel.app)  
**Status**: 🚀 Active Development | MVP Phase
