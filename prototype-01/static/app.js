'use strict';
// UI01/UI03: this browser displays backend facts and sends each command once.
// Integer tokens stay strings on arrival, preserving amounts beyond JS Number precision.
class ExactNumber {
  constructor(value) {
    this.value = value;
  }
  toString() {
    return this.value;
  }
}
function prettyExact(value, depth = 0) {
  if (value instanceof ExactNumber) return value.value;
  if (value === null || typeof value !== 'object') return JSON.stringify(value);
  const indent = '  '.repeat(depth + 1),
    close = '  '.repeat(depth);
  if (Array.isArray(value))
    return value.length
      ? '[\n' +
          value.map((item) => indent + prettyExact(item, depth + 1)).join(',\n') +
          '\n' +
          close +
          ']'
      : '[]';
  const entries = Object.entries(value);
  return entries.length
    ? '{\n' +
        entries
          .map(([key, item]) => indent + JSON.stringify(key) + ': ' + prettyExact(item, depth + 1))
          .join(',\n') +
        '\n' +
        close +
        '}'
    : '{}';
}
function parseExact(source) {
  let output = '',
    index = 0;
  while (index < source.length) {
    const char = source[index];
    if (char === '"') {
      const start = index++;
      while (index < source.length) {
        if (source[index] === '\\') {
          index += 2;
          continue;
        }
        if (source[index++] === '"') break;
      }
      output += source.slice(start, index);
    } else if (char === '-' || /[0-9]/.test(char)) {
      const start = index++;
      while (index < source.length && /[0-9eE+.\-]/.test(source[index])) index++;
      output += '{"$exactNumber":' + JSON.stringify(source.slice(start, index)) + '}';
    } else {
      output += char;
      index++;
    }
  }
  return JSON.parse(output, (key, value) =>
    value && typeof value === 'object' && Object.keys(value).length === 1 && '$exactNumber' in value
      ? new ExactNumber(value.$exactNumber)
      : value,
  );
}
const element = (id) => document.getElementById(id);
const put = (id, text) => {
  element(id).textContent = text;
};
const bidderName = (id) => (id === 'demo-1' ? 'Bidder A' : id === 'demo-2' ? 'Bidder B' : '—');
const protocolOnly = location.pathname === '/protocol';
document.body.classList.toggle('protocol-only', protocolOnly);
let state = null,
  receivedAt = 0,
  inFlight = false,
  connected = false,
  refreshing = false;
let history = [],
  historyThrough = '0';
const channel =
  typeof BroadcastChannel === 'function' ? new BroadcastChannel('numbers-prototype-01-view') : null;

async function read(url) {
  const response = await fetch(url, { cache: 'no-store' });
  const value = parseExact(await response.text());
  if (value.status !== 'success') throw new Error(value.error.message);
  return value.data;
}
function updateState(next) {
  if (
    state &&
    (BigInt(String(next.last_sequence_index)) < BigInt(String(state.last_sequence_index)) ||
      (String(next.last_sequence_index) === String(state.last_sequence_index) &&
        next.server_time < state.server_time))
  )
    return;
  state = next;
  receivedAt = performance.now();
  connected = true;
  put('connection', 'Connected locally · backend records determine the result');
  element('connection').classList.remove('error');
  renderState();
}
function renderState() {
  if (!state) return;
  const phase = state.auction_state;
  put(
    'phase',
    {
      Scheduled: 'Awaiting the first bid',
      Open: 'Auction open',
      Closed: 'Auction closed',
      AwaitingSettlement: 'Awaiting settlement',
      Finalized: 'Title finalized',
    }[phase],
  );
  put('number', state.current_number);
  const chosen = state.balances.find((balance) => balance.bidder_id === element('identity').value);
  put('available', chosen.available_rana);
  put('reserved', chosen.reserved_rana);
  put('leader-label', state.resolution ? 'Resolved winner' : 'Leading bid');
  put(
    'leader',
    state.leading_bid
      ? `${bidderName(state.leading_bid.bidder_id)} · ${state.leading_bid.amount_rana} rana`
      : 'No bids yet',
  );
  put(
    'bid-rule',
    `Minimum ${state.parameters.minimum_bid_rana} rana · increase by at least ${state.parameters.minimum_increment_rana} rana`,
  );
  element('bid-button').disabled = inFlight || !connected || !['Scheduled', 'Open'].includes(phase);
  element('settlement-controls').hidden = phase !== 'AwaitingSettlement';
  element('settle').disabled = element('expire').disabled = inFlight || !connected;
  const banner = state.title
    ? state.title.title_kind === 'winner'
      ? `Title assigned to ${bidderName(state.title.holder_id)}`
      : 'Title assigned to PublicLand'
    : state.resolution
      ? 'Winner resolved · title awaiting settlement'
      : '';
  put('title-banner', banner);
  element('title-banner').hidden = !banner;
  element('title-banner').classList.toggle('final', Boolean(state.title));
  put('protocol-state', phase);
  put('protocol-balance', state.protocol_held_rana);
  put('record-count', state.last_sequence_index);
  put('reconstructed', state.reconstruction.validated_through_sequence_index);
  // This is a readable projection. Original numeric JSON is shown in journal payloads.
  put('state-json', prettyExact(state));
  const summary = element('account-summary');
  summary.replaceChildren();
  for (const balance of state.balances) {
    const row = document.createElement('div');
    row.className = 'account';
    const name = document.createElement('span');
    name.textContent = `${bidderName(balance.bidder_id)} · simulated`;
    const amounts = document.createElement('span');
    amounts.textContent = `${balance.available_rana} available / ${balance.reserved_rana} reserved`;
    row.append(name, amounts);
    summary.append(row);
  }
  for (const [label, value] of [
    ['Settlement', state.settlement ? state.settlement.status : 'Pending'],
    ['Title', state.title ? state.title.title_kind : 'Unassigned'],
  ]) {
    const row = document.createElement('div');
    row.className = 'account';
    row.textContent = `${label}: ${value}`;
    summary.append(row);
  }
  renderClock();
}
function renderClock() {
  if (!state) return;
  const phase = state.auction_state;
  if (phase === 'Scheduled') {
    put('clock', '3:45');
    put('clock-label', 'Auction duration');
    put('clock-note', 'The first valid bid starts the clock.');
    return;
  }
  if (phase === 'AwaitingSettlement' || phase === 'Closed') {
    put('clock', '0:00');
    put('clock-label', 'Auction complete');
    put('clock-note', 'The winning reservation is held until settlement.');
    return;
  }
  const rhythm = phase === 'Finalized';
  const end = Date.parse(rhythm ? state.sequence.next_available_at : state.current_end_time);
  const observedNow = Date.parse(state.server_time) + performance.now() - receivedAt;
  const seconds = Math.max(0, Math.ceil((end - observedNow) / 1000));
  put('clock', `${Math.floor(seconds / 60)}:${String(seconds % 60).padStart(2, '0')}`);
  put('clock-label', rhythm ? 'Rhythm gap' : 'Time remaining');
  put(
    'clock-note',
    rhythm
      ? `Number ${state.sequence.next_number} appears after the backend records its arrival.`
      : `${state.extension_count} of ${state.parameters.max_extensions} extensions used. Closing is determined by the backend.`,
  );
}
function decodeCommand(hex) {
  const bytes = new Uint8Array(hex.match(/../g).map((part) => parseInt(part, 16)));
  return new TextDecoder('utf-8', { fatal: true }).decode(bytes);
}
function recordEffect(record) {
  const payload = record.payload_json;
  switch (record.record_type) {
    case 'RanaIssueRecord':
      return `${bidderName(payload.bidder_id)} receives ${payload.amount_rana} rana`;
    case 'RanaReserveRecord':
      return `${payload.amount_rana} rana reserved for ${bidderName(payload.bidder_id)}`;
    case 'RanaReleaseRecord':
      return `${payload.amount_rana} rana released to ${bidderName(payload.bidder_id)}`;
    case 'RanaCaptureRecord':
      return `${payload.amount_rana} rana captured into the protocol balance`;
    case 'BidRecord':
      return payload.validity === 'valid'
        ? `Accepted · ${payload.amount_rana} rana`
        : `Rejected · ${payload.rejection_reason}`;
    case 'FinalizationRecord':
      return payload.title_kind === 'winner'
        ? `Winner title · ${bidderName(payload.holder_id)}`
        : 'PublicLand title';
    default:
      return '';
  }
}
function renderHistory() {
  const container = element('records');
  const openIds = new Set([...container.querySelectorAll('details[open]')].map((node) => node.id));
  container.replaceChildren();
  let currentGroup = null,
    groupContainer = null;
  for (const record of history) {
    if (record.group_id !== currentGroup) {
      currentGroup = record.group_id;
      groupContainer = document.createElement('div');
      groupContainer.className = 'record-group';
      const label = document.createElement('div');
      label.className = 'group-label';
      label.textContent = `${record.group_type} · ${record.server_time}`;
      groupContainer.append(label);
      container.append(groupContainer);
    }
    const details = document.createElement('details');
    details.className = 'record';
    details.id = record.record_id;
    details.open = openIds.has(record.record_id);
    const summary = document.createElement('summary');
    const seq = document.createElement('span');
    seq.className = 'seq';
    seq.textContent = String(record.sequence_index).padStart(2, '0');
    const type = document.createElement('span');
    type.className = 'record-type';
    type.textContent = record.record_type;
    summary.append(seq, type);
    details.append(summary);
    const effect = document.createElement('p');
    effect.className = 'effect';
    effect.textContent = recordEffect(record);
    details.append(effect);
    const pre = document.createElement('pre');
    pre.textContent = prettyExact(record);
    details.append(pre);
    if (record.payload_json.command_utf8_hex) {
      const heading = document.createElement('p');
      heading.className = 'hint';
      heading.textContent = 'Exact submitted command · decoded UTF-8';
      const command = document.createElement('pre');
      command.textContent = decodeCommand(record.payload_json.command_utf8_hex);
      details.append(heading, command);
    }
    groupContainer.append(details);
  }
}
async function refreshHistory() {
  if (!state || historyThrough === String(state.last_sequence_index)) return;
  // This cache is presentation only. Each page consists of canonical backend records.
  let offset = history.length,
    next;
  do {
    const data = await read(`/auction/history?limit=100&offset=${offset}`);
    history.push(...data.records);
    next = data.pagination.next_offset;
    if (next !== null) offset = Number(next);
  } while (next !== null);
  historyThrough = history.length ? String(history[history.length - 1].sequence_index) : '0';
  renderHistory();
}
async function refresh() {
  if (refreshing || inFlight) return;
  refreshing = true;
  try {
    updateState(await read('/state'));
    await refreshHistory();
  } catch (error) {
    connected = false;
    put('connection', `Local backend unavailable: ${error.message}`);
    element('connection').classList.add('error');
    renderState();
  } finally {
    refreshing = false;
  }
}
function showResult(result, broadcast = true) {
  put('last-command', prettyExact(result));
  if (broadcast && channel) channel.postMessage(prettyExact(result));
}
async function submit(endpoint, body) {
  if (inFlight || !connected) return;
  inFlight = true;
  renderState();
  try {
    const response = await fetch(endpoint, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body,
    });
    const result = parseExact(await response.text());
    showResult(result);
    if (result.status === 'error') {
      put(
        'result',
        `${result.error.message}${['storage_unavailable', 'history_invalid', 'clock_invalid', 'arithmetic_unrepresentable'].includes(result.error.code) ? '' : ' No command record was written; inspect the journal for preceding evaluation.'}`,
      );
      element('result').classList.add('error');
    } else {
      updateState(result.data.state);
      const accepted = result.data.accepted;
      if (result.data.bid_record)
        put(
          'result',
          accepted
            ? `Number ${result.data.bid_record.number}: bid accepted. Its reservation is committed to the journal.`
            : `Number ${result.data.bid_record.number}: bid rejected: ${result.data.bid_record.payload_json.rejection_reason}. An invalid BidRecord was recorded; no rana moved.`,
        );
      else
        put(
          'result',
          result.data.state.title.title_kind === 'winner'
            ? `Number ${result.data.state.current_number}: settlement committed. Winning rana captured; title assigned to the winner.`
            : `Number ${result.data.state.current_number}: settlement committed. Winning reservation released; title assigned to PublicLand.`,
        );
      element('result').classList.toggle('error', !accepted);
    }
  } catch (error) {
    put(
      'result',
      'The response was lost or unavailable. The command may have committed. Inspect the refreshed state and journal before making a new submission.',
    );
    element('result').classList.add('error');
    showResult({
      observation: 'Response unavailable; no automatic resubmission',
      message: error.message,
    });
  } finally {
    inFlight = false;
    await refresh();
    renderState();
  }
}
element('identity').addEventListener('change', renderState);
element('bid-form').addEventListener('submit', (event) => {
  event.preventDefault();
  if (!state) return;
  const entered = element('amount').value.trim();
  // Forward numeric spelling intact, including fractional/exponent syntax for backend rejection.
  let amount;
  try {
    JSON.parse(entered);
    amount = entered;
  } catch {
    amount = JSON.stringify(entered);
  }
  const body = `{"auction_number":${state.current_number},"bidder_id":${JSON.stringify(element('identity').value)},"amount_rana":${amount}}`;
  submit('/bid', body);
});
element('settle').addEventListener('click', () =>
  submit('/demo/settlement', JSON.stringify({ auction_id: state.auction_id, outcome: 'settled' })),
);
element('expire').addEventListener('click', () =>
  submit('/demo/settlement', JSON.stringify({ auction_id: state.auction_id, outcome: 'expired' })),
);
if (channel) channel.onmessage = (event) => showResult(parseExact(event.data), false);
setInterval(refresh, 1000);
setInterval(renderClock, 100);
refresh();
