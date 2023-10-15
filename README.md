# Tauri + React + TypeScript + Vite + Tailwind 

This template provides a setup to get React working in Vite, Tauri, Tailwind and some ESLint rules.

## Getting Started

### Prerequisites

You can find more detailed information about prerequisites [here](https://tauri.app/v1/guides/getting-started/prerequisites).

#### For Windows:
- Install `Microsoft Visual Studio C++ Build Tools` from [here](https://visualstudio.microsoft.com/visual-cpp-build-tools/). Select `C++ build tools` and `Windows 10 SDK` in the installer.
- Download and run the `Evergreen Bootstrapper` from [here](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section).
- Install `Rust` with `rustup` from [here](https://www.rust-lang.org/tools/install).

#### For Linux(Debian/Ubuntu):

- Run the following commands in your terminal:

   ```bash
   sudo apt update
   sudo apt install libwebkit2gtk-4.0-dev \
      build-essential \
      curl \
      wget \
      file \
      libssl-dev \
      libgtk-3-dev \
      libayatana-appindicator3-dev \
      librsvg2-dev
   ```

- To install Rust on Linux, open a terminal and enter the following command:

   ```bash
   curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
   ```

#### For MacOS:

- To install Rust on macOS, open a terminal and enter the following command:

   ```bash
   curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
   ```

### Installation

1. Clone the repo
   ```bash
   git clone https://github.com/emrecancorapci/tauri-react-typescript-tailwind
   ```
2. Install NPM packages
   ```bash
   pnpm install
   ```

## Usage

### Development

```bash
pnpm t dev
```

### Build

For more details about build, check [here](https://tauri.app/v1/guides/building).

```bash
pnpm t build
```
