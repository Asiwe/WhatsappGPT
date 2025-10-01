#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[cfg(windows)]
use std::{path::PathBuf, os::windows::ffi::OsStrExt};
#[cfg(windows)]
use windows::core::PCWSTR;
#[cfg(windows)]
use windows::Win32::Foundation::{HWND, WPARAM, LPARAM};
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
  LoadImageW, SendMessageW, HICON, ICON_SMALL, ICON_BIG, IMAGE_ICON, LR_DEFAULTSIZE, LR_LOADFROMFILE, WM_SETICON,
};
#[cfg(windows)]
use windows::Win32::UI::Shell::{
  ITaskbarList3, TBPF_NOPROGRESS
};
#[cfg(windows)]
use windows::Win32::System::Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_INPROC_SERVER};
#[cfg(windows)]
use windows::core::GUID;

#[tauri::command]
#[cfg(windows)]
fn set_badge(window: tauri::WebviewWindow, count: u32) -> Result<(), String> {
  let hwnd: HWND = window.hwnd().map_err(|_| "hwnd not available")?;
  unsafe { set_taskbar_badge(hwnd, count); }
  Ok(())
}

#[tauri::command]
#[cfg(not(windows))]
fn set_badge(_window: tauri::WebviewWindow, _count: u32) -> Result<(), String> {
  Ok(())
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let win = app.get_webview_window("main").expect("main window");

      // Set title bar icon (small icon - always fixed)
      #[cfg(windows)]
      unsafe {
        if let Some(hicon) = load_icon_any(&["icons/icon_titlebar.ico", "resources/icons/icon_titlebar.ico"]) {
          let hwnd = win.hwnd().expect("hwnd");
          let _ = SendMessageW(
            hwnd,
            WM_SETICON,
            Some(WPARAM(ICON_SMALL as usize)),
            Some(LPARAM(hicon.0 as isize)),
          );
        }
      }

      // UI: floating button + panel + titlebar alt stroke + BADGE=read from title
      let inject = r#"
        (function () {
          if (window.__WGPT_OVERLAY_V3__) return;
          window.__WGPT_OVERLAY_V3__ = true;

          const run = () => {
            const ls = window.localStorage;
            const savedW      = parseInt(ls.getItem('wgptWidth')  || '320', 10);
            const savedBg     = ls.getItem('wgptBg')     || '#ffffff';
            const savedStroke = ls.getItem('wgptStroke') || '#bdbdbd';
            const savedOn     = ls.getItem('wgptOpen')   !== '0';

            // Tauri v1/v2 invoke
            const getInvoke = () => (window.__TAURI__?.core?.invoke) || (window.__TAURI__?.invoke);
            const invoke = (cmd, args) => {
              const fn = getInvoke();
              return fn ? fn(cmd, args) : Promise.reject('no tauri invoke');
            };

            const style = document.createElement('style');
            style.textContent = `
              :root{
                --wgpt-width:${savedW}px;
                --wgpt-bg:${savedBg};
                --wgpt-stroke:${savedStroke};
              }
              /* Title bar altına 1px stroke */
              html::before{
                content:"";
                position: fixed;
                top: 0; left: 0; right: 0;
                height: 1px;
                background: #e1e3e1;
                z-index: 2147483647;
                pointer-events: none;
              }
              /* Sol üstte havada süzülen tuş (kare; hover'da yuvarlak) */
              #wgpt-fab{
                position:fixed; top:187px; left:11px; z-index:2147483647;
                width:42px; height:42px; border-radius:0;
                background: transparent; border: 0;
                display:flex; align-items:center; justify-content:center;
                cursor:pointer; box-shadow: none;
                transition: background-color 120ms ease, border-radius 120ms ease;
                padding: 0; margin: 0;
              }
              #wgpt-fab:hover{
                background:#eae9e7;
                border-radius:9999px;
              }
              #wgpt-fab svg{ width:23px; height:23px; color:#636261 }

              /* Sol overlay panel: sitenin ÜZERİNDE, sayfayı etkilemez */
              #wgpt-panel{
                position:fixed; top:0; left:0; bottom:0;
                width:var(--wgpt-width);
                background:var(--wgpt-bg);
                z-index:2147483646;
                border-right:2px solid var(--wgpt-stroke);
                display:${savedOn ? 'block' : 'none'};
              }
              #wgpt-grip{
                position:absolute; top:0; right:-4px; bottom:0; width:8px;
                cursor:ew-resize;
              }
              #wgpt-close{
                position:absolute; top:8px; right:8px; z-index:2147483647;
                width:37px; height:37px; border-radius:0;
                background: transparent; border: 0;
                display:flex; align-items:center; justify-content:center;
                cursor:pointer; box-shadow: none;
                transition: background-color 120ms ease, border-radius 120ms ease, right 120ms ease;
                padding: 0; margin: 0;
              }
              
              /* Center close button when overlay is at minimum width */
              #wgpt-panel.min-width #wgpt-close {
                right: calc(50% - 18.5px);
              }
              #wgpt-close:hover{
                background:#eae9e7;
                border-radius:9999px;
              }
              #wgpt-close svg{ width:20px; height:20px; color:#636261 }
              
              /* Custom tooltip styling - WhatsApp Web style */
              [title] {
                position: relative;
              }
              
              #wgpt-fab:hover::after {
                content: 'Overlay';
                position: absolute;
                top: 50%;
                left: calc(100% + 8px);
                transform: translateY(-50%);
                background: #1C1C1C;
                color: #FFFFFF;
                font-family: "Segoe UI", "Helvetica Neue", Helvetica, Arial, sans-serif;
                font-size: 12px;
                font-weight: 400;
                line-height: 1.2;
                padding: 6px 8px;
                border-radius: 6px;
                white-space: nowrap;
                max-width: 240px;
                white-space: normal;
                word-wrap: break-word;
                box-shadow: 0 2px 8px rgba(0,0,0,0.25);
                z-index: 1000;
                pointer-events: none;
                opacity: 0;
                transform: translateY(-50%) translateX(2px) scale(0.98);
                transition: opacity 120ms cubic-bezier(0.2,0,0,1), 
                           transform 120ms cubic-bezier(0.2,0,0,1);
                animation: tooltipFadeIn 120ms cubic-bezier(0.2,0,0,1) forwards;
                -webkit-font-smoothing: antialiased;
                -moz-osx-font-smoothing: grayscale;
                text-rendering: optimizeLegibility;
              }
              
              @keyframes tooltipFadeIn {
                from {
                  opacity: 0;
                  transform: translateY(-50%) translateX(2px) scale(0.98);
                }
                to {
                  opacity: 1;
                  transform: translateY(-50%) translateX(0) scale(1);
                }
              }
              
              @media (prefers-reduced-motion: reduce) {
                #wgpt-fab:hover::after {
                  animation: none;
                  transition: opacity 120ms ease;
                }
                #wgpt-fab:hover::after {
                  transform: translateY(-50%);
                }
              }
            `;
            document.documentElement.appendChild(style);

            // Floating tuş
            const fab = document.createElement('button');
            fab.id = 'wgpt-fab';
            fab.innerHTML = `
              <svg viewBox="0 0 24 24" aria-hidden="true">
                <rect x="3" y="4" width="18" height="16" rx="4" ry="4" fill="none" stroke="currentColor" stroke-width="2"/>
                <line x1="12" y1="4" x2="12" y2="20" stroke="currentColor" stroke-width="2"/>
              </svg>
            `;
            fab.style.display = savedOn ? 'none' : 'flex';
            document.documentElement.appendChild(fab);

            // Panel
            const panel = document.createElement('div');
            panel.id = 'wgpt-panel';
            const grip = document.createElement('div');
            grip.id = 'wgpt-grip';
            panel.appendChild(grip);
            
            // Close button
            const closeBtn = document.createElement('button');
            closeBtn.id = 'wgpt-close';
            closeBtn.innerHTML = `
              <svg viewBox="0 0 24 24" aria-hidden="true">
                <line x1="18" y1="6" x2="6" y2="18" stroke="currentColor" stroke-width="2"/>
                <line x1="6" y1="6" x2="18" y2="18" stroke="currentColor" stroke-width="2"/>
              </svg>
            `;
            panel.appendChild(closeBtn);
            document.documentElement.appendChild(panel);
            
            // Check initial width for close button centering
            if (savedW <= 70) {
              panel.classList.add('min-width');
            }

            const toggle = () => {
              const isOpen = panel.style.display !== 'none';
              panel.style.display = isOpen ? 'none' : 'block';
              fab.style.display = isOpen ? 'flex' : 'none';
              try { ls.setItem('wgptOpen', !isOpen ? '1' : '0'); } catch(_){}
            };
            fab.addEventListener('click', toggle);
            closeBtn.addEventListener('click', toggle);
            window.addEventListener('keydown', (e) => {
              if ((e.ctrlKey || e.metaKey) && e.altKey && (e.key === 'w' || e.key === 'W')) {
                e.preventDefault(); toggle();
              }
            });

            // Sürükleyerek genişlik
            let down=false, startX=0, startW=savedW;
            const onMove=(e)=>{ if(!down) return;
              const newWidth = startW + (e.screenX - startX);
              const maxWidth = window.innerWidth - 20; // Leave 20px margin from window edge
              const w = Math.max(70, Math.min(newWidth, maxWidth));
              panel.style.setProperty('--wgpt-width', w + 'px');
              // Add/remove min-width class for close button centering
              if (w <= 70) {
                panel.classList.add('min-width');
              } else {
                panel.classList.remove('min-width');
              }
              try{ ls.setItem('wgptWidth', String(Math.round(w))); }catch(_){}
            };
            const onUp=()=>{ if(!down) return;
              down=false; document.body.style.userSelect='';
              window.removeEventListener('mousemove', onMove);
              window.removeEventListener('mouseup', onUp);
            };
            grip.addEventListener('mousedown',(e)=>{
              down=true; startX=e.screenX;
              startW=panel.getBoundingClientRect().width || savedW;
              document.body.style.userSelect='none';
              window.addEventListener('mousemove', onMove);
              window.addEventListener('mouseup', onUp);
            });

            // ====== BADGE: document.title içinden sayacı çek ======
            function countFromTitle(){
              try{
                const t = document.title || "";
                const m = t.match(/\((\d+)\)/);
                return m ? parseInt(m[1], 10) : 0;
              }catch(_){ return 0; }
            }

            let last = -1;
            const push = () => {
              const n = countFromTitle();
              if (n !== last) {
                last = n;
                invoke('set_badge', { count: n }).catch(()=>{});
              }
            };

            // <title> değişimini dinle + emniyet aralığı
            const titleEl = document.querySelector('title');
            if (titleEl) {
              const mo = new MutationObserver(() => { push(); });
              mo.observe(titleEl, { childList: true, characterData: true, subtree: true });
            }
            setInterval(push, 1500);
            push();
          };

          if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', run);
          else run();
        })();
      "#;

      let _ = win.eval(inject);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![set_badge])
    .run(tauri::generate_context!())
    .expect("error while running WhatsappGPT");
}

#[cfg(windows)]
unsafe fn load_icon_any(paths: &[&str]) -> Option<HICON> {
  for rel in paths {
    let p = PathBuf::from(rel);
    if p.exists() {
      let wide: Vec<u16> = p.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
      if let Ok(h) = LoadImageW(None, PCWSTR(wide.as_ptr()), IMAGE_ICON, 0, 0, LR_LOADFROMFILE | LR_DEFAULTSIZE) {
        return Some(HICON(h.0));
      }
    }
  }
  if let Ok(exe) = std::env::current_exe() {
    if let Some(dir) = exe.parent() {
      for alt in paths {
        let p = dir.join(alt);
        if p.exists() {
          let wide: Vec<u16> = p.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
          if let Ok(h) = LoadImageW(None, PCWSTR(wide.as_ptr()), IMAGE_ICON, 0, 0, LR_LOADFROMFILE | LR_DEFAULTSIZE) {
            return Some(HICON(h.0));
          }
        }
      }
    }
  }
  None
}

/// Windows native badge system - Uses pre-made badge icons with overlay
#[cfg(windows)]
unsafe fn set_taskbar_badge(hwnd: HWND, count: u32) {
  // COM'u başlat
  if CoInitialize(None).is_ok() {
    // TaskbarList3 interface'ini al
    let clsid = GUID::from_u128(0x56FDF344_FD6D_11d0_958A_006097C9A090);
    
    if let Ok(taskbar_list) = CoCreateInstance::<Option<&windows::core::IUnknown>, ITaskbarList3>(&clsid, None, CLSCTX_INPROC_SERVER) {
      // Progress bar'ı temizle
      let _ = taskbar_list.SetProgressState(hwnd, TBPF_NOPROGRESS);
      
      // Badge count'u ayarla (0 = badge yok, 1-9 = sayı, 10+ = 9+)
      let badge_count = if count == 0 { 0 } else { count.min(9) };
      
      if badge_count > 0 {
        // Badge icon'u yükle (1-9 için sayı, 10+ için 9+)
        let badge_path = if count > 9 {
          format!("icons/badges/9+.ico")
        } else {
          format!("icons/badges/{}.ico", badge_count)
        };
        
        if let Some(badge_icon) = load_icon_any(&[&badge_path, &format!("resources/{}", badge_path)]) {
          // Badge icon'u taskbar'a uygula (overlay)
          let _ = taskbar_list.SetOverlayIcon(hwnd, badge_icon, PCWSTR(std::ptr::null()));
        }
      } else {
        // Badge'i temizle
        let _ = taskbar_list.SetOverlayIcon(hwnd, HICON(std::ptr::null_mut()), PCWSTR(std::ptr::null()));
      }
    }
    
    // COM'u temizle
    CoUninitialize();
  }
}