// GoXLR Modern Layout Architecture & Unified Settings System
// Obsidian-Violet Design System

(function() {
  'use strict';

  let lastActiveTabId = 'mixer';

  function initModernShell() {
    document.title = 'GoXLR';
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

    // 10. Remove bottom footer completely
    setupFooterRemoval();

    // 11. Reorder Mixer tab sliders & add expander toggle
    setupMixerChannelsObserver();

    // 12. Center Lighting > Cough widget content
    setupCoughCenteringObserver();

    // 13. Setup Lighting > Alerts Sub-tab & Hardware Activity Engine
    setupLightingAlertsSystem();
  }

  function setupCoughCenteringObserver() {
    function centerCoughWidget() {
      const containers = document.querySelectorAll('.container, .group-container');
      containers.forEach(container => {
        const label = (container.getAttribute('aria-label') || '').toLowerCase();
        const titleEl = container.querySelector('.title, h1, h2, h3');
        const titleText = titleEl ? titleEl.textContent.toLowerCase() : '';
        if (label.includes('cough') || label.includes('bleep') || titleText.includes('cough') || titleText.includes('bleep')) {
          const contentEl = container.querySelector('.content');
          if (contentEl && contentEl.style.justifyContent !== 'center') {
            contentEl.style.setProperty('justify-content', 'center', 'important');
          }
        }
      });
    }
    const obs = new MutationObserver(centerCoughWidget);
    obs.observe(document.body, { childList: true, subtree: true });
    centerCoughWidget();
  }

  // ==========================================================================
  // Lighting > Alerts & Audio Activity System
  // ==========================================================================

  const DEFAULT_CHANNEL_CONFIGS = {
    Music: {
      enabled: true,
      toastEnabled: true,
      condition: 'either', // 'either', 'zero_only', 'muted_only'
      flashColor: '#FF1744',
      secondaryColor: '#000000',
      speed: 250,
      threshold: 2,
    },
    Chat: {
      enabled: true,
      toastEnabled: true,
      condition: 'either',
      flashColor: '#FF9100',
      secondaryColor: '#000000',
      speed: 250,
      threshold: 2,
    },
    Game: {
      enabled: false,
      toastEnabled: false,
      condition: 'either',
      flashColor: '#00E5FF',
      secondaryColor: '#000000',
      speed: 250,
      threshold: 2,
    },
    System: {
      enabled: true,
      toastEnabled: true,
      condition: 'either',
      flashColor: '#10B981',
      secondaryColor: '#000000',
      speed: 250,
      threshold: 2,
    },
    LineIn: {
      enabled: false,
      toastEnabled: false,
      condition: 'either',
      flashColor: '#10B981',
      secondaryColor: '#000000',
      speed: 250,
      threshold: 2,
    },
    Console: {
      enabled: false,
      toastEnabled: false,
      condition: 'either',
      flashColor: '#EC4899',
      secondaryColor: '#000000',
      speed: 250,
      threshold: 2,
    },
    Sample: {
      enabled: false,
      toastEnabled: false,
      condition: 'either',
      flashColor: '#EAB308',
      secondaryColor: '#000000',
      speed: 250,
      threshold: 2,
    }
  };

  const DEFAULT_ALERT_SETTINGS = {
    masterEnabled: true,
    talkMuted: {
      enabled: true,
      toastEnabled: true,
      target: 'both', // 'cough', 'fader', 'both'
      flashColor: '#A855F7',
      secondaryColor: '#12101F',
      speed: 250, // ms
      threshold: -36, // dB
      mode: 'alternate',
      showUiToast: true,
    },
    toast: {
      enabled: true,
      disable_fullscreen: false,
      position: 'top-right',
      show_duration_ms: 1000,
    },
    channels: DEFAULT_CHANNEL_CONFIGS,
  };

  let alertSettings = loadAlertSettings();
  let isAlertsSubtabActive = false;
  let activeChannelTab = 'Music';
  let isMutatingAlerts = false;
  let originalButtonColors = {};
  let isTalkingAlertActive = false;
  let talkHoldTimeout = null;
  let talkPhase = false;
  let lastTalkFlashTime = 0;
  let simulateTalk = false;
  let simulateChannel = null;
  let channelStates = {};
  let cachedChannelLevels = {};
  let lastChannelFetchTime = 0;
  let alertEngineStarted = false;
  let lastToastTriggerTime = 0;

  function getChannelState(chan) {
    if (!channelStates[chan]) {
      channelStates[chan] = {
        lastFlashTime: 0,
        flashPhase: false,
        lastAudioTime: 0,
        isActive: false,
      };
    }
    return channelStates[chan];
  }

  function loadAlertSettings() {
    try {
      const stored = localStorage.getItem('goxlr_alert_settings');
      if (stored) {
        const parsed = JSON.parse(stored);
        const mergedChannels = {};
        for (const [ch, def] of Object.entries(DEFAULT_CHANNEL_CONFIGS)) {
          mergedChannels[ch] = { ...def, ...(parsed.channels?.[ch] || {}) };
        }
        return {
          ...DEFAULT_ALERT_SETTINGS,
          ...parsed,
          talkMuted: { ...DEFAULT_ALERT_SETTINGS.talkMuted, ...(parsed.talkMuted || {}) },
          toast: { ...DEFAULT_ALERT_SETTINGS.toast, ...(parsed.toast || {}) },
          channels: mergedChannels,
        };
      }
    } catch (e) {}
    return JSON.parse(JSON.stringify(DEFAULT_ALERT_SETTINGS));
  }

  function saveAlertSettings() {
    try {
      localStorage.setItem('goxlr_alert_settings', JSON.stringify(alertSettings));
      if (alertSettings.toast) {
        const toastPayload = {
          enabled: alertSettings.toast.enabled !== false,
          disable_fullscreen: !!alertSettings.toast.disable_fullscreen,
          position: alertSettings.toast.position || 'bottom-center',
          show_duration_ms: alertSettings.toast.show_duration_ms || 3000,
          color: alertSettings.talkMuted?.flashColor || '#FF1744',
          mic_color: alertSettings.talkMuted?.flashColor || '#FF1744',
          chat_color: alertSettings.channels?.Chat?.flashColor || '#FF9100',
          music_color: alertSettings.channels?.Music?.flashColor || '#FF1744',
          system_color: alertSettings.channels?.System?.flashColor || '#00E5FF',
          mic_toast: alertSettings.talkMuted?.toastEnabled !== false,
          chat_toast: alertSettings.channels?.Chat?.toastEnabled !== false,
          music_toast: alertSettings.channels?.Music?.toastEnabled !== false,
          system_toast: alertSettings.channels?.System?.toastEnabled !== false,
        };
        fetch('/api/toast/settings', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(toastPayload),
        }).catch(() => {});
      }
    } catch (e) {}
  }

  function showToastNotification(text, color, force = false) {
    const toastCfg = alertSettings.toast || DEFAULT_ALERT_SETTINGS.toast;
    if (!toastCfg.enabled) return;

    const now = Date.now();
    const duration = toastCfg.show_duration_ms || 3000;
    if (!force && now - lastToastTriggerTime < duration) return;
    lastToastTriggerTime = now;

    fetch('/api/toast/trigger', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        message: text,
        color: color || '#00E5FF',
        position: toastCfg.position,
        duration_ms: duration,
      }),
    }).catch(() => {});
  }

  function cleanHex(color) {
    if (!color) return '000000';
    return color.replace('#', '').trim().toUpperCase();
  }

  function getFaderButtonForChannel(channelName) {
    if (!window.c || !window.c.hasActiveDevice || !window.c.hasActiveDevice()) return null;
    const dev = window.c.getActiveDevice();
    const faders = dev?.fader_status || {};
    for (const [faderName, status] of Object.entries(faders)) {
      if (status.channel === channelName) {
        if (faderName === 'A') return 'Fader1Mute';
        if (faderName === 'B') return 'Fader2Mute';
        if (faderName === 'C') return 'Fader3Mute';
        if (faderName === 'D') return 'Fader4Mute';
      }
    }
    return null;
  }

  function getMicFaderButton() {
    return getFaderButtonForChannel('Mic') || 'Fader1Mute';
  }

  function getChannelVolume(dev, chan) {
    if (!dev || !dev.levels) return 255;
    if (dev.levels.volumes && typeof dev.levels.volumes[chan] === 'number') {
      return dev.levels.volumes[chan];
    }
    if (dev.levels.submix?.inputs?.[chan]?.volume !== undefined) {
      return dev.levels.submix.inputs[chan].volume;
    }
    return 255;
  }

  function saveOriginalButtonColor(buttonName, flashCol) {
    if (originalButtonColors[buttonName]) return;
    const dev = window.c?.getActiveDevice?.();
    const btnCfg = dev?.lighting?.buttons?.[buttonName];
    if (btnCfg && btnCfg.colours) {
      const c1 = btnCfg.colours.colour_one || '000000';
      const c2 = btnCfg.colours.colour_two || '000000';
      if (flashCol && cleanHex(c1) === cleanHex(flashCol) && cleanHex(c2) === cleanHex(flashCol)) {
        return;
      }
      originalButtonColors[buttonName] = {
        colour_one: c1,
        colour_two: c2,
      };
    }
  }

  function restoreOriginalButtonColor(buttonName) {
    const orig = originalButtonColors[buttonName];
    if (!orig || !window.$ || !window.c || !window.c.hasActiveDevice || !window.c.hasActiveDevice()) return;
    const serial = window.c.getActiveSerial();
    window.$.send_command(serial, {
      SetButtonColours: [buttonName, orig.colour_one, orig.colour_two]
    });
    delete originalButtonColors[buttonName];
  }

  function restoreAllOriginalButtonColors() {
    for (const btn of Object.keys(originalButtonColors)) {
      restoreOriginalButtonColor(btn);
    }
    for (const state of Object.values(channelStates)) {
      state.isActive = false;
      state.flashPhase = false;
    }
  }

  function initAlertEngine() {
    if (alertEngineStarted) return;
    alertEngineStarted = true;
    saveAlertSettings();

    setInterval(async () => {
      const now = Date.now();
      if (!window.c || !window.c.hasActiveDevice || !window.c.hasActiveDevice() || !window.$) return;
      const serial = window.c.getActiveSerial();
      const dev = window.c.getActiveDevice();

      // Fetch channel peak levels periodically (every 140ms)
      if (now - lastChannelFetchTime > 140) {
        lastChannelFetchTime = now;
        try {
          const resp = await fetch('/api/channel-levels?_=' + now, { cache: 'no-store' });
          if (resp.ok) {
            cachedChannelLevels = await resp.json();
          }
        } catch (e) {}
      }

      // 1. Process Talk While Muted
      let micLevelDb = -72.2;
      try {
        if (window.$.get_mic_level) {
          const res = await window.$.get_mic_level(serial);
          if (typeof res === 'number') {
            micLevelDb = res;
          } else if (res && typeof res.MicLevel === 'number') {
            micLevelDb = res.MicLevel;
          }
        }
      } catch (e) {}

      // Fallback to WASAPI channel levels if GoXLR mic level is resting or unavailable
      if (micLevelDb <= -70 && cachedChannelLevels) {
        const wasapiMic = cachedChannelLevels['Mic'] ?? cachedChannelLevels['Chat Mic'] ?? cachedChannelLevels['mic'] ?? 0;
        if (wasapiMic > 0.001) {
          const approxDb = 20 * Math.log10(wasapiMic);
          if (approxDb > micLevelDb) {
            micLevelDb = approxDb;
          }
        }
      }

      // Update Live UI Meter if rendered
      const liveFill = document.getElementById('alerts-live-meter-fill');
      const liveVal = document.getElementById('alerts-live-meter-val');
      if (liveFill && liveVal) {
        const pct = Math.min(100, Math.max(0, ((micLevelDb + 60) / 50) * 100));
        liveFill.style.width = pct + '%';
        liveVal.textContent = Math.round(micLevelDb) + ' dB';
      }

      // Check mic muted state
      const isCoughMuted = dev?.cough_button?.state && dev.cough_button.state !== 'Unmuted';
      const micFaderBtn = getMicFaderButton();
      const micFaderStatus = dev?.fader_status?.[micFaderBtn === 'Fader1Mute' ? 'A' : micFaderBtn === 'Fader2Mute' ? 'B' : micFaderBtn === 'Fader3Mute' ? 'C' : 'D'];
      const isMicFaderMuted = micFaderStatus && micFaderStatus.mute_state !== 'Unmuted';
      const isMicMuted = isCoughMuted || isMicFaderMuted;

      const shouldFlashTalk = simulateTalk || (alertSettings.masterEnabled && alertSettings.talkMuted.enabled && isMicMuted && micLevelDb >= alertSettings.talkMuted.threshold);

      if (shouldFlashTalk) {
        const targets = [];
        if (alertSettings.talkMuted.target === 'cough' || alertSettings.talkMuted.target === 'both') {
          targets.push('Cough');
        }
        if (alertSettings.talkMuted.target === 'fader' || alertSettings.talkMuted.target === 'both') {
          targets.push(micFaderBtn);
        }

        const flashCol = cleanHex(alertSettings.talkMuted.flashColor);
        targets.forEach(btn => saveOriginalButtonColor(btn, flashCol));

        if (now - lastTalkFlashTime >= alertSettings.talkMuted.speed) {
          talkPhase = !talkPhase;
          lastTalkFlashTime = now;
          const secCol = alertSettings.talkMuted.mode === 'off' ? '000000' : cleanHex(alertSettings.talkMuted.secondaryColor);
          const c1 = talkPhase ? flashCol : secCol;
          const c2 = talkPhase ? flashCol : '000000';

          targets.forEach(btn => {
            window.$.send_command(serial, { SetButtonColours: [btn, c1, c2] });
          });
        }

        if (alertSettings.toast?.enabled && alertSettings.talkMuted?.toastEnabled !== false) {
          showToastNotification('TALKING WHILE MUTED', alertSettings.talkMuted.flashColor);
        }

        if (talkHoldTimeout) {
          clearTimeout(talkHoldTimeout);
          talkHoldTimeout = null;
        }
        isTalkingAlertActive = true;
      } else if (isTalkingAlertActive) {
        if (!talkHoldTimeout) {
          talkHoldTimeout = setTimeout(() => {
            const targets = ['Cough', micFaderBtn];
            targets.forEach(restoreOriginalButtonColor);
            isTalkingAlertActive = false;
            talkHoldTimeout = null;
          }, 350);
        }
      }

      // 2. Process Channel Activity Alert (0% volume / muted) for the audio sliders
      const availableChans = ['Chat', 'Music', 'System'];
      for (const chan of availableChans) {
        const cfg = alertSettings.channels?.[chan];
        const state = getChannelState(chan);
        const btnName = getFaderButtonForChannel(chan);

        if (!btnName || !cfg || !alertSettings.masterEnabled || (!cfg.enabled && cfg.toastEnabled === false)) {
          if (state.isActive && btnName) {
            restoreOriginalButtonColor(btnName);
            state.isActive = false;
            state.flashPhase = false;
          }
          continue;
        }

        const faderKey = btnName === 'Fader1Mute' ? 'A' : btnName === 'Fader2Mute' ? 'B' : btnName === 'Fader3Mute' ? 'C' : 'D';
        const faderStatus = dev?.fader_status?.[faderKey];
        const isChanMuted = faderStatus && faderStatus.mute_state !== 'Unmuted';
        const chanVol = getChannelVolume(dev, chan);
        const isZeroVol = chanVol <= 2; // <= 1% volume or 0 on 0-255 scale

        let condMet = false;
        if (cfg.condition === 'zero_only') condMet = isZeroVol;
        else if (cfg.condition === 'muted_only') condMet = isChanMuted;
        else condMet = isZeroVol || isChanMuted;

        const peak = cachedChannelLevels[chan] ?? cachedChannelLevels[chan.toLowerCase()] ?? 0;
        const peakPct = peak * 100;
        const isAudioActive = (simulateChannel === chan) || (peakPct >= (cfg.threshold || 2));

        if (isAudioActive) {
          state.lastAudioTime = now;
        }

        // Audio hold for 2000ms prevents flashing from stopping between beats or brief pauses
        const hasRecentAudio = (now - state.lastAudioTime < 2000);

        if (condMet && hasRecentAudio) {
          if (alertSettings.toast?.enabled && cfg.toastEnabled !== false) {
            showToastNotification('MUTED ' + chan.toUpperCase(), cfg.flashColor);
          }
          if (cfg.enabled) {
            const flashCol = cleanHex(cfg.flashColor || '#FF1744');
            saveOriginalButtonColor(btnName, flashCol);
            state.isActive = true;

            const speed = cfg.speed || 250;
            if (now - state.lastFlashTime >= speed) {
              state.flashPhase = !state.flashPhase;
              state.lastFlashTime = now;

              const secCol = cleanHex(cfg.secondaryColor || '000000');
              const curCol = state.flashPhase ? flashCol : secCol;

              // Set both colour_one and colour_two so it blinks whether unmuted (0% vol) or muted!
              window.$.send_command(serial, {
                SetButtonColours: [btnName, curCol, curCol]
              });
            }
          }
        } else if (state.isActive) {
          restoreOriginalButtonColor(btnName);
          state.isActive = false;
          state.flashPhase = false;
        }
      }
    }, 75);
  }

  function renderAlertsPanel(container, forceRebuild = false) {
    let panel = document.getElementById('modern-lighting-alerts-panel');
    if (!panel) {
      isMutatingAlerts = true;
      panel = document.createElement('div');
      panel.id = 'modern-lighting-alerts-panel';
      container.appendChild(panel);
      buildAlertsPanelContent(panel);
      isMutatingAlerts = false;
    } else {
      if (forceRebuild) {
        isMutatingAlerts = true;
        buildAlertsPanelContent(panel);
        isMutatingAlerts = false;
      }
    }
    if (panel.style.display !== 'flex') {
      panel.style.setProperty('display', 'flex', 'important');
    }
  }

  function buildAlertsPanelContent(panel) {
    const talk = alertSettings.talkMuted;
    const chatCfg = alertSettings.channels?.Chat || DEFAULT_CHANNEL_CONFIGS.Chat;
    const musicCfg = alertSettings.channels?.Music || DEFAULT_CHANNEL_CONFIGS.Music;
    const systemCfg = alertSettings.channels?.System || DEFAULT_CHANNEL_CONFIGS.System;
    const toastCfg = alertSettings.toast || DEFAULT_ALERT_SETTINGS.toast;

    panel.innerHTML = `
      <div class="modern-alerts-container">
        <!-- Left Side: 2x2 Channels Grid -->
        <div class="alerts-left-column">
          <div class="alerts-2x2-grid">
            <!-- 1. Mic Slider -->
            <div class="alerts-card-compact" id="card-alert-mic">
              <div class="alerts-card-compact-header">
                <div class="alerts-card-compact-title-group">
                  <div class="alerts-card-compact-icon">
                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3z"></path>
                      <path d="M19 10v2a7 7 0 0 1-14 0v-2"></path>
                      <line x1="12" y1="19" x2="12" y2="23"></line>
                      <line x1="8" y1="23" x2="16" y2="23"></line>
                    </svg>
                  </div>
                  <div>
                    <div class="alerts-card-compact-title">MIC</div>
                    <div class="alerts-card-compact-desc">Talking while muted</div>
                  </div>
                </div>
                <div class="alerts-card-compact-toggles">
                  <div class="alerts-inline-toggle" title="Toggle Hardware Blink">
                    <span class="alerts-toggle-label">Blink</span>
                    <label class="modern-toggle">
                      <input type="checkbox" id="setting-mic-enabled" ${talk.enabled ? 'checked' : ''}>
                      <span class="modern-toggle-slider"></span>
                    </label>
                  </div>
                  <div class="alerts-inline-toggle" title="Toggle Screen Toast Notification">
                    <span class="alerts-toggle-label">Toast</span>
                    <label class="modern-toggle">
                      <input type="checkbox" id="setting-mic-toast" ${talk.toastEnabled !== false ? 'checked' : ''}>
                      <span class="modern-toggle-slider"></span>
                    </label>
                  </div>
                </div>
              </div>

              <div class="alerts-card-compact-body">
                <div class="alerts-color-control">
                  <div class="alerts-color-swatch-wrap">
                    <div class="alerts-color-swatch" id="mic-color-swatch" style="background-color: ${talk.flashColor}"></div>
                    <input type="color" class="alerts-color-input-native" id="setting-mic-color" value="${talk.flashColor}">
                  </div>
                  <input type="text" class="alerts-hex-input" id="setting-mic-hex" value="${talk.flashColor}" maxlength="7">
                </div>
                <div class="alerts-presets-list" id="mic-color-presets">
                  <span class="alerts-preset-dot" data-col="#FF1744" style="background: #FF1744;" title="Red"></span>
                  <span class="alerts-preset-dot" data-col="#FF9100" style="background: #FF9100;" title="Orange"></span>
                  <span class="alerts-preset-dot" data-col="#A855F7" style="background: #A855F7;" title="Purple"></span>
                  <span class="alerts-preset-dot" data-col="#00E5FF" style="background: #00E5FF;" title="Cyan"></span>
                  <span class="alerts-preset-dot" data-col="#10B981" style="background: #10B981;" title="Green"></span>
                </div>
              </div>
            </div>

            <!-- 2. Voice Chat Slider -->
            <div class="alerts-card-compact" id="card-alert-chat">
              <div class="alerts-card-compact-header">
                <div class="alerts-card-compact-title-group">
                  <div class="alerts-card-compact-icon">
                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
                    </svg>
                  </div>
                  <div>
                    <div class="alerts-card-compact-title">VOICE CHAT</div>
                    <div class="alerts-card-compact-desc">Sound while 0% or muted</div>
                  </div>
                </div>
                <div class="alerts-card-compact-toggles">
                  <div class="alerts-inline-toggle" title="Toggle Hardware Blink">
                    <span class="alerts-toggle-label">Blink</span>
                    <label class="modern-toggle">
                      <input type="checkbox" id="setting-chat-enabled" ${chatCfg.enabled ? 'checked' : ''}>
                      <span class="modern-toggle-slider"></span>
                    </label>
                  </div>
                  <div class="alerts-inline-toggle" title="Toggle Screen Toast Notification">
                    <span class="alerts-toggle-label">Toast</span>
                    <label class="modern-toggle">
                      <input type="checkbox" id="setting-chat-toast" ${chatCfg.toastEnabled !== false ? 'checked' : ''}>
                      <span class="modern-toggle-slider"></span>
                    </label>
                  </div>
                </div>
              </div>

              <div class="alerts-card-compact-body">
                <div class="alerts-color-control">
                  <div class="alerts-color-swatch-wrap">
                    <div class="alerts-color-swatch" id="chat-color-swatch" style="background-color: ${chatCfg.flashColor}"></div>
                    <input type="color" class="alerts-color-input-native" id="setting-chat-color" value="${chatCfg.flashColor}">
                  </div>
                  <input type="text" class="alerts-hex-input" id="setting-chat-hex" value="${chatCfg.flashColor}" maxlength="7">
                </div>
                <div class="alerts-presets-list" id="chat-color-presets">
                  <span class="alerts-preset-dot" data-col="#FF1744" style="background: #FF1744;" title="Red"></span>
                  <span class="alerts-preset-dot" data-col="#FF9100" style="background: #FF9100;" title="Orange"></span>
                  <span class="alerts-preset-dot" data-col="#A855F7" style="background: #A855F7;" title="Purple"></span>
                  <span class="alerts-preset-dot" data-col="#00E5FF" style="background: #00E5FF;" title="Cyan"></span>
                  <span class="alerts-preset-dot" data-col="#10B981" style="background: #10B981;" title="Green"></span>
                </div>
              </div>
            </div>

            <!-- 3. Music Slider -->
            <div class="alerts-card-compact" id="card-alert-music">
              <div class="alerts-card-compact-header">
                <div class="alerts-card-compact-title-group">
                  <div class="alerts-card-compact-icon">
                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M9 18V5l12-2v13"></path>
                      <circle cx="6" cy="18" r="3"></circle>
                      <circle cx="18" cy="16" r="3"></circle>
                    </svg>
                  </div>
                  <div>
                    <div class="alerts-card-compact-title">MUSIC</div>
                    <div class="alerts-card-compact-desc">Sound while 0% or muted</div>
                  </div>
                </div>
                <div class="alerts-card-compact-toggles">
                  <div class="alerts-inline-toggle" title="Toggle Hardware Blink">
                    <span class="alerts-toggle-label">Blink</span>
                    <label class="modern-toggle">
                      <input type="checkbox" id="setting-music-enabled" ${musicCfg.enabled ? 'checked' : ''}>
                      <span class="modern-toggle-slider"></span>
                    </label>
                  </div>
                  <div class="alerts-inline-toggle" title="Toggle Screen Toast Notification">
                    <span class="alerts-toggle-label">Toast</span>
                    <label class="modern-toggle">
                      <input type="checkbox" id="setting-music-toast" ${musicCfg.toastEnabled !== false ? 'checked' : ''}>
                      <span class="modern-toggle-slider"></span>
                    </label>
                  </div>
                </div>
              </div>

              <div class="alerts-card-compact-body">
                <div class="alerts-color-control">
                  <div class="alerts-color-swatch-wrap">
                    <div class="alerts-color-swatch" id="music-color-swatch" style="background-color: ${musicCfg.flashColor}"></div>
                    <input type="color" class="alerts-color-input-native" id="setting-music-color" value="${musicCfg.flashColor}">
                  </div>
                  <input type="text" class="alerts-hex-input" id="setting-music-hex" value="${musicCfg.flashColor}" maxlength="7">
                </div>
                <div class="alerts-presets-list" id="music-color-presets">
                  <span class="alerts-preset-dot" data-col="#FF1744" style="background: #FF1744;" title="Red"></span>
                  <span class="alerts-preset-dot" data-col="#FF9100" style="background: #FF9100;" title="Orange"></span>
                  <span class="alerts-preset-dot" data-col="#A855F7" style="background: #A855F7;" title="Purple"></span>
                  <span class="alerts-preset-dot" data-col="#00E5FF" style="background: #00E5FF;" title="Cyan"></span>
                  <span class="alerts-preset-dot" data-col="#10B981" style="background: #10B981;" title="Green"></span>
                </div>
              </div>
            </div>

            <!-- 4. System Slider -->
            <div class="alerts-card-compact" id="card-alert-system">
              <div class="alerts-card-compact-header">
                <div class="alerts-card-compact-title-group">
                  <div class="alerts-card-compact-icon">
                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
                      <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
                      <path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07"></path>
                    </svg>
                  </div>
                  <div>
                    <div class="alerts-card-compact-title">SYSTEM</div>
                    <div class="alerts-card-compact-desc">Sound while 0% or muted</div>
                  </div>
                </div>
                <div class="alerts-card-compact-toggles">
                  <div class="alerts-inline-toggle" title="Toggle Hardware Blink">
                    <span class="alerts-toggle-label">Blink</span>
                    <label class="modern-toggle">
                      <input type="checkbox" id="setting-system-enabled" ${systemCfg.enabled ? 'checked' : ''}>
                      <span class="modern-toggle-slider"></span>
                    </label>
                  </div>
                  <div class="alerts-inline-toggle" title="Toggle Screen Toast Notification">
                    <span class="alerts-toggle-label">Toast</span>
                    <label class="modern-toggle">
                      <input type="checkbox" id="setting-system-toast" ${systemCfg.toastEnabled !== false ? 'checked' : ''}>
                      <span class="modern-toggle-slider"></span>
                    </label>
                  </div>
                </div>
              </div>

              <div class="alerts-card-compact-body">
                <div class="alerts-color-control">
                  <div class="alerts-color-swatch-wrap">
                    <div class="alerts-color-swatch" id="system-color-swatch" style="background-color: ${systemCfg.flashColor}"></div>
                    <input type="color" class="alerts-color-input-native" id="setting-system-color" value="${systemCfg.flashColor}">
                  </div>
                  <input type="text" class="alerts-hex-input" id="setting-system-hex" value="${systemCfg.flashColor}" maxlength="7">
                </div>
                <div class="alerts-presets-list" id="system-color-presets">
                  <span class="alerts-preset-dot" data-col="#FF1744" style="background: #FF1744;" title="Red"></span>
                  <span class="alerts-preset-dot" data-col="#FF9100" style="background: #FF9100;" title="Orange"></span>
                  <span class="alerts-preset-dot" data-col="#A855F7" style="background: #A855F7;" title="Purple"></span>
                  <span class="alerts-preset-dot" data-col="#00E5FF" style="background: #00E5FF;" title="Cyan"></span>
                  <span class="alerts-preset-dot" data-col="#10B981" style="background: #10B981;" title="Green"></span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Right Side: Toast Notification Options -->
        <div class="alerts-right-column">
          <div class="alerts-toast-card">
            <div class="alerts-toast-card-header">
              <div class="alerts-card-compact-icon">
                <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
                  <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
                  <line x1="8" y1="21" x2="16" y2="21"></line>
                  <line x1="12" y1="17" x2="12" y2="21"></line>
                </svg>
              </div>
              <div>
                <div class="alerts-toast-card-title">TOAST OVERLAY</div>
                <div class="alerts-toast-card-desc">Always on top of all apps</div>
              </div>
            </div>

            <div class="alerts-toast-controls">
              <!-- Toggles Row -->
              <div class="alerts-toast-toggles-row">
                <div class="alerts-toast-toggle-item">
                  <span class="alerts-toast-label">Enable Toast</span>
                  <label class="modern-toggle" title="Master toggle for on-screen toast">
                    <input type="checkbox" id="setting-toast-enabled" ${toastCfg.enabled !== false ? 'checked' : ''}>
                    <span class="modern-toggle-slider"></span>
                  </label>
                </div>
                <div class="alerts-toast-toggle-item">
                  <span class="alerts-toast-label">Hide Fullscreen</span>
                  <label class="modern-toggle" title="Disable toast in fullscreen games/apps only">
                    <input type="checkbox" id="setting-toast-fullscreen" ${toastCfg.disable_fullscreen ? 'checked' : ''}>
                    <span class="modern-toggle-slider"></span>
                  </label>
                </div>
              </div>

              <!-- Position Row -->
              <div class="alerts-toast-row">
                <span class="alerts-toast-label">Position</span>
                <select class="alerts-toast-select" id="setting-toast-position">
                  <option value="top-center" ${toastCfg.position === 'top-center' ? 'selected' : ''}>Top Center</option>
                  <option value="top-left" ${toastCfg.position === 'top-left' ? 'selected' : ''}>Top Left</option>
                  <option value="top-right" ${toastCfg.position === 'top-right' ? 'selected' : ''}>Top Right</option>
                  <option value="bottom-center" ${toastCfg.position === 'bottom-center' ? 'selected' : ''}>Bottom Center</option>
                  <option value="bottom-left" ${toastCfg.position === 'bottom-left' ? 'selected' : ''}>Bottom Left</option>
                  <option value="bottom-right" ${toastCfg.position === 'bottom-right' ? 'selected' : ''}>Bottom Right</option>
                  <option value="center-left" ${toastCfg.position === 'center-left' ? 'selected' : ''}>Center Left</option>
                  <option value="center-right" ${toastCfg.position === 'center-right' ? 'selected' : ''}>Center Right</option>
                  <option value="center" ${toastCfg.position === 'center' ? 'selected' : ''}>Center</option>
                </select>
              </div>

              <!-- Duration Row -->
              <div class="alerts-toast-row">
                <div class="alerts-toast-slider-group">
                  <div class="alerts-toast-slider-header">
                    <span class="alerts-toast-label">Duration</span>
                    <span class="alerts-val-badge" id="toast-duration-badge">${((toastCfg.show_duration_ms || 3000) / 1000).toFixed(1)}s</span>
                  </div>
                  <input type="range" class="alerts-range" id="setting-toast-duration" min="1000" max="8000" step="500" value="${toastCfg.show_duration_ms || 3000}">
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    `;

    function wireCard(prefix, cfg, getBtnFn) {
      const toggle = panel.querySelector(`#setting-${prefix}-enabled`);
      if (toggle) {
        toggle.addEventListener('change', (e) => {
          cfg.enabled = e.target.checked;
          saveAlertSettings();
          if (!cfg.enabled) {
            const btn = getBtnFn();
            if (Array.isArray(btn)) btn.forEach(restoreOriginalButtonColor);
            else if (btn) restoreOriginalButtonColor(btn);
          }
        });
      }

      const toastToggle = panel.querySelector(`#setting-${prefix}-toast`);
      if (toastToggle) {
        toastToggle.addEventListener('change', (e) => {
          cfg.toastEnabled = e.target.checked;
          saveAlertSettings();
        });
      }

      const colorInput = panel.querySelector(`#setting-${prefix}-color`);
      const hexInput = panel.querySelector(`#setting-${prefix}-hex`);
      const swatch = panel.querySelector(`#${prefix}-color-swatch`);

      function setColor(hex) {
        if (!hex.startsWith('#')) hex = '#' + hex;
        cfg.flashColor = hex;
        if (swatch) swatch.style.backgroundColor = hex;
        if (colorInput) colorInput.value = hex;
        if (hexInput) hexInput.value = hex.toUpperCase();
        saveAlertSettings();
      }

      if (colorInput) colorInput.addEventListener('input', (e) => setColor(e.target.value));
      if (hexInput) hexInput.addEventListener('change', (e) => setColor(e.target.value));

      panel.querySelectorAll(`#${prefix}-color-presets .alerts-preset-dot`).forEach(dot => {
        dot.addEventListener('click', () => setColor(dot.getAttribute('data-col')));
      });
    }

    wireCard('mic', talk, () => ['Cough', getMicFaderButton()]);
    wireCard('chat', chatCfg, () => getFaderButtonForChannel('Chat'));
    wireCard('music', musicCfg, () => getFaderButtonForChannel('Music'));
    wireCard('system', systemCfg, () => getFaderButtonForChannel('System'));

    // Wire Toast Options
    const toastToggle = panel.querySelector('#setting-toast-enabled');
    if (toastToggle) {
      toastToggle.addEventListener('change', (e) => {
        toastCfg.enabled = e.target.checked;
        saveAlertSettings();
      });
    }

    const fsToggle = panel.querySelector('#setting-toast-fullscreen');
    if (fsToggle) {
      fsToggle.addEventListener('change', (e) => {
        toastCfg.disable_fullscreen = e.target.checked;
        saveAlertSettings();
      });
    }

    const posSelect = panel.querySelector('#setting-toast-position');
    if (posSelect) {
      posSelect.addEventListener('change', (e) => {
        toastCfg.position = e.target.value;
        saveAlertSettings();
      });
    }

    const durationRange = panel.querySelector('#setting-toast-duration');
    const durationBadge = panel.querySelector('#toast-duration-badge');
    if (durationRange) {
      durationRange.addEventListener('input', (e) => {
        const val = parseInt(e.target.value, 10);
        toastCfg.show_duration_ms = val;
        if (durationBadge) durationBadge.textContent = (val / 1000).toFixed(1) + 's';
        saveAlertSettings();
      });
    }
  }

  function hideAlertsPanel() {
    const panel = document.getElementById('modern-lighting-alerts-panel');
    if (panel && panel.style.display !== 'none') {
      panel.style.setProperty('display', 'none', 'important');
    }
    const container = document.querySelector('.sections[data-v-774daee4]');
    if (container && container.parentElement) {
      const children = Array.from(container.parentElement.children);
      children.forEach(ch => {
        if (ch !== container && ch !== panel) {
          ch.style.removeProperty('display');
        }
      });
    }
  }

  function setupLightingAlertsSystem() {
    initAlertEngine();

    function injectAlertsTab() {
      if (isMutatingAlerts) return;
      const sections = document.querySelector('.sections[data-v-774daee4]');
      if (!sections) {
        if (isAlertsSubtabActive) {
          hideAlertsPanel();
        }
        return;
      }

      // Check if button already injected
      let alertsBtn = sections.querySelector('#modern-subtab-alerts');
      if (!alertsBtn) {
        alertsBtn = document.createElement('button');
        alertsBtn.id = 'modern-subtab-alerts';
        alertsBtn.className = 'button modern-alerts-subtab-btn';
        alertsBtn.setAttribute('data-v-774daee4', '');
        alertsBtn.setAttribute('role', 'tab');
        alertsBtn.setAttribute('tabindex', '-1');
        alertsBtn.textContent = 'Alerts';

        // Insert right after Cough button (or at the end)
        const buttons = Array.from(sections.querySelectorAll('.button'));
        const coughBtn = buttons.find(b => {
          const t = (b.textContent || '').trim().toLowerCase();
          return t.includes('cough') || t.includes('bleep');
        });

        if (coughBtn && coughBtn.nextSibling) {
          sections.insertBefore(alertsBtn, coughBtn.nextSibling);
        } else {
          sections.appendChild(alertsBtn);
        }

        alertsBtn.addEventListener('click', (e) => {
          e.preventDefault();
          e.stopPropagation();
          isAlertsSubtabActive = true;

          // Deactivate all sibling buttons
          sections.querySelectorAll('.button').forEach(b => {
            b.classList.remove('active');
            b.setAttribute('tabindex', '-1');
            b.setAttribute('aria-selected', 'false');
          });

          alertsBtn.classList.add('active');
          alertsBtn.setAttribute('tabindex', '0');
          alertsBtn.setAttribute('aria-selected', 'true');

          // Hide Vue sibling panel and show our panel
          if (sections.parentElement) {
            Array.from(sections.parentElement.children).forEach(ch => {
              if (ch !== sections && ch.id !== 'modern-lighting-alerts-panel') {
                if (ch.style.display !== 'none') {
                  ch.style.setProperty('display', 'none', 'important');
                }
              }
            });
            renderAlertsPanel(sections.parentElement);
          }
        });
      }

      // Hook click on other buttons to restore normal Vue view
      sections.querySelectorAll('.button:not(#modern-subtab-alerts)').forEach(btn => {
        if (!btn._alertsHooked) {
          btn._alertsHooked = true;
          btn.addEventListener('click', () => {
            isAlertsSubtabActive = false;
            const b = sections.querySelector('#modern-subtab-alerts');
            if (b) {
              b.classList.remove('active');
              b.setAttribute('aria-selected', 'false');
            }
            hideAlertsPanel();
          });
        }
      });

      // Maintain active state if Alerts subtab was active
      if (isAlertsSubtabActive) {
        if (!alertsBtn.classList.contains('active')) {
          alertsBtn.classList.add('active');
        }
        if (sections.parentElement) {
          Array.from(sections.parentElement.children).forEach(ch => {
            if (ch !== sections && ch.id !== 'modern-lighting-alerts-panel') {
              if (ch.style.display !== 'none') {
                ch.style.setProperty('display', 'none', 'important');
              }
            }
          });
          const existingPanel = document.getElementById('modern-lighting-alerts-panel');
          if (!existingPanel || existingPanel.style.display === 'none') {
            renderAlertsPanel(sections.parentElement);
          }
        }
      }
    }

    const obs = new MutationObserver(injectAlertsTab);
    obs.observe(document.body, { childList: true, subtree: true });
    injectAlertsTab();
  }

  let isMixerCollapsed = true;

  function getChannelOrder(el) {
    const text = (el.textContent || '').trim().toLowerCase();
    if (text.includes('microphone') || text.startsWith('mic')) return 0;
    if (text.includes('voice chat') || text.includes('chat')) return 1;
    if (text.includes('music')) return 2;

    if (text.includes('system')) return 3;
    if (text.includes('game')) return 5;
    if (text.includes('console')) return 6;
    if (text.includes('line in') || text.includes('linein')) return 7;
    if (text.includes('samples') || text.includes('sample')) return 8;
    return -1;
  }

  function updateMixerChannels() {
    const containers = document.querySelectorAll('.content[data-v-b9d1e087], .container[data-v-b9d1e087], .faders-container, .mixer-channels');
    containers.forEach(container => {
      const children = Array.from(container.children);
      const channelChildren = children.filter(ch => getChannelOrder(ch) >= 0);
      if (channelChildren.length < 4) return;

      if (container.style.display !== 'flex') {
        container.style.display = 'flex';
      }

      channelChildren.forEach(ch => {
        const order = getChannelOrder(ch);
        const orderStr = String(order);
        if (ch.style.order !== orderStr) {
          ch.style.order = orderStr;
        }
        const isCollapsible = order >= 5;
        if (isCollapsible) {
          const targetDisplay = isMixerCollapsed ? 'none' : '';
          if (ch.style.display !== targetDisplay) {
            ch.style.display = targetDisplay;
          }
        }
      });

      let expanderBtn = container.querySelector('.modern-mixer-expander');
      if (!expanderBtn) {
        expanderBtn = document.createElement('button');
        expanderBtn.className = 'expander modern-mixer-expander';
        expanderBtn.setAttribute('data-v-cb3a0b58', '');
        expanderBtn.style.order = '4';
        expanderBtn.addEventListener('click', (e) => {
          e.stopPropagation();
          isMixerCollapsed = !isMixerCollapsed;
          updateMixerChannels();
        });
        container.appendChild(expanderBtn);
      }

      // Font Awesome chevron-right (collapsed) / chevron-left (expanded) — exact same as native expander
      const iconSvg = isMixerCollapsed
        ? `<svg aria-hidden="true" focusable="false" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512" style="width:1em;height:1em"><path fill="currentColor" d="M310.6 233.4c12.5 12.5 12.5 32.8 0 45.3l-192 192c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3L242.7 256 73.4 86.6c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0l192 192z"></path></svg>`
        : `<svg aria-hidden="true" focusable="false" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512" style="width:1em;height:1em"><path fill="currentColor" d="M9.4 233.4c-12.5 12.5-12.5 32.8 0 45.3l192 192c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L77.3 256 246.6 86.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0l-192 192z"></path></svg>`;
      const title = isMixerCollapsed ? 'Expand Section' : 'Collapse Section';
      if (expanderBtn.title !== title) expanderBtn.title = title;
      if (expanderBtn.dataset.collapsed !== String(isMixerCollapsed)) {
        expanderBtn.dataset.collapsed = String(isMixerCollapsed);
        expanderBtn.innerHTML = iconSvg;
      }
    });
  }

  function setupMixerChannelsObserver() {
    const obs = new MutationObserver(updateMixerChannels);
    obs.observe(document.body, { childList: true, subtree: true });
    updateMixerChannels();
  }

  function setupFooterRemoval() {
    function removeFooter() {
      const versionEls = document.querySelectorAll('.version');
      versionEls.forEach(el => {
        el.style.setProperty('display', 'none', 'important');
      });

      // Target language select dropdown at root level if present, hide it cleanly without deleting containers
      const rootSelects = document.querySelectorAll('#app > select, #app > div > select');
      rootSelects.forEach(sel => {
        sel.style.setProperty('display', 'none', 'important');
      });
    }
    const obs = new MutationObserver(removeFooter);
    obs.observe(document.body, { childList: true, subtree: true });
    removeFooter();
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
            appWindow.setSize(new LogicalSize(1188, 713));
          } catch(e) {}
        } else if (window.resizeTo) {
          try {
            window.resizeTo(1188, 713);
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

  // Quick Mic Gain & Reworked Mic Setup Widget
  let micGainSyncInterval = null;
  function setupMicGainObserver() {
    function injectQuickGain() {
      const micSetupBtn = document.getElementById('mic_setup');
      if (!micSetupBtn) return;

      const parent = micSetupBtn.parentElement;
      if (!parent || parent.querySelector('#modern-mic-setup-card')) return;

      let currentGain = 40;
      let micType = 'Dynamic';
      if (window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
        try {
          const status = window.c.getActiveDevice().mic_status;
          micType = status.mic_type || 'Dynamic';
          currentGain = (status.mic_gains && status.mic_gains[micType] !== undefined) ? status.mic_gains[micType] : 40;
        } catch(e) {}
      }

      const widgetCard = document.createElement('div');
      widgetCard.id = 'modern-mic-setup-card';
      widgetCard.className = 'modern-mic-setup-card';

      widgetCard.innerHTML = `
        <div class="modern-widget-header">
          <span class="modern-widget-title">MIC SETUP</span>
        </div>
        <div class="modern-widget-body">
          <div class="quick-gain-header">
            <span class="quick-gain-title">GAIN</span>
            <span class="quick-gain-val" id="modern-gain-display">${currentGain} dB</span>
          </div>
          <input type="range" id="modern-gain-slider" min="0" max="72" value="${currentGain}" class="modern-gain-range" />
          <button class="modern-mic-setup-trigger" id="modern-mic-setup-trigger" title="Open Mic Setup Options">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3z"></path>
              <path d="M19 10v2a7 7 0 0 1-14 0v-2"></path>
              <line x1="12" y1="19" x2="12" y2="23"></line>
              <line x1="8" y1="23" x2="16" y2="23"></line>
            </svg>
            <span>Mic Setup</span>
          </button>
        </div>
      `;

      micSetupBtn.style.setProperty('display', 'none', 'important');
      parent.appendChild(widgetCard);

      const slider = widgetCard.querySelector('#modern-gain-slider');
      const display = widgetCard.querySelector('#modern-gain-display');
      const trigger = widgetCard.querySelector('#modern-mic-setup-trigger');

      trigger.addEventListener('click', (e) => {
        e.preventDefault();
        micSetupBtn.click();
      });

      slider.addEventListener('input', (e) => {
        const val = parseInt(e.target.value, 10);
        display.textContent = `${val} dB`;
        if (window.$ && window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
          try {
            const status = window.c.getActiveDevice().mic_status;
            const mType = status.mic_type || 'Dynamic';
            window.$.send_command(window.c.getActiveSerial(), { SetMicrophoneGain: [mType, val] });
            if (status.mic_gains) {
              status.mic_gains[mType] = val;
            }
          } catch(err) {}
        }
      });
    }

    if (!micGainSyncInterval) {
      micGainSyncInterval = setInterval(() => {
        const slider = document.getElementById('modern-gain-slider');
        const display = document.getElementById('modern-gain-display');
        if (!slider || !display) return;
        if (window.c && window.c.hasActiveDevice && window.c.hasActiveDevice()) {
          try {
            const status = window.c.getActiveDevice().mic_status;
            const mType = status.mic_type || 'Dynamic';
            const liveGain = (status.mic_gains && status.mic_gains[mType] !== undefined) ? status.mic_gains[mType] : 40;
            if (document.activeElement !== slider) {
              slider.value = liveGain;
              display.textContent = `${liveGain} dB`;
            }
          } catch(e) {}
        }
      }, 500);
    }

    const obs = new MutationObserver(injectQuickGain);
    obs.observe(document.body, { childList: true, subtree: true });
    injectQuickGain();
  }

  // Routing Visibility Preferences Management
  const ROUTING_HIDDEN_INPUTS_KEY = 'goxlr_routing_hidden_inputs';
  const ROUTING_HIDDEN_OUTPUTS_KEY = 'goxlr_routing_hidden_outputs';

  function getRoutingVisibilitySettings() {
    let hiddenInputs = [];
    let hiddenOutputs = [];
    try {
      hiddenInputs = JSON.parse(localStorage.getItem(ROUTING_HIDDEN_INPUTS_KEY) || '[]');
      hiddenOutputs = JSON.parse(localStorage.getItem(ROUTING_HIDDEN_OUTPUTS_KEY) || '[]');
    } catch(e) {}
    return { hiddenInputs, hiddenOutputs };
  }

  function saveRoutingVisibilitySettings(hiddenInputs, hiddenOutputs) {
    try {
      localStorage.setItem(ROUTING_HIDDEN_INPUTS_KEY, JSON.stringify(hiddenInputs));
      localStorage.setItem(ROUTING_HIDDEN_OUTPUTS_KEY, JSON.stringify(hiddenOutputs));
    } catch(e) {}
  }

  function applyRoutingVisibility(routingTable) {
    if (!routingTable) routingTable = document.querySelector('table[data-v-3bfabf52]');
    if (!routingTable) return;

    const { hiddenInputs, hiddenOutputs } = getRoutingVisibilitySettings();

    // 1. Join Top-Left Containers into a single unified cell with gear icon
    const topRow = routingTable.querySelector('thead tr:first-child');
    const subHeaderRow = routingTable.querySelector('thead tr.subHeader');

    if (topRow && subHeaderRow) {
      const cornerCell1 = topRow.children[0];
      const cornerCell2 = subHeaderRow.children[0];

      if (cornerCell1) {
        cornerCell1.rowSpan = 2;
        cornerCell1.colSpan = 2;
        cornerCell1.classList.remove('hidden');
        cornerCell1.classList.add('modern-routing-top-left-cell');

        if (!cornerCell1.querySelector('.modern-routing-gear-btn')) {
          cornerCell1.innerHTML = `
            <div class="modern-routing-corner-inner">
              <button class="modern-routing-gear-btn" id="modern-routing-gear-btn" title="Customize Routing Matrix" type="button">
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"/>
                  <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
                </svg>
              </button>
            </div>
          `;
          const gearBtn = cornerCell1.querySelector('#modern-routing-gear-btn');
          if (gearBtn) {
            gearBtn.addEventListener('click', (e) => {
              e.preventDefault();
              e.stopPropagation();
              openRoutingCustomizeModal();
            });
          }
        }
      }

      if (cornerCell2) {
        cornerCell2.style.setProperty('display', 'none', 'important');
      }
    }

    // 2. Filter Input Columns (Columns)
    const subHeaderThs = subHeaderRow ? Array.from(subHeaderRow.children).filter(el => {
      if (el === cornerCell1 || el === cornerCell2) return false;
      if (el.classList.contains('rotated') || el.classList.contains('hidden')) return false;
      const txt = el.textContent.trim();
      return txt !== '';
    }) : [];

    let visibleColCount = 0;

    subHeaderThs.forEach((th, colIdx) => {
      const channelName = th.textContent.trim();
      const isHidden = hiddenInputs.includes(channelName);
      if (isHidden) {
        th.style.setProperty('display', 'none', 'important');
      } else {
        th.style.removeProperty('display');
        visibleColCount++;
      }

      // Hide corresponding matrix td in every tbody row
      const tbodyRows = routingTable.querySelectorAll('tbody tr');
      tbodyRows.forEach(tr => {
        const tds = Array.from(tr.querySelectorAll('td'));
        if (tds[colIdx]) {
          if (isHidden) {
            tds[colIdx].style.setProperty('display', 'none', 'important');
          } else {
            tds[colIdx].style.removeProperty('display');
          }
        }
      });
    });

    // Find "Inputs" top header cell and update colspan
    const inputsTopTh = topRow ? Array.from(topRow.children).find(c => c !== cornerCell1 && (c.textContent.toUpperCase().includes('INPUT') || c.hasAttribute('colspan'))) : null;
    if (topRow) {
      Array.from(topRow.children).forEach(c => {
        if (c !== cornerCell1 && c !== inputsTopTh) {
          c.style.setProperty('display', 'none', 'important');
        }
      });
    }

    if (inputsTopTh) {
      inputsTopTh.colSpan = Math.max(1, visibleColCount);
      if (visibleColCount === 0) {
        inputsTopTh.style.setProperty('display', 'none', 'important');
      } else {
        inputsTopTh.style.removeProperty('display');
      }
    }

    // 3. Filter Output Rows (Rows)
    const tbodyRows = Array.from(routingTable.querySelectorAll('tbody tr'));
    const visibleRows = [];

    tbodyRows.forEach(tr => {
      const rowHeader = tr.querySelector('th:not(.rotated)');
      let rowName = '';
      if (rowHeader) {
        rowName = rowHeader.textContent.trim();
      }
      const isHidden = hiddenOutputs.includes(rowName);
      if (isHidden) {
        tr.style.setProperty('display', 'none', 'important');
      } else {
        tr.style.removeProperty('display');
        visibleRows.push(tr);
      }
    });

    // 4. Update vertical "Outputs" th.rotated header
    let rotatedTh = routingTable.querySelector('th.rotated');
    if (rotatedTh) {
      if (visibleRows.length === 0) {
        rotatedTh.style.setProperty('display', 'none', 'important');
      } else {
        rotatedTh.style.removeProperty('display');
        rotatedTh.rowSpan = visibleRows.length;
        const firstVisibleRow = visibleRows[0];
        if (firstVisibleRow && rotatedTh.parentElement !== firstVisibleRow) {
          firstVisibleRow.insertBefore(rotatedTh, firstVisibleRow.firstChild);
        }
      }
    }
  }

  function openRoutingCustomizeModal() {
    const routingTable = document.querySelector('table[data-v-3bfabf52]');
    if (!routingTable) return;

    // Extract current input channels from subHeader
    const topRow = routingTable.querySelector('thead tr:first-child');
    const subHeaderRow = routingTable.querySelector('thead tr.subHeader');
    const cornerCell1 = topRow ? topRow.children[0] : null;
    const cornerCell2 = subHeaderRow ? subHeaderRow.children[0] : null;

    const inputThs = subHeaderRow ? Array.from(subHeaderRow.children).filter(el => {
      if (el === cornerCell1 || el === cornerCell2) return false;
      if (el.classList.contains('rotated') || el.classList.contains('hidden')) return false;
      const txt = el.textContent.trim();
      return txt !== '';
    }) : [];
    const inputChannels = inputThs.map(th => th.textContent.trim()).filter(Boolean);

    // Extract current output channels from tbody rows
    const tbodyRows = Array.from(routingTable.querySelectorAll('tbody tr'));
    const outputChannels = [];
    tbodyRows.forEach(tr => {
      const rh = tr.querySelector('th:not(.rotated)');
      if (rh) {
        const name = rh.textContent.trim();
        if (name && !outputChannels.includes(name)) {
          outputChannels.push(name);
        }
      }
    });

    let { hiddenInputs, hiddenOutputs } = getRoutingVisibilitySettings();

    // Create or locate Modal overlay
    let modalOverlay = document.getElementById('modern-routing-customize-modal');
    if (!modalOverlay) {
      modalOverlay = document.createElement('div');
      modalOverlay.id = 'modern-routing-customize-modal';
      modalOverlay.className = 'modern-modal-overlay';
      document.body.appendChild(modalOverlay);
    }

    const renderModalContent = () => {
      modalOverlay.innerHTML = `
        <div class="modern-modal-card mini-routing-modal-card">
          <div class="modern-modal-header">
            <div class="modern-modal-title-wrap">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"/>
                <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
              </svg>
              <h3 class="modern-modal-title">CUSTOMIZE ROUTING MATRIX</h3>
            </div>
            <button class="modern-modal-close-btn" id="mini-routing-close-x" type="button">&times;</button>
          </div>
          <p class="mini-routing-hint">Click any column (input) or row (output) to toggle visibility in the routing matrix. Hidden items are greyed out.</p>

          <div class="mini-routing-container">
            <table class="mini-routing-grid">
              <thead>
                <tr>
                  <th rowspan="2" class="mini-grid-corner-cell">
                    <span class="mini-grid-corner-label">GRID</span>
                  </th>
                  <th colspan="${inputChannels.length}" class="mini-grid-top-title">INPUT CHANNELS</th>
                </tr>
                <tr class="mini-grid-subheader-row">
                  ${inputChannels.map(inp => {
                    const isHidden = hiddenInputs.includes(inp);
                    return `
                      <th>
                        <button type="button" class="mini-grid-btn mini-grid-col-btn ${isHidden ? 'is-hidden' : 'is-visible'}" data-channel="${inp}">
                          <span>${inp}</span>
                        </button>
                      </th>
                    `;
                  }).join('')}
                </tr>
              </thead>
              <tbody>
                ${outputChannels.map(outp => {
                  const isOutHidden = hiddenOutputs.includes(outp);
                  return `
                    <tr>
                      <th>
                        <button type="button" class="mini-grid-btn mini-grid-row-btn ${isOutHidden ? 'is-hidden' : 'is-visible'}" data-channel="${outp}">
                          <span>${outp}</span>
                        </button>
                      </th>
                      ${inputChannels.map(inp => {
                        const isInpHidden = hiddenInputs.includes(inp);
                        const isCellHidden = isInpHidden || isOutHidden;
                        return `
                          <td>
                            <div class="mini-grid-cell ${isCellHidden ? 'is-hidden' : 'is-visible'}" data-input="${inp}" data-output="${outp}"></div>
                          </td>
                        `;
                      }).join('')}
                    </tr>
                  `;
                }).join('')}
              </tbody>
            </table>
          </div>

          <div class="modern-modal-footer">
            <button type="button" class="modern-btn-secondary" id="mini-routing-show-all">Show All Channels</button>
            <button type="button" class="modern-btn-primary" id="mini-routing-done">Done</button>
          </div>
        </div>
      `;

      // Wire up event listeners inside modal
      const colBtns = modalOverlay.querySelectorAll('.mini-grid-col-btn');
      colBtns.forEach(btn => {
        btn.addEventListener('click', (e) => {
          e.preventDefault();
          const channel = btn.getAttribute('data-channel');
          if (hiddenInputs.includes(channel)) {
            hiddenInputs = hiddenInputs.filter(x => x !== channel);
          } else {
            hiddenInputs.push(channel);
          }
          saveRoutingVisibilitySettings(hiddenInputs, hiddenOutputs);
          applyRoutingVisibility(routingTable);
          renderModalContent();
        });
      });

      const gridCells = modalOverlay.querySelectorAll('.mini-grid-cell');
      gridCells.forEach(cell => {
        cell.addEventListener('click', (e) => {
          e.preventDefault();
          const inp = cell.getAttribute('data-input');
          const outp = cell.getAttribute('data-output');
          const isInpHidden = hiddenInputs.includes(inp);
          const isOutHidden = hiddenOutputs.includes(outp);

          if (isInpHidden && isOutHidden) {
            hiddenInputs = hiddenInputs.filter(x => x !== inp);
            hiddenOutputs = hiddenOutputs.filter(x => x !== outp);
          } else if (isInpHidden) {
            hiddenInputs = hiddenInputs.filter(x => x !== inp);
          } else if (isOutHidden) {
            hiddenOutputs = hiddenOutputs.filter(x => x !== outp);
          } else {
            hiddenInputs.push(inp);
          }
          saveRoutingVisibilitySettings(hiddenInputs, hiddenOutputs);
          applyRoutingVisibility(routingTable);
          renderModalContent();
        });
      });

      const rowBtns = modalOverlay.querySelectorAll('.mini-grid-row-btn');
      rowBtns.forEach(btn => {
        btn.addEventListener('click', (e) => {
          e.preventDefault();
          const channel = btn.getAttribute('data-channel');
          if (hiddenOutputs.includes(channel)) {
            hiddenOutputs = hiddenOutputs.filter(x => x !== channel);
          } else {
            hiddenOutputs.push(channel);
          }
          saveRoutingVisibilitySettings(hiddenInputs, hiddenOutputs);
          applyRoutingVisibility(routingTable);
          renderModalContent();
        });
      });

      const showAllBtn = modalOverlay.querySelector('#mini-routing-show-all');
      if (showAllBtn) {
        showAllBtn.addEventListener('click', (e) => {
          e.preventDefault();
          hiddenInputs = [];
          hiddenOutputs = [];
          saveRoutingVisibilitySettings(hiddenInputs, hiddenOutputs);
          applyRoutingVisibility(routingTable);
          renderModalContent();
        });
      }

      const closeX = modalOverlay.querySelector('#mini-routing-close-x');
      const doneBtn = modalOverlay.querySelector('#mini-routing-done');
      const closeModal = () => {
        modalOverlay.style.display = 'none';
      };
      if (closeX) closeX.addEventListener('click', closeModal);
      if (doneBtn) doneBtn.addEventListener('click', closeModal);
    };

    renderModalContent();
    modalOverlay.style.display = 'flex';

    modalOverlay.onclick = (e) => {
      if (e.target === modalOverlay) {
        modalOverlay.style.display = 'none';
      }
    };
  }

  // Routing Table Precision Crosshair & Matrix Polish
  function setupRoutingTableObserver() {
    const attachToTable = () => {
      const routingTable = document.querySelector('table[data-v-3bfabf52]');
      if (!routingTable) return;

      applyRoutingVisibility(routingTable);

      if (routingTable._modernEnhanced) return;
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
