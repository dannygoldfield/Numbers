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
  // Validate JSON syntax first; discard this rounded parse before preserving tokens.
  JSON.parse(source);
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
const recordOnly = location.pathname === '/record';
document.body.classList.toggle('record-only', recordOnly);
let state = null,
  receivedAt = 0,
  inFlight = false,
  connected = false,
  refreshing = false,
  access = null,
  pendingCommand = null,
  freshInspection = false,
  inspectionGeneration = 0;
let history = [],
  historyThrough = '0';
// UI03: retain only the uncertain target, never a command to replay.
function pendingKey() {
  return (
    'numbers-pending:' +
    JSON.stringify([access.session_label, access.spec_revision, access.role, access.bidder_id])
  );
}
function clearPresentation() {
  state = null;
  history = [];
  historyThrough = '0';
  connected = false;
  freshInspection = false;
  for (const id of [
    'number',
    'phase',
    'clock',
    'clock-label',
    'clock-note',
    'bid-rule',
    'leader-label',
    'available',
    'reserved',
    'leader',
    'record-state',
    'protocol-balance',
    'record-count',
    'reconstructed',
    'state-json',
    'ownership-banner',
  ])
    put(id, '—');
  element('records').replaceChildren();
  element('account-summary').replaceChildren();
  element('ownership-banner').hidden = true;
  element('ownership-banner').classList.remove('final');
  put('last-command', 'No command submitted in this view.');
  put('result', 'Two simulated bidders. One recorded history.');
  element('bid-button').disabled = element('settle').disabled = element('expire').disabled = true;
}
function acceptAccess(next) {
  if (
    !next ||
    next.spec_revision !== '0.3' ||
    typeof next.session_label !== 'string' ||
    !/^[A-Za-z0-9_-]{1,64}$/.test(next.session_label) ||
    !(
      (next.role === 'bidder' && ['demo-1', 'demo-2'].includes(next.bidder_id)) ||
      (next.role === 'facilitator' && next.bidder_id === null)
    )
  )
    throw new Error('Invalid access response');
  if (JSON.stringify(next) !== JSON.stringify(access)) {
    clearPresentation();
    access = next;
    pendingCommand = JSON.parse(sessionStorage.getItem(pendingKey()) || 'null');
    put(
      'identity',
      access.role === 'facilitator'
        ? 'Facilitator'
        : `${bidderName(access.bidder_id)} · ${access.bidder_id}`,
    );
    put(
      'session-label',
      `Session ${access.session_label} · ${access.role === 'facilitator' ? 'Facilitator' : bidderName(access.bidder_id)} · simulated funds`,
    );
    element('bid-panel').hidden = access.role !== 'bidder';
    element('settlement-controls').hidden = true;
  }
}
async function request(url, options = {}) {
  const controller = new AbortController();
  let timer;
  try {
    return await Promise.race([
      (async () => {
        const response = await fetch(url, {
          ...options,
          cache: 'no-store',
          credentials: 'same-origin',
          signal: controller.signal,
        });
        return parseExact(await response.text());
      })(),
      new Promise((_, reject) => {
        timer = setTimeout(() => {
          controller.abort();
          reject(new Error('The 30-second wait ended; this does not cancel a command.'));
        }, 30000);
      }),
    ]);
  } finally {
    clearTimeout(timer);
  }
}
async function read(url) {
  const value = await request(url);
  if (value?.status !== 'success' || !value.data || typeof value.data !== 'object')
    throw new Error(value?.error?.message || 'Unconfirmed backend response');
  return value.data;
}
function renderUncertainty() {
  element('uncertainty').hidden = !pendingCommand;
  element('acknowledge').disabled = inFlight || !connected || !freshInspection;
  if (pendingCommand)
    put(
      'uncertain-target',
      `Outcome unconfirmed for ${pendingCommand.target}. The command may have committed or may still complete. Inspect the shared records; absence in a read is not proof of rejection. Ask the facilitator if the outcome remains unclear.`,
    );
}
function acknowledgeNewCommand() {
  if (inFlight || !connected || !freshInspection || !pendingCommand || recordOnly) return;
  sessionStorage.removeItem(pendingKey());
  pendingCommand = null;
  renderState();
}

function validateState(next) {
  const integer = (v) => v instanceof ExactNumber && /^(0|[1-9][0-9]*)$/.test(v.value);
  if (
    !next ||
    !integer(next.current_number) ||
    !integer(next.last_sequence_index) ||
    typeof next.server_time !== 'string' ||
    !Number.isFinite(Date.parse(next.server_time)) ||
    !['Scheduled', 'Open', 'Closed', 'AwaitingSettlement', 'Finalized'].includes(
      next.auction_state,
    ) ||
    !Array.isArray(next.balances) ||
    next.balances.length !== 2 ||
    !next.balances.every(
      (b, i) =>
        b.bidder_id === ['demo-1', 'demo-2'][i] &&
        integer(b.available_rana) &&
        integer(b.reserved_rana),
    ) ||
    !integer(next.protocol_held_rana) ||
    !integer(next.reconstruction?.validated_through_sequence_index) ||
    !next.parameters ||
    !integer(next.parameters.minimum_bid_rana) ||
    !integer(next.parameters.minimum_increment_rana)
  )
    throw new Error('Invalid state projection');
  return next;
}
function updateState(next) {
  validateState(next);
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
  put('connection', 'Connected · backend records determine the result');
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
      Finalized: 'Ownership finalized',
    }[phase],
  );
  put('number', state.current_number);
  const chosen = state.balances.find((balance) => balance.bidder_id === access?.bidder_id);
  put('available', chosen?.available_rana ?? '—');
  put('reserved', chosen?.reserved_rana ?? '—');
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
  element('bid-button').disabled =
    recordOnly ||
    access?.role !== 'bidder' ||
    inFlight ||
    !connected ||
    Boolean(pendingCommand) ||
    !['Scheduled', 'Open'].includes(phase);
  element('settlement-controls').hidden =
    access?.role !== 'facilitator' || phase !== 'AwaitingSettlement';
  element('settle').disabled = element('expire').disabled =
    recordOnly ||
    access?.role !== 'facilitator' ||
    inFlight ||
    !connected ||
    Boolean(pendingCommand);
  renderUncertainty();
  // RT06/UI02: ownership is pending until its canonical finalization exists.
  const banner = state.ownership
    ? state.ownership.ownership_kind === 'winner'
      ? `Owned by ${bidderName(state.ownership.owner_id)}`
      : 'Unowned'
    : state.resolution
      ? 'Winner resolved · ownership awaiting settlement'
      : '';
  put('ownership-banner', banner);
  element('ownership-banner').hidden = !banner;
  element('ownership-banner').classList.toggle('final', Boolean(state.ownership));
  put('record-state', phase);
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
    [
      'Ownership',
      state.ownership
        ? state.ownership.ownership_kind === 'winner'
          ? `Owned by ${bidderName(state.ownership.owner_id)}`
          : 'Unowned'
        : 'Pending',
    ],
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
      return payload.ownership_kind === 'winner'
        ? `Owned by ${bidderName(payload.owner_id)}`
        : 'Unowned · no owner';
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
  const generation = inspectionGeneration;
  try {
    acceptAccess(await read('/access'));
    updateState(await read('/state'));
    freshInspection = generation === inspectionGeneration && !inFlight;
    await refreshHistory();
  } catch (error) {
    connected = false;
    put('connection', `Connection unavailable: ${error.message}`);
    element('connection').classList.add('error');
    renderState();
  } finally {
    refreshing = false;
    renderUncertainty();
  }
}
function showResult(result) {
  put('last-command', prettyExact(result));
}
// UI01/UI03: explain the backend's SE03 decision using its response snapshot.
// This display calculation never validates, increases, or resubmits a bid.
function bidRejectionReason(reason, snapshot) {
  switch (reason) {
    case 'wrong_auction_number':
      return `This bid was for a different number. The current number is ${snapshot.current_number}.`;
    case 'missing_required_field':
      return 'The bid is missing required details.';
    case 'malformed_field':
      return 'The bid details have an invalid format. Enter a whole-number amount.';
    case 'unknown_identity':
      return 'The submitted identity is not a supported simulated bidder.';
    case 'amount_below_minimum':
      return `Bid at least ${snapshot.parameters.minimum_bid_rana} rana.`;
    case 'amount_below_required_increment': {
      const required =
        BigInt(String(snapshot.leading_bid.amount_rana)) +
        BigInt(String(snapshot.parameters.minimum_increment_rana));
      return `Bid at least ${required} rana.`;
    }
    case 'amount_above_maximum':
      return `Bid no more than ${snapshot.parameters.maximum_bid_rana} rana.`;
    case 'insufficient_available_rana':
      return 'You do not have enough rana for this bid.';
    default:
      return 'The backend rejected this bid. Inspect the command result for details.';
  }
}
async function submit(endpoint, body) {
  if (
    recordOnly ||
    inFlight ||
    !connected ||
    pendingCommand ||
    !access ||
    (endpoint === '/bid'
      ? access.role !== 'bidder'
      : endpoint !== '/demo/settlement' || access.role !== 'facilitator')
  )
    return;
  const submittedAccess = pendingKey();
  inspectionGeneration += 1;
  inFlight = true;
  renderState();
  try {
    pendingCommand = {
      target: `Number ${state.current_number} · ${endpoint === '/bid' ? 'bid' : 'settlement'}`,
    };
    freshInspection = false;
    sessionStorage.setItem(submittedAccess, JSON.stringify(pendingCommand));
    renderUncertainty();
    const result = await request(endpoint, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body,
    });
    if (pendingKey() !== submittedAccess)
      throw new Error(
        'The session changed while the command response was pending. Inspect the previous session separately.',
      );
    if (
      !(
        result?.status === 'error' &&
        [
          'authentication_required',
          'forbidden',
          'unsupported_media_type',
          'request_too_large',
          'malformed_request',
          'invalid_read_parameters',
          'unknown_endpoint',
          'method_not_allowed',
          'auction_not_accepting_bids',
          'settlement_not_available',
          'missing_required_field',
          'malformed_field',
          'wrong_auction',
          'invalid_settlement_outcome',
          'storage_unavailable',
          'history_invalid',
          'clock_invalid',
          'arithmetic_unrepresentable',
        ].includes(result.error?.code) &&
        typeof result.error.message === 'string' &&
        Array.isArray(result.evaluation_records)
      ) &&
      !(
        result?.status === 'success' &&
        typeof result.data?.accepted === 'boolean' &&
        result.data.state &&
        Array.isArray(result.data.command_records) &&
        Array.isArray(result.data.evaluation_records)
      )
    )
      throw new Error('Unconfirmed command response');
    if (result.status === 'success') {
      validateState(result.data.state);
      if (
        endpoint === '/bid' &&
        (!result.data.bid_record ||
          result.data.bid_record.number === undefined ||
          (!result.data.accepted && !result.data.bid_record.payload_json))
      )
        throw new Error('Invalid bid response');
      if (
        endpoint === '/demo/settlement' &&
        !['winner', 'unowned'].includes(result.data.state.ownership?.ownership_kind)
      )
        throw new Error('Invalid settlement response');
    }
    if (!(
      result.status === 'error' &&
      [
        'storage_unavailable',
        'history_invalid',
        'clock_invalid',
        'arithmetic_unrepresentable',
      ].includes(result.error.code)
    )) {
      sessionStorage.removeItem(submittedAccess);
      pendingCommand = null;
    }
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
            : `Number ${result.data.bid_record.number}: bid rejected. ${bidRejectionReason(result.data.bid_record.payload_json.rejection_reason, result.data.state)} Your rejected bid was recorded; your balance is unchanged.`,
        );
      else
        put(
          'result',
          result.data.state.ownership.ownership_kind === 'winner'
            ? `Number ${result.data.state.current_number}: settlement committed. Winning rana captured; the winner is now the owner.`
            : `Number ${result.data.state.current_number}: settlement committed. Winning reservation released; the number is Unowned.`,
        );
      element('result').classList.toggle('error', !accepted);
    }
  } catch (error) {
    put(
      'result',
      pendingKey() === submittedAccess
        ? 'The response was lost or unavailable. The command may have committed. Inspect the refreshed state and journal. A missing record does not prove the request cannot still complete. Acknowledge before making a new command.'
        : 'The session changed while a command was pending. Inspect that previous session separately; no command has been carried into this session.',
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
element('acknowledge').addEventListener('click', acknowledgeNewCommand);
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
  const body = `{"auction_number":${state.current_number},"bidder_id":${JSON.stringify(access.bidder_id)},"amount_rana":${amount}}`;
  submit('/bid', body);
});
element('settle').addEventListener('click', () =>
  submit('/demo/settlement', JSON.stringify({ auction_id: state.auction_id, outcome: 'settled' })),
);
element('expire').addEventListener('click', () =>
  submit('/demo/settlement', JSON.stringify({ auction_id: state.auction_id, outcome: 'expired' })),
);
setInterval(refresh, 1000);
setInterval(renderClock, 100);
refresh();
