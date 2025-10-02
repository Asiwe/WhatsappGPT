// WhatsappGPT Main Application
const { invoke } = window.__TAURI__?.core || window.__TAURI__;

// Initialize the application
window.addEventListener("DOMContentLoaded", async () => {
  console.log("WhatsappGPT starting...");
  
  try {
    // Initialize icon manager and preload common icons
    if (window.iconManager) {
      await window.iconManager.preloadIcons();
      console.log("Icons preloaded successfully");
      
      // List available icons for debugging
      const availableIcons = await window.iconManager.listAvailableIcons();
      console.log("Available icons:", availableIcons);
      
      // Example: Create a badge element and add it to the page
      try {
        const badgeElement = await window.iconManager.createBadgeElement(3, {
          width: '24',
          height: '24',
          style: 'margin: 10px; border: 1px solid #ccc; border-radius: 4px;'
        });
        
        const appDiv = document.getElementById('app');
        if (appDiv) {
          const badgeContainer = document.createElement('div');
          badgeContainer.innerHTML = '<p>Example badge icon:</p>';
          badgeContainer.appendChild(badgeElement);
          appDiv.appendChild(badgeContainer);
        }
      } catch (error) {
        console.warn("Could not create badge element:", error);
      }
    }
    
    // Set up badge testing buttons
    createBadgeTestControls();
    
  } catch (error) {
    console.error("Error initializing WhatsappGPT:", error);
  }
});

// Create test controls for badge functionality
function createBadgeTestControls() {
  const appDiv = document.getElementById('app');
  if (!appDiv) return;
  
  const controlsDiv = document.createElement('div');
  controlsDiv.innerHTML = `
    <div style="margin: 20px 0; padding: 20px; border: 1px solid #ddd; border-radius: 8px;">
      <h3>Badge Test Controls</h3>
      <p>Test the taskbar badge functionality:</p>
      <div style="margin: 10px 0;">
        <button onclick="testSetBadge(1)">Badge 1</button>
        <button onclick="testSetBadge(3)">Badge 3</button>
        <button onclick="testSetBadge(5)">Badge 5</button>
        <button onclick="testSetBadge(9)">Badge 9</button>
        <button onclick="testSetBadge(15)">Badge 15 (9+)</button>
        <button onclick="testSetBadge(0)">Clear Badge</button>
      </div>
      <div id="badge-status" style="margin-top: 10px; font-style: italic; color: #666;"></div>
    </div>
  `;
  
  appDiv.appendChild(controlsDiv);
}

// Test badge setting
async function testSetBadge(count) {
  const statusDiv = document.getElementById('badge-status');
  
  try {
    if (invoke) {
      await invoke('set_badge', { count });
      statusDiv.textContent = count === 0 ? 'Badge cleared' : `Badge set to ${count}`;
      statusDiv.style.color = '#0a7c0a';
    } else {
      statusDiv.textContent = 'Tauri invoke not available';
      statusDiv.style.color = '#c41e3a';
    }
  } catch (error) {
    statusDiv.textContent = `Error: ${error}`;
    statusDiv.style.color = '#c41e3a';
    console.error('Badge error:', error);
  }
}

// Make testSetBadge available globally
window.testSetBadge = testSetBadge;

// Example function to demonstrate icon path retrieval
async function demonstrateIconAccess() {
  if (!window.iconManager) {
    console.log("Icon manager not available");
    return;
  }
  
  try {
    // Get main icon path
    const mainIconPath = await window.iconManager.getIconPath('icon.ico');
    console.log("Main icon path:", mainIconPath);
    
    // Get badge icon path
    const badgeIconPath = await window.iconManager.getBadgeIconPath(5);
    console.log("Badge icon path:", badgeIconPath);
    
    // List all available icons
    const allIcons = await window.iconManager.listAvailableIcons();
    console.log("All available icons:", allIcons);
    
  } catch (error) {
    console.error("Error accessing icons:", error);
  }
}

// Run demonstration
setTimeout(demonstrateIconAccess, 1000);
