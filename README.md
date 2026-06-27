# DBX - Distrobox Container Manager

A modern desktop application for managing [Distrobox](https://distrobox.it/) containers, built with Tauri, Vue 3, and TypeScript. DBX provides an intuitive graphical interface for creating, managing, and exporting applications from Linux containers to your host system.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Tauri](https://img.shields.io/badge/Tauri-2.0-FFC131?logo=tauri)
![Vue](https://img.shields.io/badge/Vue-3.5-4FC08D?logo=vue.js)
![TypeScript](https://img.shields.io/badge/TypeScript-5.6-3178C6?logo=typescript)

## ✨ Features

- **Container Management**: Create, start, stop, pause, and delete Distrobox containers with ease
- **App Exporting**: Seamlessly export applications from containers to your host system using `distrobox-export`
- **System Apps Listing**: View installed packages and applications within each container
- **Backup & Restore**: Create container backups and restore them when needed
- **Performance Monitoring**: Track container performance metrics with historical charts powered by uPlot
- **Auto-Refresh**: Real-time container status updates with configurable refresh intervals
- **Modern UI**: Clean, responsive interface built with Vuetify and Tailwind CSS

## 🚀 Getting Started

### Prerequisites

Before installing DBX, ensure you have the following installed:

- **[Podman](https://podman.io/)** or Docker (for container runtime)
- **[Distrobox](https://distrobox.it/)** (for container management and app exporting)
- **Node.js** (v18 or later)
- **Rust** (v1.70 or later) - required for building the Tauri backend

### Building From Source

1. **Clone the repository:**
   ```bash
   git clone https://github.com/dbx-manager/dbx.git
   cd dbx
2. Install frontend dependencies:
    ```bash
      npm install
4. Install Tauri CLI:
    ```bash
    cargo install tauri-cli

6. Run in development mode:
    ```bash
    npm run tauri:dev
7. Build for production:
   ```bash
    npm run build
    npm run tauri build

   
## Usage
  Once launched, DBX will automatically detect existing Distrobox containers on your system. You can:
  - **Create New Containers:** Click the "New Container" button to set up a fresh Distrobox environment
  - **Manage Containers:** Start, stop, pause, or delete containers from the main dashboard
  - **Export Applications:** Browse available apps within a container and export them to your host system
  - **View System Apps:** See all installed packages in each container
  - **Backup Containers:** Create and restore container snapshots
## 🛠️ Tech Stack
- **Frontend:** Vue 3 with <script setup> SFCs, TypeScript, Pinia (state management), Vue Router
- **UI Framework:** Vuetify 4, Tailwind CSS 4, Material Design Icons
- **Backend:** Rust with Tauri 2, Podman API integration
- **Build Tools:** Vite 6, vue-tsc
Charts: uPlot for performance monitoring

## 👥 Creators:
- **AGminuse - [Github Profile](https://github.com/AGminuse)**
- **Elie-AboZraa - [Github Profile](https://github.com/Elie-AboZraa)**

## 🤝 Contributing
Contributions are welcome! Please read our Contributing Guidelines before submitting pull requests.
1. Fork the repository
2. Create your feature branch (git checkout -b feature/amazing-feature)
3. Commit your changes (git commit -m 'Add amazing feature')
4. Push to the branch (git push origin feature/amazing-feature)
5. Open a Pull Request

## 📄 License
**This project is licensed under the MIT License - see the LICENSE file for details.**

## 🙏 Acknowledgments
- Distrobox - The foundation that makes this possible
- Podman - Container runtime
- Tauri - Building smaller, faster desktop apps
- Vue.js - Progressive JavaScript framework
