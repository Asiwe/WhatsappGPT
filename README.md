# WhatsappGPT

[![Download](https://img.shields.io/badge/Download-Latest%20Release-blue?style=for-the-badge&logo=download)](./releases/WhatsappGPT_1.0.0_x64-setup.exe)
[![Version](https://img.shields.io/badge/Version-1.0.0-green?style=for-the-badge)](./releases/)
[![Platform](https://img.shields.io/badge/Platform-Windows-lightgrey?style=for-the-badge&logo=windows)](./releases/)

A Tauri-based desktop application that provides a native wrapper for WhatsApp Web with enhanced features and a clean user interface.

## Features

- 🚀 **Native Desktop App** - Built with Tauri for optimal performance
- 💻 **WhatsApp Web Integration** - Access WhatsApp directly from your desktop
- 🎨 **Clean Interface** - Modern, responsive design
- 🔒 **Secure** - No data collection, your messages stay private
- ⚡ **Fast** - Lightweight and efficient
- 🖥️ **Cross-Platform** - Works on Windows, macOS, and Linux

## Screenshots

The application provides a clean, native desktop experience for WhatsApp Web.

## 📥 Quick Download & Install

### For End Users (Recommended)

**[🚀 Download Latest Release - WhatsappGPT v1.0.0](./releases/WhatsappGPT_1.0.0_x64-setup.exe)**

1. **Download** the setup file above
2. **Run** the installer and follow the wizard
3. **Launch** WhatsappGPT from Start Menu or Desktop
4. **Scan** QR code with your phone's WhatsApp
5. **Start chatting!**

> 💡 **Quick Access**: All releases are available in the [`/releases`](./releases/) folder

### System Requirements
- Windows 10/11 (64-bit)
- 4GB RAM (8GB recommended)
- 100MB free storage
- Internet connection

---

## 🛠️ Development Setup

### For Developers

1. Clone the repository:
   ```bash
   git clone https://github.com/Asiwe/WhatsappGPT.git
   cd WhatsappGPT
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

### Building

To build the application:

```bash
npm run tauri build
```

The built application will be in `src-tauri/target/release/`.

### System Requirements

- [Rust](https://rustup.rs/) (latest stable version)
- [Node.js](https://nodejs.org/) (v16 or later)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)
- Internet connection (for WhatsApp Web)

## Usage

1. **Run the development version** using `npm run tauri dev`
2. **Or build and run** the compiled executable
3. **Scan the QR code** with your phone's WhatsApp app
4. **Start chatting** - enjoy the native desktop experience!

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Node.js](https://nodejs.org/) (v16 or later)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/Asiwe/WhatsappGPT.git
   cd WhatsappGPT
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

### Building

To build the application:

```bash
npm run tauri build
```

The built application will be in `src-tauri/target/release/`.

## Project Structure

```
WhatsappGPT/
├── releases/              # 📦 Ready-to-install executables
│   ├── WhatsappGPT_1.0.0_x64-setup.exe
│   └── README.md
├── src/                   # Frontend source code
│   ├── index.html
│   ├── main.js
│   ├── styles.css
│   └── icon-manager.js
├── src-tauri/            # Tauri backend
│   ├── src/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── target/release/   # Build output
└── README.md
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/)
- Uses [WhatsApp Web](https://web.whatsapp.com/)
- Icons and assets are included for desktop integration

## Support

If you encounter any issues or have questions, please open an issue on GitHub.

---

**Note**: This application is not affiliated with WhatsApp or Meta. It's an unofficial desktop wrapper for WhatsApp Web.