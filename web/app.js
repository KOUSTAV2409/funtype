/* ==============================================================================
   FunType - Web Simulator & Audio Synthesizer Engine
   Pure Web Audio API & Canvas Particle System
   ============================================================================== */

// --- 1. Web Audio Synthesizer (Zero External Audio Files) ---
class MechanicalAudioEngine {
  constructor() {
    this.ctx = null;
    this.enabled = true;
    this.profile = 'holypanda'; // 'holypanda', 'blue', 'creamy', 'bubble', 'silent'
  }

  init() {
    if (!this.ctx) {
      const AudioCtx = window.AudioContext || window.webkitAudioContext;
      if (AudioCtx) {
        this.ctx = new AudioCtx();
      }
    }
    if (this.ctx && this.ctx.state === 'suspended') {
      this.ctx.resume();
    }
  }

  playKey() {
    if (!this.enabled || this.profile === 'silent') return;
    this.init();
    if (!this.ctx) return;

    const t = this.ctx.currentTime;

    switch (this.profile) {
      case 'holypanda':
        this.synthesizeHolyPanda(t);
        break;
      case 'blue':
        this.synthesizeCherryBlue(t);
        break;
      case 'creamy':
        this.synthesizeCreamyLinear(t);
        break;
      case 'bubble':
        this.synthesizeBubblePop(t);
        break;
    }
  }

  playExplosion() {
    if (!this.enabled || this.profile === 'silent') return;
    this.init();
    if (!this.ctx) return;

    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();

    osc.type = 'triangle';
    osc.frequency.setValueAtTime(160, t);
    osc.frequency.exponentialRampToValueAtTime(35, t + 0.18);

    gain.gain.setValueAtTime(0.25, t);
    gain.gain.exponentialRampToValueAtTime(0.001, t + 0.18);

    osc.connect(gain);
    gain.connect(this.ctx.destination);

    osc.start(t);
    osc.stop(t + 0.18);
  }

  // Holy Panda: Deep body + tactile snap
  synthesizeHolyPanda(t) {
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    const filter = this.ctx.createBiquadFilter();

    osc.type = 'triangle';
    const baseFreq = 140 + Math.random() * 25;
    osc.frequency.setValueAtTime(baseFreq, t);
    osc.frequency.exponentialRampToValueAtTime(45, t + 0.05);

    filter.type = 'lowpass';
    filter.frequency.setValueAtTime(450, t);

    gain.gain.setValueAtTime(0.28, t);
    gain.gain.exponentialRampToValueAtTime(0.001, t + 0.055);

    osc.connect(filter);
    filter.connect(gain);
    gain.connect(this.ctx.destination);

    osc.start(t);
    osc.stop(t + 0.06);

    // High snap click
    const snapOsc = this.ctx.createOscillator();
    const snapGain = this.ctx.createGain();
    snapOsc.type = 'sine';
    snapOsc.frequency.setValueAtTime(1100 + Math.random() * 200, t);
    snapGain.gain.setValueAtTime(0.08, t);
    snapGain.gain.exponentialRampToValueAtTime(0.001, t + 0.015);

    snapOsc.connect(snapGain);
    snapGain.connect(this.ctx.destination);
    snapOsc.start(t);
    snapOsc.stop(t + 0.02);
  }

  // Cherry Blue: High crisp click
  synthesizeCherryBlue(t) {
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();

    osc.type = 'sawtooth';
    osc.frequency.setValueAtTime(1800 + Math.random() * 300, t);
    osc.frequency.exponentialRampToValueAtTime(300, t + 0.03);

    gain.gain.setValueAtTime(0.18, t);
    gain.gain.exponentialRampToValueAtTime(0.001, t + 0.035);

    osc.connect(gain);
    gain.connect(this.ctx.destination);

    osc.start(t);
    osc.stop(t + 0.04);
  }

  // Creamy Linear: Soft dampened pop
  synthesizeCreamyLinear(t) {
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    const filter = this.ctx.createBiquadFilter();

    osc.type = 'sine';
    osc.frequency.setValueAtTime(260 + Math.random() * 20, t);
    osc.frequency.exponentialRampToValueAtTime(80, t + 0.04);

    filter.type = 'lowpass';
    filter.frequency.setValueAtTime(320, t);

    gain.gain.setValueAtTime(0.22, t);
    gain.gain.exponentialRampToValueAtTime(0.001, t + 0.045);

    osc.connect(filter);
    filter.connect(gain);
    gain.connect(this.ctx.destination);

    osc.start(t);
    osc.stop(t + 0.05);
  }

  // Bubble Pop: Harmonic upward water-droplet sweep
  synthesizeBubblePop(t) {
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();

    osc.type = 'sine';
    const startF = 350 + Math.random() * 80;
    osc.frequency.setValueAtTime(startF, t);
    osc.frequency.exponentialRampToValueAtTime(startF * 2.2, t + 0.06);

    gain.gain.setValueAtTime(0.22, t);
    gain.gain.exponentialRampToValueAtTime(0.001, t + 0.065);

    osc.connect(gain);
    gain.connect(this.ctx.destination);

    osc.start(t);
    osc.stop(t + 0.07);
  }
}

const audio = new MechanicalAudioEngine();

// --- 2. Interactive Simulator Controller ---
class SimulatorApp {
  constructor() {
    this.activeMode = 'shredder'; // 'shredder', 'zen', 'sprint'

    // Shredder state
    this.stressors = [];
    this.stressLevel = 85;
    this.score = 0;
    this.particles = [];
    this.shredderCanvas = document.getElementById('shredder-canvas');
    this.ctx = this.shredderCanvas ? this.shredderCanvas.getContext('2d') : null;
    this.lastSpawn = 0;
    this.stressWords = [
      'merge conflict', 'segfault', 'prod incident', 'infinite loop',
      'jira sprint', 'emergency call', 'memory leak', 'null pointer',
      'git push -f', '404 not found', 'unpaid overtime', 'scope creep',
      'unresolved blocker', 'corrupted cache', 'stack overflow'
    ];

    // Sprint state
    this.sprintQuote = "Clean code reads like well written prose. It never obscures the designer intent, but rather is full of crisp abstractions and straightforward lines of control.";
    this.sprintIndex = 0;
    this.sprintMistakes = 0;
    this.sprintStartTime = null;
    this.sprintTimerInterval = null;
    this.sprintDuration = 30;
    this.sprintTimeLeft = 30;
    this.sprintActive = false;

    // Zen state
    this.zenPhrases = [
      "Breathe in tranquility. Release the deadline tension.",
      "The obstacle in the code becomes the way forward.",
      "Calm mind, steady hands, effortless cadence.",
      "You are not your compiler errors; you are the architect.",
      "Let each keystroke be deliberate and without haste."
    ];
    this.zenPhraseIndex = 0;
    this.zenCharIndex = 0;

    this.init();
  }

  init() {
    this.bindEvents();
    this.setupShredderCanvas();
    this.renderSprintText();
    this.renderZenText();
    this.startShredderLoop();
  }

  bindEvents() {
    // Mode tabs
    const modeTabs = document.querySelectorAll('.mode-tab-btn');
    modeTabs.forEach(btn => {
      btn.addEventListener('click', (e) => {
        const mode = btn.dataset.mode;
        this.switchMode(mode);
      });
    });

    // Sound controls
    const soundToggle = document.getElementById('sound-toggle-btn');
    if (soundToggle) {
      soundToggle.addEventListener('click', () => {
        audio.init();
        audio.enabled = !audio.enabled;
        soundToggle.textContent = audio.enabled ? '🔊 Sound: ON' : '🔇 Sound: OFF';
      });
    }

    const soundSelect = document.getElementById('sound-profile-select');
    if (soundSelect) {
      soundSelect.addEventListener('change', (e) => {
        audio.init();
        audio.profile = e.target.value;
        audio.playKey();
      });
    }

    // Audio cards preview buttons in Audio section
    const audioCards = document.querySelectorAll('.audio-card');
    audioCards.forEach(card => {
      card.addEventListener('click', () => {
        audio.init();
        const prof = card.dataset.profile;
        if (prof) {
          audio.profile = prof;
          if (soundSelect) soundSelect.value = prof;
          audio.playKey();
        }
      });
    });

    // Shredder typing input
    const shredderInput = document.getElementById('shredder-input');
    if (shredderInput) {
      shredderInput.addEventListener('input', (e) => {
        audio.playKey();
        this.checkShredderMatch(e.target.value);
      });
      shredderInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
          shredderInput.value = '';
        }
      });
    }

    // Sprint Typer Input
    const sprintTyper = document.getElementById('sprint-typer');
    if (sprintTyper) {
      sprintTyper.addEventListener('input', (e) => {
        this.handleSprintInput(e);
      });
      // Focus simulator on click anywhere inside sprint body
      const sprintView = document.getElementById('sprint-view');
      if (sprintView) {
        sprintView.addEventListener('click', () => sprintTyper.focus());
      }
    }

    // Zen Typer Input
    const zenTyper = document.getElementById('zen-typer');
    if (zenTyper) {
      zenTyper.addEventListener('input', (e) => {
        this.handleZenInput(e);
      });
      const zenView = document.getElementById('zen-view');
      if (zenView) {
        zenView.addEventListener('click', () => zenTyper.focus());
      }
    }

    // Rematch / Reset buttons
    const restartBtn = document.getElementById('btn-simulator-restart');
    if (restartBtn) {
      restartBtn.addEventListener('click', () => {
        this.resetCurrentMode();
      });
    }
  }

  switchMode(mode) {
    this.activeMode = mode;
    document.querySelectorAll('.mode-tab-btn').forEach(b => {
      b.classList.toggle('active', b.dataset.mode === mode);
    });

    document.getElementById('shredder-view').classList.toggle('active', mode === 'shredder');
    document.getElementById('zen-view').classList.toggle('active', mode === 'zen');
    document.getElementById('sprint-view').classList.toggle('active', mode === 'sprint');

    if (mode === 'shredder') {
      const inp = document.getElementById('shredder-input');
      if (inp) inp.focus();
    } else if (mode === 'sprint') {
      const inp = document.getElementById('sprint-typer');
      if (inp) inp.focus();
      this.resetSprint();
    } else if (mode === 'zen') {
      const inp = document.getElementById('zen-typer');
      if (inp) inp.focus();
    }
  }

  resetCurrentMode() {
    if (this.activeMode === 'shredder') {
      this.stressors = [];
      this.stressLevel = 85;
      this.score = 0;
      this.updateShredderHud();
      const inp = document.getElementById('shredder-input');
      if (inp) { inp.value = ''; inp.focus(); }
    } else if (this.activeMode === 'sprint') {
      this.resetSprint();
    } else if (this.activeMode === 'zen') {
      this.zenCharIndex = 0;
      this.renderZenText();
      const inp = document.getElementById('zen-typer');
      if (inp) { inp.value = ''; inp.focus(); }
    }
  }

  // --- Shredder Mode Mechanics ---
  setupShredderCanvas() {
    if (!this.shredderCanvas) return;
    const resize = () => {
      const rect = this.shredderCanvas.parentElement.getBoundingClientRect();
      this.shredderCanvas.width = rect.width;
      this.shredderCanvas.height = rect.height;
    };
    resize();
    window.addEventListener('resize', resize);
  }

  startShredderLoop() {
    const loop = (timestamp) => {
      if (this.activeMode === 'shredder' && this.ctx) {
        this.updateShredder(timestamp);
        this.renderShredder();
      }
      requestAnimationFrame(loop);
    };
    requestAnimationFrame(loop);
  }

  updateShredder(time) {
    if (time - this.lastSpawn > 1400 && this.stressors.length < 5) {
      this.lastSpawn = time;
      const word = this.stressWords[Math.floor(Math.random() * this.stressWords.length)];
      const x = 50 + Math.random() * (this.shredderCanvas.width - 200);
      this.stressors.push({
        text: word,
        x: x,
        y: 20,
        speed: 0.5 + Math.random() * 0.4,
        color: '#f38ba8'
      });
    }

    // Move stressors
    for (let i = this.stressors.length - 1; i >= 0; i--) {
      const s = this.stressors[i];
      s.y += s.speed;
      if (s.y > this.shredderCanvas.height - 70) {
        // reached bottom - add a bit of stress back
        this.stressLevel = Math.min(100, this.stressLevel + 4);
        this.stressors.splice(i, 1);
        this.updateShredderHud();
      }
    }

    // Update particles
    for (let i = this.particles.length - 1; i >= 0; i--) {
      const p = this.particles[i];
      p.x += p.vx;
      p.y += p.vy;
      p.alpha -= 0.025;
      if (p.alpha <= 0) {
        this.particles.splice(i, 1);
      }
    }
  }

  renderShredder() {
    this.ctx.clearRect(0, 0, this.shredderCanvas.width, this.shredderCanvas.height);

    // Draw stressors
    this.ctx.font = '15px "JetBrains Mono", monospace';
    this.ctx.textAlign = 'left';

    this.stressors.forEach(s => {
      const textWidth = this.ctx.measureText(s.text).width;
      
      // Capsule pill background
      this.ctx.fillStyle = 'rgba(24, 24, 37, 0.85)';
      this.ctx.strokeStyle = '#f38ba8';
      this.ctx.lineWidth = 1.5;
      this.ctx.beginPath();
      this.ctx.roundRect(s.x - 8, s.y - 18, textWidth + 16, 26, 6);
      this.ctx.fill();
      this.ctx.stroke();

      // Text
      this.ctx.fillStyle = '#f38ba8';
      this.ctx.fillText(s.text, s.x, s.y);
    });

    // Draw explosion particles
    this.particles.forEach(p => {
      this.ctx.fillStyle = p.color;
      this.ctx.globalAlpha = p.alpha;
      this.ctx.beginPath();
      this.ctx.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
      this.ctx.fill();
    });
    this.ctx.globalAlpha = 1.0;
  }

  checkShredderMatch(query) {
    const clean = query.trim().toLowerCase();
    for (let i = 0; i < this.stressors.length; i++) {
      if (this.stressors[i].text.toLowerCase() === clean) {
        const s = this.stressors[i];
        this.spawnExplosion(s.x + 40, s.y);
        audio.playExplosion();
        this.stressors.splice(i, 1);
        this.score += 150;
        this.stressLevel = Math.max(0, this.stressLevel - 12);
        this.updateShredderHud();
        const inp = document.getElementById('shredder-input');
        if (inp) inp.value = '';
        break;
      }
    }
  }

  spawnExplosion(x, y) {
    const colors = ['#a6e3a1', '#cba6f7', '#89b4fa', '#fab387', '#f5c2e7'];
    for (let i = 0; i < 30; i++) {
      const angle = Math.random() * Math.PI * 2;
      const speed = 1.5 + Math.random() * 4;
      this.particles.push({
        x: x,
        y: y,
        vx: Math.cos(angle) * speed,
        vy: Math.sin(angle) * speed,
        radius: 2 + Math.random() * 2.5,
        alpha: 1.0,
        color: colors[Math.floor(Math.random() * colors.length)]
      });
    }
  }

  updateShredderHud() {
    const fill = document.getElementById('stress-fill');
    const val = document.getElementById('stress-val');
    const scoreVal = document.getElementById('shredder-score-val');
    if (fill) fill.style.width = `${this.stressLevel}%`;
    if (val) val.textContent = `${this.stressLevel}%`;
    if (scoreVal) scoreVal.textContent = this.score;
  }

  // --- Speed Sprint Mechanics ---
  renderSprintText() {
    const container = document.getElementById('sprint-text-display');
    if (!container) return;
    let html = '';
    for (let i = 0; i < this.sprintQuote.length; i++) {
      const char = this.sprintQuote[i];
      let cls = '';
      if (i < this.sprintIndex) {
        cls = 'char-correct';
      } else if (i === this.sprintIndex) {
        cls = 'char-current';
      }
      html += `<span class="${cls}">${char}</span>`;
    }
    container.innerHTML = html;
  }

  handleSprintInput(e) {
    const typed = e.target.value;
    audio.playKey();

    if (!this.sprintActive) {
      this.startSprintTimer();
    }

    if (typed.length > 0) {
      const lastChar = typed[typed.length - 1];
      const targetChar = this.sprintQuote[this.sprintIndex];

      if (lastChar === targetChar) {
        this.sprintIndex++;
      } else {
        this.sprintMistakes++;
      }
      this.renderSprintText();
      this.updateSprintStats();

      if (this.sprintIndex >= this.sprintQuote.length) {
        this.finishSprint();
      }
    }
    e.target.value = '';
  }

  startSprintTimer() {
    this.sprintActive = true;
    this.sprintStartTime = Date.now();
    this.sprintTimeLeft = this.sprintDuration;

    this.sprintTimerInterval = setInterval(() => {
      this.sprintTimeLeft--;
      const timerBadge = document.getElementById('sprint-timer-val');
      if (timerBadge) timerBadge.textContent = `${this.sprintTimeLeft}s`;

      if (this.sprintTimeLeft <= 0) {
        this.finishSprint();
      }
    }, 1000);
  }

  updateSprintStats() {
    if (!this.sprintStartTime) return;
    const elapsedMinutes = (Date.now() - this.sprintStartTime) / 60000;
    const words = this.sprintIndex / 5;
    const wpm = elapsedMinutes > 0 ? Math.round(words / elapsedMinutes) : 0;
    const totalTyped = this.sprintIndex + this.sprintMistakes;
    const acc = totalTyped > 0 ? Math.round((this.sprintIndex / totalTyped) * 100) : 100;

    const wpmElem = document.getElementById('sprint-wpm-val');
    const accElem = document.getElementById('sprint-acc-val');
    if (wpmElem) wpmElem.textContent = wpm;
    if (accElem) accElem.textContent = `${acc}%`;
  }

  finishSprint() {
    clearInterval(this.sprintTimerInterval);
    this.sprintActive = false;
    audio.playExplosion();
    const timerBadge = document.getElementById('sprint-timer-val');
    if (timerBadge) timerBadge.textContent = 'DONE!';
  }

  resetSprint() {
    clearInterval(this.sprintTimerInterval);
    this.sprintActive = false;
    this.sprintIndex = 0;
    this.sprintMistakes = 0;
    this.sprintStartTime = null;
    this.sprintTimeLeft = this.sprintDuration;

    const timerBadge = document.getElementById('sprint-timer-val');
    const wpmElem = document.getElementById('sprint-wpm-val');
    const accElem = document.getElementById('sprint-acc-val');
    if (timerBadge) timerBadge.textContent = `${this.sprintDuration}s`;
    if (wpmElem) wpmElem.textContent = '0';
    if (accElem) accElem.textContent = '100%';

    this.renderSprintText();
    const inp = document.getElementById('sprint-typer');
    if (inp) { inp.value = ''; inp.focus(); }
  }

  // --- Zen Flow Mechanics ---
  renderZenText() {
    const container = document.getElementById('zen-text-display');
    if (!container) return;
    const currentPhrase = this.zenPhrases[this.zenPhraseIndex];
    let html = '';
    for (let i = 0; i < currentPhrase.length; i++) {
      const char = currentPhrase[i];
      let cls = '';
      if (i < this.zenCharIndex) {
        cls = 'char-correct';
      } else if (i === this.zenCharIndex) {
        cls = 'char-current';
      }
      html += `<span class="${cls}">${char}</span>`;
    }
    container.innerHTML = html;
  }

  handleZenInput(e) {
    const typed = e.target.value;
    audio.playKey();
    const currentPhrase = this.zenPhrases[this.zenPhraseIndex];

    if (typed.length > 0) {
      const lastChar = typed[typed.length - 1];
      if (lastChar === currentPhrase[this.zenCharIndex]) {
        this.zenCharIndex++;
      }
      this.renderZenText();

      if (this.zenCharIndex >= currentPhrase.length) {
        this.zenPhraseIndex = (this.zenPhraseIndex + 1) % this.zenPhrases.length;
        this.zenCharIndex = 0;
        setTimeout(() => this.renderZenText(), 300);
      }
    }
    e.target.value = '';
  }
}

// --- 3. UI Helpers: Copy Command Tabs, Themes, Hotkeys ---
document.addEventListener('DOMContentLoaded', () => {
  // Initialize Simulator
  const app = new SimulatorApp();

  // Install command switcher
  const installCommands = {
    curl: "curl -sSL https://raw.githubusercontent.com/KOUSTAV2409/funtype/main/install.sh | bash",
    clone: "git clone https://github.com/KOUSTAV2409/funtype.git && cd funtype && ./install.sh",
    hyprland: "./install.sh # Auto-configures SUPER+Y & floating rules in ~/.config/hypr/"
  };

  const tabs = document.querySelectorAll('.install-tab');
  const codeElem = document.getElementById('install-cmd-text');
  const copyBtn = document.getElementById('install-copy-btn');

  tabs.forEach(tab => {
    tab.addEventListener('click', () => {
      tabs.forEach(t => t.classList.remove('active'));
      tab.classList.add('active');
      const target = tab.dataset.target;
      if (codeElem && installCommands[target]) {
        codeElem.textContent = installCommands[target];
      }
    });
  });

  if (copyBtn && codeElem) {
    copyBtn.addEventListener('click', async () => {
      try {
        await navigator.clipboard.writeText(codeElem.textContent.trim());
        copyBtn.textContent = '✓ Copied!';
        copyBtn.classList.add('copied');
        setTimeout(() => {
          copyBtn.textContent = 'Copy';
          copyBtn.classList.remove('copied');
        }, 2000);
      } catch (err) {
        console.error('Clipboard copy failed', err);
      }
    });
  }

  // Theme Switcher
  const themeBtn = document.getElementById('theme-toggle-btn');
  const themes = ['catppuccin', 'tokyo-night', 'nord', 'rose-pine'];
  let currentThemeIdx = 0;

  if (themeBtn) {
    themeBtn.addEventListener('click', () => {
      currentThemeIdx = (currentThemeIdx + 1) % themes.length;
      const theme = themes[currentThemeIdx];
      document.documentElement.setAttribute('data-theme', theme);
      themeBtn.innerHTML = `🎨 Theme: ${theme.replace('-', ' ').toUpperCase()}`;
    });
  }
});
