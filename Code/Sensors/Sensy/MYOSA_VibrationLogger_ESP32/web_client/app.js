// Web Bluetooth client for MYOSA-VibeLogger
// No external JS dependencies.

const SERVICE_UUID = '7b4e3a0d-6f1b-4d1a-9c62-4c6b8a2e7a10';
const DATA_CHAR_UUID = '7b4e3a0d-6f1b-4d1a-9c62-4c6b8a2e7a11';
const TIME_CHAR_UUID = '7b4e3a0d-6f1b-4d1a-9c62-4c6b8a2e7a12';

const WINDOW_MS = 500; // must match firmware

const LEVEL_LABELS = ['Still', 'Light', 'Caution', 'Partial', 'Severe'];

const btnConnect = document.getElementById('btnConnect');
const btnDisconnect = document.getElementById('btnDisconnect');
const btnDownloadSamples = document.getElementById('btnDownloadSamples');
const btnDownloadEvents = document.getElementById('btnDownloadEvents');

const statusDot = document.getElementById('statusDot');
const statusText = document.getElementById('statusText');
const browserHint = document.getElementById('browserHint');

const vibeText = document.getElementById('vibeText');
const vibeSub = document.getElementById('vibeSub');
const tempText = document.getElementById('tempText');
const timeText = document.getElementById('timeText');
const meterFill = document.getElementById('meterFill');
const levelBadge = document.getElementById('levelBadge');
const peakBadge = document.getElementById('peakBadge');

const samplesBody = document.getElementById('samplesBody');
const eventsBody = document.getElementById('eventsBody');
const eventCount = document.getElementById('eventCount');

const canvas = document.getElementById('chart');
const ctx = canvas.getContext('2d');

let bleDevice = null;
let gattServer = null;
let dataChar = null;
let timeChar = null;

const state = {
  samples: [],
  events: [],
  currentEvent: null,
  belowCount: 0,
  sampleIndex: 0,
  connectedAtMs: 0,
};

function setStatus(connected, text) {
  statusDot.style.background = connected ? '#3ddc97' : '#666';
  statusText.textContent = text;
}

function detectBrowserSupport() {
  if (!('bluetooth' in navigator)) {
    browserHint.textContent = 'Web Bluetooth not supported';
    browserHint.style.borderColor = 'rgba(255,107,107,0.5)';
    return false;
  }
  browserHint.textContent = 'Web Bluetooth ready';
  return true;
}

function fmtTime(d) {
  if (!d) return '—';
  // Keep it readable for humans.
  return d.toLocaleString(undefined, {
    hour12: false,
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
}

function fmtDuration(ms) {
  if (!isFinite(ms) || ms < 0) return '—';
  if (ms < 1000) return `${ms} ms`;
  const s = ms / 1000;
  if (s < 60) return `${s.toFixed(1)} s`;
  const m = Math.floor(s / 60);
  const rem = s - m * 60;
  return `${m}m ${rem.toFixed(0)}s`;
}

function resizeCanvasToDisplaySize(c) {
  const dpr = window.devicePixelRatio || 1;
  const rect = c.getBoundingClientRect();
  const w = Math.max(1, Math.round(rect.width * dpr));
  const h = Math.max(1, Math.round(rect.height * dpr));
  if (c.width !== w || c.height !== h) {
    c.width = w;
    c.height = h;
  }
}

function drawChart() {
  resizeCanvasToDisplaySize(canvas);

  const w = canvas.width;
  const h = canvas.height;

  ctx.clearRect(0, 0, w, h);

  // Background
  ctx.fillStyle = 'rgba(0,0,0,0.08)';
  ctx.fillRect(0, 0, w, h);

  const points = state.samples.slice(-240); // ~2 minutes at 0.5s
  if (points.length < 2) {
    ctx.fillStyle = 'rgba(232,236,255,0.6)';
    ctx.font = `${Math.round(12 * (window.devicePixelRatio || 1))}px system-ui`;
    ctx.fillText('Waiting for data…', 16, 24);
    return;
  }

  const rmsValues = points.map(p => p.rms_g).filter(v => Number.isFinite(v));
  const maxRms = rmsValues.length ? Math.max(...rmsValues) : 0;
  const yMax = Math.max(0.05, maxRms * 1.2);

  const padL = 42;
  const padR = 12;
  const padT = 12;
  const padB = 26;

  // Axes
  ctx.strokeStyle = 'rgba(232,236,255,0.14)';
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(padL, padT);
  ctx.lineTo(padL, h - padB);
  ctx.lineTo(w - padR, h - padB);
  ctx.stroke();

  // Y labels
  ctx.fillStyle = 'rgba(232,236,255,0.55)';
  ctx.font = `${Math.round(12 * (window.devicePixelRatio || 1))}px system-ui`;
  ctx.fillText('g RMS', 8, 18);

  const ticks = 4;
  for (let i = 0; i <= ticks; i++) {
    const v = (yMax * i) / ticks;
    const y = padT + (h - padT - padB) * (1 - i / ticks);
    ctx.strokeStyle = 'rgba(232,236,255,0.06)';
    ctx.beginPath();
    ctx.moveTo(padL, y);
    ctx.lineTo(w - padR, y);
    ctx.stroke();

    ctx.fillStyle = 'rgba(232,236,255,0.45)';
    ctx.fillText(v.toFixed(2), 6, y + 4);
  }

  // Line
  ctx.strokeStyle = 'rgba(61,220,151,0.95)';
  ctx.lineWidth = 2;
  ctx.beginPath();

  for (let i = 0; i < points.length; i++) {
    const x = padL + (w - padL - padR) * (i / (points.length - 1));
    const v = Number.isFinite(points[i].rms_g) ? points[i].rms_g : 0;
    const y = padT + (h - padT - padB) * (1 - (v / yMax));
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  }
  ctx.stroke();
}

function parsePacket(dv) {
  // Firmware struct (little-endian):
  // uint32 t_s
  // uint16 t_ms
  // uint16 rms_mg
  // uint16 peak_mg
  // uint16 shake_score
  // int16  temp_c_x100
  // uint8  level

  if (dv.byteLength < 15) {
    return null;
  }

  let o = 0;
  const t_s = dv.getUint32(o, true); o += 4;
  const t_ms = dv.getUint16(o, true); o += 2;
  const rms_mg = dv.getUint16(o, true); o += 2;
  const peak_mg = dv.getUint16(o, true); o += 2;
  const shake_score = dv.getUint16(o, true); o += 2;
  const temp_c_x100 = dv.getInt16(o, true); o += 2;
  const level = dv.getUint8(o); o += 1;

  const sensor_ok = !(rms_mg === 65535 && peak_mg === 65535);
  const temp_ok = (temp_c_x100 !== -32768);

  const rms_g = sensor_ok ? (rms_mg / 1000) : NaN;
  const peak_g = sensor_ok ? (peak_mg / 1000) : NaN;
  const temp_c = temp_ok ? (temp_c_x100 / 100) : NaN;

  let time = null;
  if (t_s !== 0) {
    time = new Date(t_s * 1000 + t_ms);
  } else {
    // If device isn't time-synced, build a relative timestamp since connection.
    const relMs = state.sampleIndex * WINDOW_MS;
    time = new Date(state.connectedAtMs + relMs);
  }

  return { t_s, t_ms, time, rms_g, peak_g, temp_c, shake_score, level, sensor_ok, temp_ok };
}

function updateDashboard(p) {
  const label = LEVEL_LABELS[p.level] ?? '—';

  if (!p.sensor_ok) {
    vibeText.textContent = '—';
    vibeSub.textContent = 'Sensor missing (MPU6050 not detected)';
    levelBadge.textContent = 'NoSensor';
    peakBadge.textContent = '—';
    meterFill.style.width = '0%';
  } else {
    vibeText.textContent = `${p.shake_score} / 1000`;
    vibeSub.textContent = `Level: ${label} · RMS: ${p.rms_g.toFixed(3)} g · Peak: ${p.peak_g.toFixed(3)} g`;
    levelBadge.textContent = `Level ${p.level} · ${label}`;
    peakBadge.textContent = `Peak ${p.peak_g.toFixed(3)} g`;

    // Meter scale: 0..1000 score maps to 0..100%.
    const pct = Math.min(100, Math.max(0, (p.shake_score / 1000) * 100));
    meterFill.style.width = `${pct}%`;
  }

  tempText.textContent = p.temp_ok ? `${p.temp_c.toFixed(2)} °C` : '—';
  timeText.textContent = `Last update: ${fmtTime(p.time)}`;
}

function renderSamplesTable() {
  const rows = state.samples.slice(-20).reverse();
  if (rows.length === 0) {
    samplesBody.innerHTML = '<tr><td colspan="6" class="hint">No data yet.</td></tr>';
    return;
  }

  samplesBody.innerHTML = rows.map(p => {
    const label = p.sensor_ok ? (LEVEL_LABELS[p.level] ?? '') : 'NoSensor';
    return `<tr>
      <td>${fmtTime(p.time)}</td>
      <td>${p.sensor_ok ? p.shake_score : '—'}</td>
      <td>${p.sensor_ok ? p.rms_g.toFixed(4) : '—'}</td>
      <td>${p.sensor_ok ? p.peak_g.toFixed(4) : '—'}</td>
      <td>${p.temp_ok ? p.temp_c.toFixed(2) : '—'}</td>
      <td>${p.sensor_ok ? `${p.level} · ${label}` : label}</td>
    </tr>`;
  }).join('');
}

function renderEventsTable() {
  const evs = state.events;
  eventCount.textContent = `${evs.length} event${evs.length === 1 ? '' : 's'}`;

  if (evs.length === 0) {
    eventsBody.innerHTML = '<tr><td colspan="7" class="hint">No shake events yet.</td></tr>';
    return;
  }

  eventsBody.innerHTML = evs.slice(0, 50).map(ev => {
    const dur = ev.end.getTime() - ev.start.getTime();
    const avgRms = ev.sumRms / Math.max(1, ev.count);
    const label = LEVEL_LABELS[ev.maxLevel] ?? String(ev.maxLevel);

    return `<tr>
      <td>${fmtTime(ev.start)}</td>
      <td>${fmtTime(ev.end)}</td>
      <td>${fmtDuration(dur)}</td>
      <td>${ev.maxLevel} · ${label}</td>
      <td>${ev.maxScore}</td>
      <td>${ev.maxPeak.toFixed(3)}</td>
      <td>${avgRms.toFixed(3)}</td>
    </tr>`;
  }).join('');
}

function pushEventLogic(p) {
  const SHAKE_LEVEL_MIN = 2; // Caution+
  const END_HYSTERESIS = 2;  // require 2 consecutive "not shaking" windows to close

  const isShaking = p.level >= SHAKE_LEVEL_MIN;

  if (isShaking) {
    state.belowCount = 0;

    if (!state.currentEvent) {
      state.currentEvent = {
        start: p.time,
        end: p.time,
        lastHigh: p.time,
        maxLevel: p.level,
        maxScore: p.shake_score,
        maxPeak: p.peak_g,
        sumRms: p.rms_g,
        count: 1,
      };
    } else {
      state.currentEvent.lastHigh = p.time;
      state.currentEvent.end = p.time;
      state.currentEvent.maxLevel = Math.max(state.currentEvent.maxLevel, p.level);
      state.currentEvent.maxScore = Math.max(state.currentEvent.maxScore, p.shake_score);
      state.currentEvent.maxPeak = Math.max(state.currentEvent.maxPeak, p.peak_g);
      state.currentEvent.sumRms += p.rms_g;
      state.currentEvent.count += 1;
    }

    return;
  }

  // Not shaking
  if (state.currentEvent) {
    state.belowCount += 1;
    if (state.belowCount >= END_HYSTERESIS) {
      // Close event at last time it was above threshold
      state.currentEvent.end = state.currentEvent.lastHigh;
      state.events.unshift(state.currentEvent);
      state.currentEvent = null;
      state.belowCount = 0;
      renderEventsTable();
    }
  }
}

function downloadText(filename, text) {
  const blob = new Blob([text], { type: 'text/csv;charset=utf-8' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  a.remove();
  URL.revokeObjectURL(url);
}

function downloadSamplesCsv() {
  const header = 'time,shake_score,rms_g,peak_g,temp_c,level,label\n';
  const lines = state.samples.map(p => {
    const label = p.sensor_ok ? (LEVEL_LABELS[p.level] ?? '') : 'NoSensor';
    // ISO is good for spreadsheets
    const t = p.time ? p.time.toISOString() : '';
    const score = p.sensor_ok ? p.shake_score : '';
    return `${t},${score},${p.rms_g.toFixed(6)},${p.peak_g.toFixed(6)},${p.temp_c.toFixed(2)},${p.level},${label}`;
  });
  downloadText('myosa_vibration_samples.csv', header + lines.join('\n') + '\n');
}

function downloadEventsCsv() {
  const header = 'start,end,duration_ms,max_level,max_score,max_peak_g,avg_rms_g\n';
  const lines = state.events.map(ev => {
    const dur = ev.end.getTime() - ev.start.getTime();
    const avgRms = ev.sumRms / Math.max(1, ev.count);
    return `${ev.start.toISOString()},${ev.end.toISOString()},${dur},${ev.maxLevel},${ev.maxScore},${ev.maxPeak.toFixed(6)},${avgRms.toFixed(6)}`;
  });
  downloadText('myosa_vibration_events.csv', header + lines.join('\n') + '\n');
}

async function syncTimeToDevice() {
  if (!timeChar) return;

  const nowMs = Date.now();

  // Try 8-byte epoch ms first.
  if (typeof DataView.prototype.setBigUint64 === 'function' && typeof BigInt === 'function') {
    const buf = new ArrayBuffer(8);
    const dv = new DataView(buf);
    dv.setBigUint64(0, BigInt(nowMs), true);
    await timeChar.writeValue(buf);
    return;
  }

  // Fallback: 4-byte epoch seconds
  const buf = new ArrayBuffer(4);
  const dv = new DataView(buf);
  dv.setUint32(0, Math.floor(nowMs / 1000), true);
  await timeChar.writeValue(buf);
}

function handleDisconnect() {
  setStatus(false, 'Disconnected');
  btnConnect.disabled = false;
  btnDisconnect.disabled = true;
  btnDownloadSamples.disabled = state.samples.length === 0;
  btnDownloadEvents.disabled = state.events.length === 0;
}

async function connect() {
  if (!detectBrowserSupport()) return;

  setStatus(false, 'Requesting device…');

  // Note: Web Bluetooth requires user gesture.
  try {
    // Prefer service filter (cleaner list). Some stacks don't advertise the custom
    // service UUID reliably, so we fall back to showing all BLE devices.
    bleDevice = await navigator.bluetooth.requestDevice({
      filters: [{ services: [SERVICE_UUID] }],
      optionalServices: [SERVICE_UUID],
    });
  } catch (e) {
    if (e && e.name === 'NotFoundError') {
      setStatus(false, 'No compatible device found (showing all BLE devices)…');
      bleDevice = await navigator.bluetooth.requestDevice({
        acceptAllDevices: true,
        optionalServices: [SERVICE_UUID],
      });
    } else {
      throw e;
    }
  }

  bleDevice.addEventListener('gattserverdisconnected', handleDisconnect);

  setStatus(false, 'Connecting…');
  gattServer = await bleDevice.gatt.connect();

  const service = await gattServer.getPrimaryService(SERVICE_UUID);
  dataChar = await service.getCharacteristic(DATA_CHAR_UUID);
  timeChar = await service.getCharacteristic(TIME_CHAR_UUID);

  // Reset state
  state.samples = [];
  state.events = [];
  state.currentEvent = null;
  state.belowCount = 0;
  state.sampleIndex = 0;
  state.connectedAtMs = Date.now();

  renderSamplesTable();
  renderEventsTable();
  drawChart();

  // Sync time
  setStatus(true, 'Connected (syncing time…)');
  await syncTimeToDevice();

  // Subscribe
  dataChar.addEventListener('characteristicvaluechanged', (ev) => {
    const dv = ev.target.value;
    const p = parsePacket(dv);
    if (!p) return;

    state.sampleIndex += 1;
    state.samples.push(p);

    updateDashboard(p);
    pushEventLogic(p);

    // UI refresh
    renderSamplesTable();
    drawChart();

    btnDownloadSamples.disabled = state.samples.length === 0;
    btnDownloadEvents.disabled = state.events.length === 0;
  });

  await dataChar.startNotifications();

  setStatus(true, 'Connected (receiving data)');
  btnConnect.disabled = true;
  btnDisconnect.disabled = false;
  btnDownloadSamples.disabled = state.samples.length === 0;
  btnDownloadEvents.disabled = state.events.length === 0;
}

async function disconnect() {
  if (bleDevice && bleDevice.gatt && bleDevice.gatt.connected) {
    bleDevice.gatt.disconnect();
  }
  handleDisconnect();
}

btnConnect.addEventListener('click', () => {
  connect().catch(err => {
    console.error(err);
    setStatus(false, `Error: ${err.message || err}`);
  });
});

btnDisconnect.addEventListener('click', () => {
  disconnect().catch(console.error);
});

btnDownloadSamples.addEventListener('click', () => {
  downloadSamplesCsv();
});

btnDownloadEvents.addEventListener('click', () => {
  downloadEventsCsv();
});

window.addEventListener('resize', () => {
  drawChart();
});

detectBrowserSupport();
setStatus(false, 'Not connected');
