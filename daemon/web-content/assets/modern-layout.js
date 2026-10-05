// GoXLR Modern Layout Architecture & Unified Settings System
// Obsidian-Violet Design System

(function() {
  'use strict';

  let lastActiveTabId = 'mixer';

  function initModernShell() {
    const mainEl = document.getElementById('main');
    if (!mainEl || document.querySelector('.modern-header-bar')) {
      return;
    }

    // 1. Create Clean Top Header Bar
    const headerBar = document.createElement('header');
    headerBar.className = 'modern-header-bar';
    headerBar.innerHTML = `
      <div class="modern-header-left">
        <div class="modern-brand-logo">GO<span>XLR</span></div>
        <div class="modern-status-badge" id="modern-status-badge" title="Device Connection Status">
          <span class="modern-status-dot"></span>
          <span id="modern-status-text">Connected</span>
        </div>
        <div class="modern-profile-bar" id="modern-profile-bar">
          <span class="profile-label">Profile:</span>
          <select id="modern-profile-select" class="modern-select" title="Switch active profile"></select>
          <button class="modern-mini-btn" id="modern-profile-save" title="Save current profile">
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
              <polyline points="17 21 17 13 7 13 7 21"></polyline>
              <polyline points="7 3 7 8 15 8"></polyline>
            </svg>
          </button>
          <button class="modern-mini-btn" id="modern-profile-folder" title="Open Profiles Folder">
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
            </svg>
          </button>
        </div>
        <div class="modern-profile-bar" id="modern-mic-profile-bar">
          <span class="profile-label">Mic Profile:</span>
          <select id="modern-mic-profile-select" class="modern-select" title="Switch active mic profile"></select>
          <button class="modern-mini-btn" id="modern-mic-profile-save" title="Save current mic profile">
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
              <polyline points="17 21 17 13 7 13 7 21"></polyline>
              <polyline points="7 3 7 8 15 8"></polyline>
            </svg>
          </button>
          <button class="modern-mini-btn" id="modern-mic-profile-folder" title="Open Mic Profiles Folder">
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
            </svg>
          </button>
        </div>
      </div>
      <div class="modern-header-right">
        <span class="modern-version-tag" id="modern-version-tag">GoXLR Utility</span>
        <button class="modern-header-btn" id="modern-btn-vis-toggle" title="Toggle Mixer Visualiser">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
            <line x1="8" y1="21" x2="16" y2="21"></line>
            <line x1="12" y1="17" x2="12" y2="21"></line>
          </svg>
        </button>
        <button class="modern-header-btn" id="modern-btn-settings" title="System Settings">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3"></circle>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
          </svg>
        </button>
      </div>
    `;

    mainEl.insertBefore(headerBar, mainEl.firstChild);

    // Track active tabs to restore when settings close
    setupTabTracker();

    // 2. Wire Profile Dropdown, Save, & Open Folder
    initProfileControls();

    // 3. Wire Visualiser Toggle Button
    const visBtn = headerBar.querySelector('#modern-btn-vis-toggle');
    if (visBtn) {
      visBtn.addEventListener('click', () => {
        const vis = document.getElementById('goxlr-visualiser');
        if (vis) {
          vis.classList.toggle('vis-hidden');
          const isHidden = vis.classList.contains('vis-hidden');
          visBtn.classList.toggle('active', !isHidden);
          if (vis.parentElement) {
            vis.parentElement.classList.toggle('vis-hidden', isHidden);
          }
        }
      });
    }

    // 4. Create Unified Settings Modal
    createUnifiedSettingsModal();

    const topSettingsBtn = headerBar.querySelector('#modern-btn-settings');
    if (topSettingsBtn) {
      topSettingsBtn.addEventListener('click', () => {
        openUnifiedSettings();
      });
    }

    // 5. Setup Routing Table Crosshair & Observer
    setupRoutingTableObserver();

    // 6. Setup Quick Mic Gain Slider outside popup
    setupMicGainObserver();

    // Hide old Mic Profiles panel from body
    setupMicProfileObserver();

    // 7. Observe System Tab to Unify Settings Buttons
    setupSystemSettingsObserver();

    setupVisualizerHeightObserver();
    enforceDefaultWindowSize();

    // 8. Compact rename: "Channel X" -> "Ch X" in lighting mixer fader buttons
    setupCompactRenameObserver();

    // 9. Default to 'Mic' tab on startup instead of 'Mixer'
    setDefaultTabToMic();
  }

  function setupTabTracker() {
    document.addEventListener('click', (e) => {
      const tabBtn = e.target.closest('.tab button');
      if (tabBtn) {
        const text = (tabBtn.textContent || '').trim().toLowerCase();
        if (text && !text.includes('system')) {
          lastActiveTabId = text;
        }
      }
    });
  }

  // Populate & Sync Profile & Mic Profile Dropdowns and Action Buttons
  function initProfileControls() {
    const select = document.getElementById('modern-profile-select');
    const saveBtn = document.getElementById('modern-profile-save');
    const folderBtn = document.getElementById('modern-profile-folder');

    const micSelect = document.getElementById('modern-mic-profile-select');
    const micSaveBtn = document.getElementById('modern-mic-profile-save');
    const micFolderBtn = document.getElementById('modern-mic-profile-folder');

    const statusDot = document.querySelector('.modern-status-dot');
    const statusText = document.getElementById('modern-status-text');

    function refreshProfiles() {
      if (typeof window.c === 'undefined') return;

      const isConn = window.c.isConnected && window.c.isConnected();
      if (statusText) statusText.textContent = isConn ? 'Connected' : 'Disconnected';
      if (statusDot) {
        statusDot.style.backgroundColor = isConn ? 'var(--status-connected)' : 'var(--status-danger)';
        statusDot.style.boxShadow = isConn ? '0 0 8px var(--status-connected)' : '0 0 8px var(--status-danger)';
      }

      if (!isConn) return;

      try {
        let files = (window.c.getProfileFiles ? window.c.getProfileFiles() : []) || [];
        if (!Array.isArray(files)) {
          files = [];
        }
        const activeDev = window.c.getActiveDevice ? window.c.getActiveDevice() : null;
        const active = activeDev ? activeDev.profile_name : '';

        const cacheKey = files.join('|') + '::' + active;
        if (select && (select._cacheKey !== cacheKey || select.options.length !== files.length)) {
          select._cacheKey = cacheKey;
          select.innerHTML = '';
          files.forEach(f => {
            const opt = document.createElement('option');
            opt.value = f;
            opt.textContent = f;
            if (f === active) {
              opt.selected = true;
            }
            select.appendChild(opt);
          });
        }
        if (select && active && select.value !== active) {
          select.value = active;
        }

        // Mic Profiles
        let micFiles = (window.c.getMicProfileFiles ? window.c.getMicProfileFiles() : []) || [];
        if (!Array.isArray(micFiles)) {
          micFiles = [];
        }
        const activeMic = activeDev ? activeDev.mic_profile_name : '';

        const micCacheKey = micFiles.join('|') + '::' + activeMic;
        if (micSelect && (micSelect._cacheKey !== micCacheKey || micSelect.options.length !== micFiles.length)) {
          micSelect._cacheKey = micCacheKey;
          micSelect.innerHTML = '';
          micFiles.forEach(f => {
            const opt = document.createElement('option');
            opt.value = f;
            opt.textContent = f;
            if (f === activeMic) {
              opt.selected = true;
            }
            micSelect.appendChild(opt);
          });
        }
        if (micSelect && activeMic && micSelect.value !== activeMic) {
          micSelect.value = activeMic;
        }
      } catch (err) {
        console.warn('Profile refresh error:', err);
      }
    }

    if (select) {
      select.addEventListener('change', (e) => {
        const val = e.target.value;
        if (val && window.$ && window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
          window.$.send_command(window.c.getActiveSerial(), { LoadProfile: [val, true] });
        }
      });
    }

    if (saveBtn) {
      saveBtn.addEventListener('click', () => {
        if (window.$ && window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
          window.$.send_command(window.c.getActiveSerial(), { SaveProfile: [] });
          saveBtn.classList.add('save-success');
          setTimeout(() => saveBtn.classList.remove('save-success'), 1200);
        }
      });
    }

    if (folderBtn) {
      folderBtn.addEventListener('click', () => {
        if (window.$ && window.$.open_path) {
          window.$.open_path('Profiles');
        }
      });
    }

    if (micSelect) {
      micSelect.addEventListener('change', (e) => {
        const val = e.target.value;
        if (val && window.$ && window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
          window.$.send_command(window.c.getActiveSerial(), { LoadMicProfile: [val, true] });
        }
      });
    }

    if (micSaveBtn) {
      micSaveBtn.addEventListener('click', () => {
        if (window.$ && window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
          window.$.send_command(window.c.getActiveSerial(), { SaveMicProfile: [] });
          micSaveBtn.classList.add('save-success');
          setTimeout(() => micSaveBtn.classList.remove('save-success'), 1200);
        }
      });
    }

    if (micFolderBtn) {
      micFolderBtn.addEventListener('click', () => {
        if (window.$ && window.$.open_path) {
          window.$.open_path('MicProfiles');
        }
      });
    }

    if (window.c && window.c.onConnected) {
      window.c.onConnected(refreshProfiles);
    }
    setInterval(refreshProfiles, 1000);
    refreshProfiles();
  }

  // Unified Settings Modal
  function createUnifiedSettingsModal() {
    if (document.getElementById('modern-unified-settings-modal')) return;

    const modal = document.createElement('div');
    modal.id = 'modern-unified-settings-modal';
    modal.className = 'modern-settings-overlay';

    modal.innerHTML = `
      <div class="modern-settings-window">
        <div class="modern-settings-header">
          <div class="modern-settings-title-group">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="3"></circle>
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
            </svg>
            <h2>Settings</h2>
          </div>
          <button class="modern-settings-close-btn" id="modern-settings-close" title="Close Settings">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="modern-settings-tabs">
          <button class="modern-settings-tab-btn active" data-pane="device">Device Settings</button>
          <button class="modern-settings-tab-btn" data-pane="utility">Utility Settings</button>
          <button class="modern-settings-tab-btn" data-pane="power">Power Actions</button>
        </div>
        <div class="modern-settings-body">
          <div class="modern-settings-pane active" id="pane-device">
            <div class="pane-loading">Loading Device Settings...</div>
          </div>
          <div class="modern-settings-pane" id="pane-utility">
            <div class="pane-loading">Loading Utility Settings...</div>
          </div>
          <div class="modern-settings-pane" id="pane-power">
            <div class="pane-loading">Loading Power Actions...</div>
          </div>
        </div>
      </div>
    `;

    document.body.appendChild(modal);

    // Tab switching inside modal
    const tabBtns = modal.querySelectorAll('.modern-settings-tab-btn');
    tabBtns.forEach(btn => {
      btn.addEventListener('click', () => {
        tabBtns.forEach(b => b.classList.remove('active'));
        btn.classList.add('active');
        const targetPane = btn.getAttribute('data-pane');
        modal.querySelectorAll('.modern-settings-pane').forEach(p => p.classList.remove('active'));
        const pane = modal.querySelector(`#pane-${targetPane}`);
        if (pane) pane.classList.add('active');
      });
    });

    // Close buttons
    modal.querySelector('#modern-settings-close').addEventListener('click', closeUnifiedSettings);
    modal.addEventListener('click', (e) => {
      if (e.target === modal) {
        closeUnifiedSettings();
      }
    });

    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && modal.classList.contains('open')) {
        closeUnifiedSettings();
      }
    });
  }

  function openUnifiedSettings() {
    const modal = document.getElementById('modern-unified-settings-modal');
    if (!modal) return;
    populateUnifiedSettings();
    modal.classList.add('open');
  }

  function closeUnifiedSettings() {
    const modal = document.getElementById('modern-unified-settings-modal');
    if (modal) {
      modal.classList.remove('open');
    }
  }

  function populateUnifiedSettings() {
    const paneDevice = document.getElementById('pane-device');
    const paneUtility = document.getElementById('pane-utility');
    const panePower = document.getElementById('pane-power');

    // 1. Utility Settings
    if (paneUtility) {
      const config = (window.c && window.c.getConfig && window.c.getConfig()) || {};
      const isWindowSizeSaved = config.save_window_size ?? (localStorage.getItem('goxlr_save_window_size') === 'true');

      paneUtility.innerHTML = `
        <div class="modern-settings-list">
          <div class="modern-setting-item">
            <div class="modern-setting-info">
              <div class="modern-setting-label">AutoStart on Login</div>
              <div class="modern-setting-desc">Start GoXLR Utility when the user logs in</div>
            </div>
            <label class="modern-toggle">
              <input type="checkbox" id="setting-autostart" ${config.autostart_enabled ? 'checked' : ''}>
              <span class="modern-toggle-slider"></span>
            </label>
          </div>

          <div class="modern-setting-item">
            <div class="modern-setting-info">
              <div class="modern-setting-label">Show UI on Launch</div>
              <div class="modern-setting-desc">Automatically launch the UI when GoXLR Utility starts</div>
            </div>
            <label class="modern-toggle">
              <input type="checkbox" id="setting-show-on-launch" ${config.open_ui_on_launch ? 'checked' : ''}>
              <span class="modern-toggle-slider"></span>
            </label>
          </div>

          <div class="modern-setting-item">
            <div class="modern-setting-info">
              <div class="modern-setting-label">Show Tray Icon</div>
              <div class="modern-setting-desc">Show the GoXLR Utility icon in the system tray (requires restart)</div>
            </div>
            <label class="modern-toggle">
              <input type="checkbox" id="setting-show-tray" ${config.show_tray_icon ? 'checked' : ''}>
              <span class="modern-toggle-slider"></span>
            </label>
          </div>

          <div class="modern-setting-item">
            <div class="modern-setting-info">
              <div class="modern-setting-label">Allow UI Network Access</div>
              <div class="modern-setting-desc">Allow the UI to be accessed from other devices on the network (requires restart)</div>
            </div>
            <label class="modern-toggle">
              <input type="checkbox" id="setting-network-access" ${config.allow_network_access ? 'checked' : ''}>
              <span class="modern-toggle-slider"></span>
            </label>
          </div>

          <div class="modern-setting-item">
            <div class="modern-setting-info">
              <div class="modern-setting-label">Switch LEDs Off on Shutdown</div>
              <div class="modern-setting-desc">Automatically switch all device LEDs off when PC shuts down</div>
            </div>
            <label class="modern-toggle">
              <input type="checkbox" id="setting-leds-off" ${config.shutdown_leds_off ? 'checked' : ''}>
              <span class="modern-toggle-slider"></span>
            </label>
          </div>

          <div class="modern-setting-item">
            <div class="modern-setting-info">
              <div class="modern-setting-label">Save Window Size</div>
              <div class="modern-setting-desc">Persist resized window dimensions across launches (default: off)</div>
            </div>
            <label class="modern-toggle">
              <input type="checkbox" id="setting-save-window-size" ${isWindowSizeSaved ? 'checked' : ''}>
              <span class="modern-toggle-slider"></span>
            </label>
          </div>
        </div>
      `;

      // Wire utility toggles
      const autostart = paneUtility.querySelector('#setting-autostart');
      if (autostart) autostart.addEventListener('change', (e) => {
        if (window.$) window.$.send_daemon_command({ SetAutoStartEnabled: e.target.checked });
      });

      const showLaunch = paneUtility.querySelector('#setting-show-on-launch');
      if (showLaunch) showLaunch.addEventListener('change', (e) => {
        if (window.$) window.$.send_daemon_command({ SetUiLaunchOnLoad: e.target.checked });
      });

      const showTray = paneUtility.querySelector('#setting-show-tray');
      if (showTray) showTray.addEventListener('change', (e) => {
        if (window.$) window.$.send_daemon_command({ SetShowTrayIcon: e.target.checked });
      });

      const netAccess = paneUtility.querySelector('#setting-network-access');
      if (netAccess) netAccess.addEventListener('change', (e) => {
        if (window.$) window.$.send_daemon_command({ SetAllowNetworkAccess: e.target.checked });
      });

      const ledsOff = paneUtility.querySelector('#setting-leds-off');
      if (ledsOff) ledsOff.addEventListener('change', (e) => {
        if (window.$) window.$.send_daemon_command({ SetShutdownLedsOff: e.target.checked });
      });

      const winSize = paneUtility.querySelector('#setting-save-window-size');
      if (winSize) winSize.addEventListener('change', (e) => {
        const val = e.target.checked;
        localStorage.setItem('goxlr_save_window_size', val ? 'true' : 'false');
        if (window.$ && window.$.send_daemon_command) {
          window.$.send_daemon_command({ SetSaveWindowSize: val });
        }
        if (!val) {
          enforceDefaultWindowSize();
        }
      });
    }

    // 2. Device Settings
    if (paneDevice) {
      if (window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
        const dev = window.c.getActiveDevice();
        const settings = dev.settings || {};
        const holdDelay = dev.hold_delay ?? settings.hold_delay ?? 500;
        const chatMute = dev.chat_mute_mutes_mic_to_chat ?? settings.chat_mute_mutes_mic_to_chat ?? true;
        const lockFaders = dev.lock_faders ?? settings.lock_faders ?? false;
        const monitorFx = dev.enable_monitor_with_fx ?? settings.enable_monitor_with_fx ?? false;

        paneDevice.innerHTML = `
          <div class="modern-settings-list">
            <div class="modern-setting-item">
              <div class="modern-setting-info">
                <div class="modern-setting-label">Mute Hold Duration</div>
                <div class="modern-setting-desc">Time a mute button must be held to activate secondary action</div>
              </div>
              <div class="modern-setting-slider-wrap">
                <input type="range" min="0" max="5000" step="50" id="setting-hold-delay" value="${holdDelay}">
                <span id="setting-hold-delay-val">${holdDelay}ms</span>
              </div>
            </div>

            <div class="modern-setting-item">
              <div class="modern-setting-info">
                <div class="modern-setting-label">Chat Mute Also Mutes Mic to Chat</div>
                <div class="modern-setting-desc">Muting Voice Chat also mutes Microphone to Chat Mic</div>
              </div>
              <label class="modern-toggle">
                <input type="checkbox" id="setting-chat-mute" ${chatMute ? 'checked' : ''}>
                <span class="modern-toggle-slider"></span>
              </label>
            </div>

            <div class="modern-setting-item">
              <div class="modern-setting-info">
                <div class="modern-setting-label">Lock Faders When Muting to All</div>
                <div class="modern-setting-desc">Prevent motorized faders from dropping when muting to all</div>
              </div>
              <label class="modern-toggle">
                <input type="checkbox" id="setting-lock-faders" ${lockFaders ? 'checked' : ''}>
                <span class="modern-toggle-slider"></span>
              </label>
            </div>

            <div class="modern-setting-item">
              <div class="modern-setting-info">
                <div class="modern-setting-label">Enable Monitoring with FX</div>
                <div class="modern-setting-desc">Hear effects in your headphones when FX are enabled</div>
              </div>
              <label class="modern-toggle">
                <input type="checkbox" id="setting-monitor-fx" ${monitorFx ? 'checked' : ''}>
                <span class="modern-toggle-slider"></span>
              </label>
            </div>
          </div>
        `;

        const holdSlider = paneDevice.querySelector('#setting-hold-delay');
        const holdVal = paneDevice.querySelector('#setting-hold-delay-val');
        if (holdSlider) holdSlider.addEventListener('input', (e) => {
          const val = parseInt(e.target.value, 10);
          if (holdVal) holdVal.textContent = `${val}ms`;
          if (window.$ && window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
            window.$.send_command(window.c.getActiveSerial(), { SetMuteHoldDuration: val });
          }
        });

        const chatMuteBox = paneDevice.querySelector('#setting-chat-mute');
        if (chatMuteBox) chatMuteBox.addEventListener('change', (e) => {
          if (window.$ && window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
            window.$.send_command(window.c.getActiveSerial(), { SetVCMuteAlsoMuteCM: e.target.checked });
          }
        });

        const lockFadersBox = paneDevice.querySelector('#setting-lock-faders');
        if (lockFadersBox) lockFadersBox.addEventListener('change', (e) => {
          if (window.$ && window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
            window.$.send_command(window.c.getActiveSerial(), { SetLockFaders: e.target.checked });
          }
        });

        const monitorFxBox = paneDevice.querySelector('#setting-monitor-fx');
        if (monitorFxBox) monitorFxBox.addEventListener('change', (e) => {
          if (window.$ && window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
            window.$.send_command(window.c.getActiveSerial(), { SetMonitorWithFx: e.target.checked });
          }
        });
      } else {
        paneDevice.innerHTML = '<div class="pane-loading">No active GoXLR device detected.</div>';
      }
    }

    // 3. Power Actions Grid Layout
    if (panePower) {
      const activeDev = window.c && window.c.hasActiveDevice && window.c.hasActiveDevice() ? window.c.getActiveDevice() : null;
      const shutdownCmds = activeDev?.shutdown_commands || [];
      const sleepCmds = activeDev?.sleep_commands || [];
      const wakeCmds = activeDev?.wake_commands || [];

      function hasCmd(cmdList, type) {
        return cmdList.some(c => typeof c === 'object' && c !== null && type in c);
      }

      function createPowerSection(title, desc, cmdList, setCmdName) {
        const isSaveProfile = hasCmd(cmdList, 'SaveProfile');
        const isSaveMicProfile = hasCmd(cmdList, 'SaveMicProfile');
        const isLoadProfile = hasCmd(cmdList, 'LoadProfile');
        const isLoadMicProfile = hasCmd(cmdList, 'LoadMicProfile');

        return `
          <div class="power-section-card">
            <div class="power-section-title">${title}</div>
            <div class="power-section-desc">${desc}</div>
            <div class="power-grid-2x2">
              <label class="power-grid-item">
                <input type="checkbox" data-setcmd="${setCmdName}" data-type="SaveProfile" ${isSaveProfile ? 'checked' : ''}>
                <span>Save Profile</span>
              </label>
              <label class="power-grid-item">
                <input type="checkbox" data-setcmd="${setCmdName}" data-type="SaveMicProfile" ${isSaveMicProfile ? 'checked' : ''}>
                <span>Save Mic Profile</span>
              </label>
              <label class="power-grid-item">
                <input type="checkbox" data-setcmd="${setCmdName}" data-type="LoadProfile" ${isLoadProfile ? 'checked' : ''}>
                <span>Load Profile</span>
              </label>
              <label class="power-grid-item">
                <input type="checkbox" data-setcmd="${setCmdName}" data-type="LoadMicProfile" ${isLoadMicProfile ? 'checked' : ''}>
                <span>Load Mic Profile</span>
              </label>
            </div>
          </div>
        `;
      }

      panePower.innerHTML = `
        <div class="power-actions-wrapper">
          ${createPowerSection('Shutdown Actions', 'Actions executed when shutting down', shutdownCmds, 'SetShutdownCommands')}
          ${createPowerSection('Sleep Actions', 'Actions executed when system enters sleep', sleepCmds, 'SetSleepCommands')}
          ${createPowerSection('Wake Actions', 'Actions executed when system wakes up', wakeCmds, 'SetWakeCommands')}

          <div class="power-section-card daemon-shutdown-card">
            <div class="power-section-info">
              <div class="power-section-title">Shutdown GoXLR Utility</div>
              <div class="power-section-desc">Terminates background daemon service and closes communication</div>
            </div>
            <button class="modern-danger-btn" id="btn-shutdown-daemon">Shutdown Utility</button>
          </div>
        </div>
      `;

      // Wire checkboxes
      panePower.querySelectorAll('.power-grid-item input').forEach(input => {
        input.addEventListener('change', () => {
          const setCmdName = input.getAttribute('data-setcmd');
          const sectionGrid = input.closest('.power-grid-2x2');
          const checkboxes = sectionGrid.querySelectorAll('input');
          const selectedCmds = [];

          checkboxes.forEach(cb => {
            if (cb.checked) {
              const type = cb.getAttribute('data-type');
              if (type === 'SaveProfile') selectedCmds.push({ SaveProfile: [] });
              else if (type === 'SaveMicProfile') selectedCmds.push({ SaveMicProfile: [] });
              else if (type === 'LoadProfile') {
                const curProf = activeDev ? activeDev.profile_name : 'Default';
                selectedCmds.push({ LoadProfile: [curProf, true] });
              } else if (type === 'LoadMicProfile') {
                const curMicProf = activeDev ? activeDev.mic_profile_name : 'Default Mic';
                selectedCmds.push({ LoadMicProfile: [curMicProf, true] });
              }
            }
          });

          if (window.$ && window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
            const payload = {};
            payload[setCmdName] = selectedCmds;
            window.$.send_command(window.c.getActiveSerial(), payload);
          }
        });
      });

      const shutdownBtn = panePower.querySelector('#btn-shutdown-daemon');
      if (shutdownBtn) shutdownBtn.addEventListener('click', () => {
        if (confirm('Are you sure you want to shutdown the GoXLR Utility daemon?')) {
          if (window.$) window.$.send_daemon_command('StopDaemon');
          closeUnifiedSettings();
        }
      });
    }
  }

  function setupSystemSettingsObserver() {
    function removeSystemTab() {
      // Hide system tab button from main horizontal navigation
      const navButtons = document.querySelectorAll('.tab button');
      navButtons.forEach(btn => {
        const text = (btn.textContent || '').trim().toLowerCase();
        if (text.includes('system')) {
          if (btn.style.display !== 'none') {
            btn.style.setProperty('display', 'none', 'important');
          }
          // If System tab happened to be active, switch to Mic tab
          if (btn.classList.contains('active')) {
            const micBtn = Array.from(navButtons).find(b => (b.textContent || '').trim().toLowerCase() === 'mic');
            if (micBtn) micBtn.click();
          }
        }
      });
    }

    const obs = new MutationObserver(removeSystemTab);
    obs.observe(document.body, { childList: true, subtree: true });
    removeSystemTab();
  }

  function enforceDefaultWindowSize() {
    function applySize() {
      const isSaved = (window.c && window.c.getConfig && window.c.getConfig()?.save_window_size) ||
                      (localStorage.getItem('goxlr_save_window_size') === 'true');
      if (!isSaved) {
        if (window.__TAURI__ && window.__TAURI__.window && window.__TAURI__.window.appWindow) {
          try {
            const { appWindow, LogicalSize } = window.__TAURI__.window;
            appWindow.setSize(new LogicalSize(1271, 770));
          } catch(e) {}
        } else if (window.resizeTo) {
          try {
            window.resizeTo(1271, 770);
          } catch(e) {}
        }
      }
    }
    applySize();
    setTimeout(applySize, 300);
    setTimeout(applySize, 1000);
  }

  function setupVisualizerHeightObserver() {
    function align() {
      const routingCard = document.querySelector('.modern-top-routing div:has(> table)') ||
                          document.querySelector('div:has(> table[data-v-3bfabf52])');
      const vis = document.getElementById('goxlr-visualiser');
      const svg = vis ? vis.querySelector('svg') : null;
      if (routingCard && vis) {
        const h = Math.round(routingCard.getBoundingClientRect().height);
        if (h > 50) {
          [vis, svg].forEach(el => {
            if (!el) return;
            el.style.setProperty('height', `${h}px`, 'important');
            el.style.setProperty('min-height', `${h}px`, 'important');
            el.style.setProperty('max-height', `${h}px`, 'important');
            el.style.setProperty('width', 'auto', 'important');
          });
          if (vis.parentElement) {
            vis.parentElement.style.setProperty('height', `${h}px`, 'important');
            vis.parentElement.style.setProperty('min-height', `${h}px`, 'important');
            vis.parentElement.style.setProperty('max-height', `${h}px`, 'important');
          }
        }
      }
    }
    const obs = new MutationObserver(align);
    obs.observe(document.body, { childList: true, subtree: true });
    window.addEventListener('resize', align);
    align();
    setTimeout(align, 200);
    setTimeout(align, 600);
    setTimeout(align, 1500);
  }

  // Hide Old Mic Profiles Panel from body
  function setupMicProfileObserver() {
    function hideOldMicProfiles() {
      const oldProfiles = document.querySelectorAll('.profile-border, [data-v-a1adf6fe]');
      oldProfiles.forEach(el => {
        if (el.style.display !== 'none') {
          el.style.setProperty('display', 'none', 'important');
        }
        if (el.parentElement && el.parentElement.style.display !== 'none') {
          el.parentElement.style.setProperty('display', 'none', 'important');
        }
      });
    }

    const obs = new MutationObserver(hideOldMicProfiles);
    obs.observe(document.body, { childList: true, subtree: true });
    hideOldMicProfiles();
  }

  // Quick Mic Gain Slider Outside Popup
  function setupMicGainObserver() {
    function injectQuickGain() {
      const micSetupBtn = document.getElementById('mic_setup');
      if (!micSetupBtn) return;

      const container = micSetupBtn.closest('.content');
      if (!container || container.querySelector('#modern-quick-gain-box')) return;

      const gainBox = document.createElement('div');
      gainBox.id = 'modern-quick-gain-box';
      gainBox.className = 'modern-quick-gain-box';

      let currentGain = 40;
      let micType = 'Dynamic';
      if (window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
        try {
          const status = window.c.getActiveDevice().mic_status;
          micType = status.mic_type;
          currentGain = status.mic_gains[micType] ?? 40;
        } catch(e) {}
      }

      gainBox.innerHTML = `
        <div class="quick-gain-header">
          <span class="quick-gain-title">Mic Gain</span>
          <span class="quick-gain-val" id="modern-gain-display">${currentGain} dB</span>
        </div>
        <input type="range" id="modern-gain-slider" min="0" max="72" value="${currentGain}" class="modern-gain-range" />
      `;

      micSetupBtn.parentElement.appendChild(gainBox);

      const slider = gainBox.querySelector('#modern-gain-slider');
      const display = gainBox.querySelector('#modern-gain-display');

      slider.addEventListener('input', (e) => {
        const val = parseInt(e.target.value, 10);
        display.textContent = `${val} dB`;
        if (window.$ && window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
          try {
            const mType = window.c.getActiveDevice().mic_status.mic_type;
            window.$.send_command(window.c.getActiveSerial(), { SetMicrophoneGain: [mType, val] });
            window.c.getActiveDevice().mic_status.mic_gains[mType] = val;
          } catch(err) {}
        }
      });
    }

    const obs = new MutationObserver(injectQuickGain);
    obs.observe(document.body, { childList: true, subtree: true });
    injectQuickGain();
  }

  // Routing Table Precision Crosshair & Matrix Polish
  function setupRoutingTableObserver() {
    const attachToTable = () => {
      const routingTable = document.querySelector('table[data-v-3bfabf52]');
      if (!routingTable || routingTable._modernEnhanced) return;
      routingTable._modernEnhanced = true;

      routingTable.addEventListener('mouseover', (e) => {
        const cell = e.target.closest('td[data-v-a1a932ca], div[data-v-a1a932ca]');
        if (!cell) return;

        const td = cell.closest('td');
        if (!td) return;

        const tr = td.parentElement;
        const allTdsInRow = Array.from(tr.querySelectorAll('td'));
        const colIndex = allTdsInRow.indexOf(td);

        routingTable.querySelectorAll('.active-col-header, .active-row-header, .active-cell-crosshair').forEach(el => {
          el.classList.remove('active-col-header', 'active-row-header', 'active-cell-crosshair');
        });

        const subHeader = routingTable.querySelector('thead tr.subHeader');
        if (subHeader) {
          const colHeaders = Array.from(subHeader.querySelectorAll('th:not(.hidden)'));
          if (colHeaders[colIndex]) {
            colHeaders[colIndex].classList.add('active-col-header');
          }
        }

        const rowHeader = tr.querySelector('th:not(.rotated)');
        if (rowHeader) {
          rowHeader.classList.add('active-row-header');
        }

        td.classList.add('active-cell-crosshair');
      });

      routingTable.addEventListener('mouseleave', () => {
        routingTable.querySelectorAll('.active-col-header, .active-row-header, .active-cell-crosshair').forEach(el => {
          el.classList.remove('active-col-header', 'active-row-header', 'active-cell-crosshair');
        });
      });
    };

    const observer = new MutationObserver(attachToTable);
    observer.observe(document.body, { childList: true, subtree: true });
    attachToTable();
  }

  // Compact Rename: "Channel X" -> "Ch X" in Lighting > Mixer fader buttons
  function setupCompactRenameObserver() {
    function renameChannelButtons() {
      // Target lighting mixer fader buttons (data-v-336ef9bd scope, data-v-71d31aa9 buttons)
      const lightingMixerButtons = document.querySelectorAll('div[data-v-336ef9bd] .button[data-v-71d31aa9]');
      lightingMixerButtons.forEach(btn => {
        const leftSide = btn.querySelector('.left_side');
        if (!leftSide) return;
        const text = leftSide.textContent.trim();
        if (text.startsWith('Channel ') && !text.startsWith('Ch ')) {
          leftSide.textContent = text.replace('Channel ', 'Ch ');
        }
      });
    }

    const obs = new MutationObserver(renameChannelButtons);
    obs.observe(document.body, { childList: true, subtree: true });
    renameChannelButtons();
  }

  function resetScrollPositions() {
    requestAnimationFrame(() => {
      document.querySelectorAll('.tabs-details, .container[data-v-b9b06ac2]').forEach(el => {
        el.scrollLeft = 0;
      });
    });
  }

  function setDefaultTabToMic() {
    let switched = false;
    const interval = setInterval(() => {
      const tabButtons = document.querySelectorAll('.tab button');
      for (const btn of tabButtons) {
        const txt = (btn.textContent || '').trim().toLowerCase();
        if (txt === 'mic') {
          if (!btn.classList.contains('active')) {
            btn.click();
          }
          resetScrollPositions();
          switched = true;
          clearInterval(interval);
          break;
        }
      }
    }, 40);

    setTimeout(() => {
      clearInterval(interval);
      resetScrollPositions();
    }, 4000);

    // Also reset scroll whenever any tab button or expander is clicked
    document.addEventListener('click', (e) => {
      if (e.target.closest('.tab button') || e.target.closest('.expander[data-v-cb3a0b58]')) {
        setTimeout(resetScrollPositions, 50);
        setTimeout(resetScrollPositions, 200);
      }
    });
  }

  // Initialize on DOM Ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
      const waitMain = setInterval(() => {
        if (document.getElementById('main')) {
          clearInterval(waitMain);
          initModernShell();
        }
      }, 50);
    });
  } else {
    const waitMain = setInterval(() => {
      if (document.getElementById('main')) {
        clearInterval(waitMain);
        initModernShell();
      }
    }, 50);
  }

})();
