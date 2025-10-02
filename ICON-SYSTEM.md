# WhatsappGPT Icon System

This document explains how the icon bundling and runtime access system works in WhatsappGPT.

## Overview

The icon system allows you to:
- Bundle all icon files with the application
- Access icons at runtime from JavaScript
- Use icons for taskbar badges, UI elements, and system integration
- Automatically discover available icons

## Configuration

### Tauri Configuration (`src-tauri/tauri.conf.json`)

```json
{
  "bundle": {
    "resources": [
      "icons/**/*",
      "icons/badges/**/*"
    ]
  }
}
```

This configuration ensures all icons in the `icons/` directory and `icons/badges/` subdirectory are included in the production build.

## Icon Structure

```
src-tauri/icons/
├── icon.ico                 # Main application icon
├── icon.png                 # PNG version of main icon
├── icon.icns                # macOS version
├── icon-titlebar.png        # Title bar icon
├── badges/                  # Badge icons for notifications
│   ├── 1.ico
│   ├── 2.ico
│   ├── 3.ico
│   ├── ...
│   ├── 9.ico
│   └── 9+.ico              # For counts > 9
├── android/                 # Android icons
├── ios/                     # iOS icons
└── ...                      # Other platform-specific icons
```

## Rust API

### Commands

#### `get_icon_path(icon_name: String) -> Result<String, String>`
Returns the full file system path to the specified icon.

```rust
// Example usage in Rust
let path = get_icon_path("icon.ico".to_string())?;
```

#### `list_available_icons() -> Result<Vec<String>, String>`
Returns a list of all available icon files in the bundle.

#### `get_badge_icon_path(count: u32) -> Result<String, String>`
Returns the path to the appropriate badge icon for the given count.
- Count 1-9: Returns `badges/{count}.ico`
- Count 10+: Returns `badges/9+.ico`

### Icon Loading Function

```rust
unsafe fn load_icon_any(paths: &[&str]) -> Option<HICON>
```

This function tries to load an icon from multiple possible paths:
1. Current directory relative paths
2. Executable directory relative paths
3. Resource directory paths

## JavaScript API

### IconManager Class

The `IconManager` class provides a convenient JavaScript interface for accessing icons.

#### Methods

##### `getIconPath(iconName: string): Promise<string>`
```javascript
const iconPath = await iconManager.getIconPath('icon.ico');
```

##### `listAvailableIcons(): Promise<string[]>`
```javascript
const icons = await iconManager.listAvailableIcons();
console.log('Available icons:', icons);
```

##### `getBadgeIconPath(count: number): Promise<string>`
```javascript
const badgePath = await iconManager.getBadgeIconPath(5);
```

##### `createIconElement(iconName: string, options?: object): Promise<HTMLImageElement>`
```javascript
const img = await iconManager.createIconElement('icon.png', {
  width: '32',
  height: '32',
  alt: 'App Icon'
});
document.body.appendChild(img);
```

##### `createBadgeElement(count: number, options?: object): Promise<HTMLImageElement>`
```javascript
const badge = await iconManager.createBadgeElement(3, {
  width: '16',
  height: '16'
});
document.body.appendChild(badge);
```

##### `preloadIcons(iconNames?: string[]): Promise<void>`
```javascript
// Preload common icons
await iconManager.preloadIcons(['icon.ico', 'badges/1.ico', 'badges/5.ico']);
```

### Usage Examples

#### Basic Icon Access
```javascript
// Get an icon path
try {
  const iconPath = await iconManager.getIconPath('icon.ico');
  console.log('Icon located at:', iconPath);
} catch (error) {
  console.error('Icon not found:', error);
}
```

#### Creating UI Elements
```javascript
// Create an icon element for the UI
const iconElement = await iconManager.createIconElement('icon.png', {
  width: '24',
  height: '24',
  class: 'app-icon'
});

// Add to DOM
document.getElementById('header').appendChild(iconElement);
```

#### Badge System Integration
```javascript
// Set taskbar badge
await invoke('set_badge', { count: 5 });

// Create badge UI element
const badgeElement = await iconManager.createBadgeElement(5);
document.getElementById('notification-area').appendChild(badgeElement);
```

#### Listing Available Icons
```javascript
// Get all available icons
const availableIcons = await iconManager.listAvailableIcons();

// Filter for badge icons
const badgeIcons = availableIcons.filter(icon => icon.startsWith('badges/'));
console.log('Badge icons:', badgeIcons);
```

## Runtime Behavior

### Development Mode
In development mode (`npm run tauri dev`), icons are loaded from the `src-tauri/icons/` directory.

### Production Mode
In production builds (`npm run tauri build`), icons are bundled into the application and accessed from the resources directory.

### Path Resolution
The system tries multiple paths in order:
1. `icons/{iconName}`
2. `icons/badges/{iconName}`
3. `resources/icons/{iconName}`
4. `resources/icons/badges/{iconName}`

### Caching
The JavaScript `IconManager` caches icon paths to improve performance on repeated access.

## Badge System

The badge system uses numbered icon files (1.ico through 9.ico and 9+.ico) to display notification counts on the Windows taskbar.

### Badge Icons
- `badges/1.ico` through `badges/9.ico`: Individual count badges
- `badges/9+.ico`: Used for counts of 10 or higher

### Usage
```javascript
// Set badge count (updates taskbar)
await invoke('set_badge', { count: 3 });

// Clear badge
await invoke('set_badge', { count: 0 });

// Get badge icon path for UI
const badgePath = await iconManager.getBadgeIconPath(7);
```

## Error Handling

All icon operations include proper error handling:

```javascript
try {
  const iconPath = await iconManager.getIconPath('nonexistent.ico');
} catch (error) {
  console.error('Icon not found:', error.message);
  // Fallback to default icon or show placeholder
}
```

## Best Practices

1. **Preload Common Icons**: Use `preloadIcons()` to cache frequently used icons
2. **Error Handling**: Always wrap icon operations in try-catch blocks
3. **Fallbacks**: Provide fallback icons or placeholders for missing icons
4. **Caching**: The system automatically caches paths, but you can clear cache if needed
5. **File Formats**: Use `.ico` for Windows, `.png` for cross-platform compatibility

## Troubleshooting

### Icons Not Found
- Verify icons exist in `src-tauri/icons/` directory
- Check that `tauri.conf.json` includes the correct resource paths
- Ensure icons are included in the build (check bundle configuration)

### Path Issues
- Icons are accessed using forward slashes (`/`) even on Windows
- Paths are relative to the icons directory
- Use `badges/1.ico` not `badges\1.ico`

### Performance
- Use `preloadIcons()` for better performance
- Cache is automatically managed but can be cleared with `clearCache()`
- Consider icon file sizes for faster loading
