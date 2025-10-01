# WhatsappGPT

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

## Installation

### Portable Version (Recommended)

1. Download the latest release from the [Releases](https://github.com/Asiwe/WhatsappGPT/releases) page
2. Extract the `WhatsappGPT-Portable` folder anywhere on your computer
3. Double-click `Start-WhatsappGPT.vbs` to launch the application

### System Requirements

- Windows 10 or later
- Internet connection (for WhatsApp Web)
- No additional dependencies required

## Usage

1. **Launch the application** using `Start-WhatsappGPT.vbs` or `WhatsappGPT.bat`
2. **Scan the QR code** with your phone's WhatsApp app
3. **Start chatting** - enjoy the native desktop experience!

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
├── src/                    # Frontend source code
│   ├── index.html
│   ├── main.js
│   └── styles.css
├── src-tauri/             # Tauri backend
│   ├── src/
│   ├── Cargo.toml
│   └── tauri.conf.json
├── WhatsappGPT-Portable/  # Portable distribution
│   ├── whatsappgpt.exe
│   ├── Start-WhatsappGPT.vbs
│   └── icons/
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