/**
 * Icon Manager - Helper functions for accessing bundled icons at runtime
 */

class IconManager {
  constructor() {
    this.invoke = null;
    this.iconCache = new Map();
    this.availableIcons = null;
    
    // Initialize Tauri invoke function
    this.initInvoke();
  }
  
  initInvoke() {
    // Support both Tauri v1 and v2
    if (window.__TAURI__?.core?.invoke) {
      this.invoke = window.__TAURI__.core.invoke;
    } else if (window.__TAURI__?.invoke) {
      this.invoke = window.__TAURI__.invoke;
    } else {
      console.warn('Tauri invoke not available - icon manager will not work');
    }
  }
  
  /**
   * Get the file path for a specific icon
   * @param {string} iconName - Name of the icon file (e.g., "icon.ico", "badges/1.ico")
   * @returns {Promise<string>} - Full path to the icon file
   */
  async getIconPath(iconName) {
    if (!this.invoke) {
      throw new Error('Tauri invoke not available');
    }
    
    // Check cache first
    if (this.iconCache.has(iconName)) {
      return this.iconCache.get(iconName);
    }
    
    try {
      const path = await this.invoke('get_icon_path', { iconName });
      this.iconCache.set(iconName, path);
      return path;
    } catch (error) {
      throw new Error(`Icon not found: ${iconName} - ${error}`);
    }
  }
  
  /**
   * Get all available icons in the bundle
   * @returns {Promise<string[]>} - Array of available icon names
   */
  async listAvailableIcons() {
    if (!this.invoke) {
      throw new Error('Tauri invoke not available');
    }
    
    // Return cached result if available
    if (this.availableIcons) {
      return this.availableIcons;
    }
    
    try {
      this.availableIcons = await this.invoke('list_available_icons');
      return this.availableIcons;
    } catch (error) {
      throw new Error(`Failed to list icons: ${error}`);
    }
  }
  
  /**
   * Get the path for a badge icon based on count
   * @param {number} count - Badge count (1-9, 10+ shows "9+")
   * @returns {Promise<string>} - Path to the badge icon
   */
  async getBadgeIconPath(count) {
    if (!this.invoke) {
      throw new Error('Tauri invoke not available');
    }
    
    if (count <= 0) {
      throw new Error('Badge count must be greater than 0');
    }
    
    const cacheKey = `badge_${count}`;
    if (this.iconCache.has(cacheKey)) {
      return this.iconCache.get(cacheKey);
    }
    
    try {
      const path = await this.invoke('get_badge_icon_path', { count });
      this.iconCache.set(cacheKey, path);
      return path;
    } catch (error) {
      throw new Error(`Badge icon not found for count ${count}: ${error}`);
    }
  }
  
  /**
   * Create an HTML img element with the specified icon
   * @param {string} iconName - Name of the icon file
   * @param {Object} options - Optional attributes for the img element
   * @returns {Promise<HTMLImageElement>} - Image element with the icon
   */
  async createIconElement(iconName, options = {}) {
    try {
      const iconPath = await this.getIconPath(iconName);
      const img = document.createElement('img');
      
      // Convert file path to file:// URL for web context
      img.src = `file://${iconPath}`;
      
      // Apply options as attributes
      Object.entries(options).forEach(([key, value]) => {
        img.setAttribute(key, value);
      });
      
      return img;
    } catch (error) {
      throw new Error(`Failed to create icon element: ${error}`);
    }
  }
  
  /**
   * Create a badge icon element
   * @param {number} count - Badge count
   * @param {Object} options - Optional attributes for the img element
   * @returns {Promise<HTMLImageElement>} - Badge image element
   */
  async createBadgeElement(count, options = {}) {
    try {
      const iconPath = await this.getBadgeIconPath(count);
      const img = document.createElement('img');
      
      img.src = `file://${iconPath}`;
      img.alt = `Badge ${count}`;
      img.title = `${count} notification${count !== 1 ? 's' : ''}`;
      
      // Apply default badge styling
      const defaultOptions = {
        width: '16',
        height: '16',
        style: 'display: inline-block; vertical-align: middle;'
      };
      
      Object.entries({ ...defaultOptions, ...options }).forEach(([key, value]) => {
        img.setAttribute(key, value);
      });
      
      return img;
    } catch (error) {
      throw new Error(`Failed to create badge element: ${error}`);
    }
  }
  
  /**
   * Clear the icon cache
   */
  clearCache() {
    this.iconCache.clear();
    this.availableIcons = null;
  }
  
  /**
   * Preload commonly used icons
   * @param {string[]} iconNames - Array of icon names to preload
   */
  async preloadIcons(iconNames = []) {
    const defaultIcons = [
      'icon.ico',
      'icon.png',
      'badges/1.ico',
      'badges/2.ico',
      'badges/3.ico',
      'badges/9+.ico'
    ];
    
    const iconsToLoad = [...defaultIcons, ...iconNames];
    
    const loadPromises = iconsToLoad.map(async (iconName) => {
      try {
        await this.getIconPath(iconName);
      } catch (error) {
        // Ignore errors for missing icons during preload
        console.debug(`Preload: Icon not found - ${iconName}`);
      }
    });
    
    await Promise.allSettled(loadPromises);
  }
}

// Create global instance
window.iconManager = new IconManager();

// Export for module systems
if (typeof module !== 'undefined' && module.exports) {
  module.exports = IconManager;
}

// Example usage:
/*
// Get an icon path
const iconPath = await iconManager.getIconPath('icon.ico');

// List all available icons
const icons = await iconManager.listAvailableIcons();
console.log('Available icons:', icons);

// Create a badge element
const badgeElement = await iconManager.createBadgeElement(5, { 
  width: '20', 
  height: '20' 
});
document.body.appendChild(badgeElement);

// Get badge icon for taskbar
const badgePath = await iconManager.getBadgeIconPath(3);

// Preload commonly used icons
await iconManager.preloadIcons(['icon.png', 'badges/5.ico']);
*/
