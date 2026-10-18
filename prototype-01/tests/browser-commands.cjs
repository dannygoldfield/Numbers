const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');
const path = require('node:path');

const source = fs.readFileSync(path.join(__dirname, '../static/app.js'), 'utf8');
function snapshot() {
  return {
    server_time: '2026-10-10T12:00:00.000Z',
    current_number: 1,
    auction_id: 'auc_000000000001',
    auction_state: 'Scheduled',
    parameters: {
      minimum_bid_rana: 5,
      minimum_increment_rana: 1,
      maximum_bid_rana: null,
      max_extensions: 2,
    },
    extension_count: 0,
    current_end_time: null,
    leading_bid: null,
    resolution: null,
    settlement: null,
    ownership: null,
    balances: [
      { bidder_id: 'demo-1', available_rana: 600, reserved_rana: 0 },
      { bidder_id: 'demo-2', available_rana: 600, reserved_rana: 0 },
    ],
    protocol_held_rana: 0,
    last_sequence_index: 3,
    reconstruction: { validated_through_sequence_index: 3 },
  };
}

// Exercise the shipped submission/rendering code with controlled transport and a small DOM.
// No server, journal, clock advancement, or extra browser dependency is involved.
async function browser(
  reply,
  pathname = '/',
  initial = snapshot(),
  role = 'bidder',
  storage = new Map(),
  readReply = null,
) {
  const elements = new Map(),
    requests = [];
  let current = initial,
    access = {
      role,
      bidder_id: role === 'bidder' ? 'demo-1' : null,
      session_label: 'test',
      spec_revision: '0.3',
    };
  const timers = new Map();
  let nextTimer = 0;
  const node = () => ({
    value: 'demo-1',
    textContent: '',
    disabled: false,
    classList: { toggle() {}, add() {}, remove() {} },
    replaceChildren() {},
    append() {},
    addEventListener() {},
    querySelectorAll() {
      return [];
    },
  });
  const element = (id) => {
    if (!elements.has(id)) elements.set(id, node());
    return elements.get(id);
  };
  const context = vm.createContext({
    document: { body: node(), getElementById: element, createElement: node },
    location: { pathname },
    performance: { now: () => 0 },
    setInterval() {},
    AbortController,
    setTimeout(fn) {
      const id = ++nextTimer;
      timers.set(id, fn);
      return id;
    },
    clearTimeout(id) {
      timers.delete(id);
    },
    sessionStorage: {
      getItem: (key) => storage.get(key) ?? null,
      setItem: (key, value) => storage.set(key, value),
      removeItem: (key) => storage.delete(key),
    },
    fetch: async (url, options = {}) => {
      requests.push({ url, ...options });
      if (options.method === 'POST') {
        const submittedAccess = JSON.stringify(access);
        const raw = await reply();
        let parsed;
        try {
          parsed = JSON.parse(raw);
        } catch {
          // Malformed bytes still reach the shipped response parser.
        }
        if (parsed?.data?.state && JSON.stringify(access) === submittedAccess)
          current = parsed.data.state;
        return { text: async () => raw };
      }
      if (readReply) {
        const raw = await readReply(url);
        if (raw !== undefined) return { text: async () => raw };
      }
      const data =
        url === '/access'
          ? access
          : url === '/state'
            ? current
            : { records: [], pagination: { next_offset: null } };
      return { text: async () => JSON.stringify({ status: 'success', data }) };
    },
  });
  vm.runInContext(source, context);
  await new Promise((resolve) => setImmediate(resolve));
  return {
    element,
    requests,
    storage,
    timeout: () => {
      for (const fn of [...timers.values()]) fn();
    },
    acknowledge: () => vm.runInContext('acknowledgeNewCommand()', context),
    refresh: () => vm.runInContext('refresh()', context),
    acceptAccessNow: (next) => vm.runInContext(`acceptAccess(${JSON.stringify(next)})`, context),
    changeAccess: (next) => {
      access = next;
    },
    setState: (next) => {
      current = next;
    },
    settle: (body) =>
      vm.runInContext(`submit('/demo/settlement', ${JSON.stringify(body)})`, context),
    recordOnly: () => vm.runInContext('recordOnly', context),
    submit: (body) => vm.runInContext(`submit('/bid', ${JSON.stringify(body)})`, context),
  };
}
const command = '{"auction_number":1,"bidder_id":"demo-1","amount_rana":10}';

test('rejected bids explain the exact required increase from the response and retain raw evidence', async () => {
  for (const leading of ['90', '10000000000000000000003']) {
    const state = snapshot();
    state.auction_state = 'Open';
    state.current_end_time = '2026-10-10T12:03:45.000Z';
    state.last_sequence_index = 7;
    state.leading_bid = { bidder_id: 'demo-2', amount_rana: 'LEADING_AMOUNT' };
    const response = JSON.stringify({
      status: 'success',
      data: {
        command_records: [],
        evaluation_records: [],
        accepted: false,
        bid_record: {
          number: 1,
          payload_json: { rejection_reason: 'amount_below_required_increment' },
        },
        state,
      },
    }).replace('"LEADING_AMOUNT"', leading);
    const app = await browser(async () => response);
    await app.submit(command);
    const message = app.element('result').textContent;
    assert.ok(message.includes(`Bid at least ${BigInt(leading) + 1n} rana.`));
    assert.ok(message.includes('Your rejected bid was recorded; your balance is unchanged.'));
    assert.ok(!message.includes('amount_below_required_increment'));
    assert.ok(app.element('last-command').textContent.includes('amount_below_required_increment'));
    assert.equal(String(app.element('available').textContent), '600');
    const posts = app.requests.filter((r) => r.method === 'POST');
    assert.equal(posts.length, 1);
    assert.equal(posts[0].body, command);
  }
});

test('a pending submission blocks repeat clicks and changes balances only after the accepted response', async () => {
  let release;
  const pending = new Promise((resolve) => {
    release = resolve;
  });
  const app = await browser(() => pending);
  const first = app.submit(command);
  assert.equal(app.element('bid-button').disabled, true);
  await app.submit(command);
  assert.equal(app.requests.filter((r) => r.method === 'POST').length, 1);
  assert.equal(String(app.element('available').textContent), '600');
  assert.equal(String(app.element('reserved').textContent), '0');
  const state = snapshot();
  state.auction_state = 'Open';
  state.current_end_time = '2026-10-10T12:03:45.000Z';
  state.last_sequence_index = 6;
  state.leading_bid = { bidder_id: 'demo-1', amount_rana: 10 };
  state.balances[0] = { bidder_id: 'demo-1', available_rana: 590, reserved_rana: 10 };
  release(
    JSON.stringify({
      status: 'success',
      data: {
        command_records: [],
        evaluation_records: [],
        accepted: true,
        bid_record: { number: 1 },
        state,
      },
    }),
  );
  await first;
  assert.ok(app.element('result').textContent.includes('bid accepted'));
  assert.equal(String(app.element('available').textContent), '590');
  assert.equal(String(app.element('reserved').textContent), '10');
  assert.equal(app.element('bid-button').disabled, false);
});

test('a lost response prompts inspection and refreshes state without resubmitting', async () => {
  const app = await browser(async () => {
    throw new Error('Controlled lost response');
  });
  await app.submit(command);
  assert.equal(app.requests.filter((r) => r.method === 'POST').length, 1);
  assert.ok(app.element('result').textContent.includes('The command may have committed.'));
  assert.ok(app.element('result').textContent.includes('Inspect the refreshed state and journal'));
  assert.ok(app.element('last-command').textContent.includes('no automatic resubmission'));
  const postIndex = app.requests.findIndex((r) => r.method === 'POST');
  assert.ok(app.requests.slice(postIndex + 1).some((r) => r.url === '/state'));
});

test('revision 0.3 ownership displays pending, winner and unowned without legacy fields', async () => {
  const pending = await browser(async () => '');
  assert.equal(pending.element('ownership-banner').textContent, '');
  for (const [kind, owner, label, outcome] of [
    ['winner', 'demo-1', 'Owned by Bidder A', 'settled'],
    ['unowned', null, 'Unowned', 'expired'],
  ]) {
    const state = snapshot();
    state.auction_state = 'Finalized';
    state.sequence = { next_number: 2, next_available_at: '2026-10-10T12:00:12.000Z' };
    state.ownership = {
      record_id: 'rec_000000000011',
      ownership_kind: kind,
      owner_id: owner,
      finalization_time: state.server_time,
    };
    state.settlement = { status: outcome };
    const app = await browser(
      async () =>
        JSON.stringify({
          status: 'success',
          data: { command_records: [], evaluation_records: [], accepted: true, state },
        }),
      '/',
      snapshot(),
      'facilitator',
    );
    await app.settle(JSON.stringify({ auction_id: state.auction_id, outcome }));
    assert.equal(app.element('ownership-banner').textContent, label);
    assert.ok(app.element('state-json').textContent.includes('ownership_kind'));
    assert.ok(!app.element('state-json').textContent.includes('title_kind'));
    assert.equal(app.requests.filter((r) => r.method === 'POST').length, 1);
  }
});

test('Record address selects the read-only presentation', async () => {
  const app = await browser(async () => '', '/record');
  assert.equal(app.recordOnly(), true);
  assert.equal(app.requests.filter((r) => r.method === 'POST').length, 0);
});

test('fixed identities and facilitator controls cannot be switched in the UI', async () => {
  const bidder = await browser(async () => '');
  assert.ok(bidder.element('identity').textContent.includes('Bidder A'));
  assert.equal(bidder.element('bid-panel').hidden, false);
  await bidder.settle('{}');
  assert.equal(bidder.requests.filter((r) => r.method === 'POST').length, 0);
  const state = snapshot();
  state.auction_state = 'AwaitingSettlement';
  const facilitator = await browser(async () => '', '/', state, 'facilitator');
  assert.equal(facilitator.element('bid-panel').hidden, true);
  assert.equal(facilitator.element('settlement-controls').hidden, false);
  await facilitator.submit(command);
  assert.equal(facilitator.requests.filter((r) => r.method === 'POST').length, 0);
});

test('timeout keeps uncertainty through inspection and reload and never resends the POST', async () => {
  let release;
  const delayed = new Promise((resolve) => {
    release = resolve;
  });
  const app = await browser(() => delayed);
  const submission = app.submit(command);
  await new Promise((resolve) => setImmediate(resolve));
  app.timeout();
  await submission;
  assert.equal(app.requests.filter((r) => r.method === 'POST').length, 1);
  assert.equal(app.element('bid-button').disabled, true);
  assert.ok(app.element('uncertain-target').textContent.includes('absence in a read is not proof'));
  await app.submit(command);
  assert.equal(app.requests.filter((r) => r.method === 'POST').length, 1);
  const restored = await browser(async () => '', '/', snapshot(), 'bidder', app.storage);
  assert.equal(restored.element('uncertainty').hidden, false);
  assert.ok(restored.element('uncertain-target').textContent.includes('Number 1'));
  assert.equal(restored.requests.filter((r) => r.method === 'POST').length, 0);
  restored.acknowledge();
  assert.equal(restored.element('bid-button').disabled, false);
  assert.equal(restored.element('uncertainty').hidden, true);
  release('{}');
});

test('malformed success and gateway pages keep the outcome unconfirmed', async () => {
  for (const reply of [
    '<html>timeout</html>',
    '{"status":"success","data":{}}',
    '{"status":"error","error":{"code":"gateway_problem","message":"unknown"},"evaluation_records":[]}',
  ]) {
    const app = await browser(async () => reply);
    await app.submit(command);
    assert.equal(app.element('uncertainty').hidden, false);
    assert.equal(app.element('bid-button').disabled, true);
    assert.equal(app.requests.filter((r) => r.method === 'POST').length, 1);
  }
});

test('malformed numeric JSON remains unconfirmed without losing exact integer support', async () => {
  const malformed = JSON.stringify({
    status: 'success',
    data: {
      accepted: true,
      command_records: [],
      evaluation_records: [],
      bid_record: { number: 1 },
      state: snapshot(),
    },
  }).replace('"max_extensions":2', '"max_extensions":02');
  const app = await browser(async () => malformed);
  await app.submit(command);
  assert.equal(app.element('uncertainty').hidden, false);
  assert.equal(app.element('bid-button').disabled, true);
  assert.equal(app.requests.filter((r) => r.method === 'POST').length, 1);
});

test('changing session clears all previous timing and phase facts even if the new read fails', async () => {
  let failState = false;
  const initial = snapshot();
  initial.auction_state = 'Open';
  initial.current_end_time = '2026-10-10T12:03:45.000Z';
  const app = await browser(
    async () => '',
    '/',
    initial,
    'bidder',
    new Map(),
    async (url) => {
      if (failState && url === '/state') throw new Error('Unavailable new history');
    },
  );
  assert.equal(app.element('phase').textContent, 'Auction open');
  failState = true;
  app.changeAccess({
    role: 'bidder',
    bidder_id: 'demo-2',
    session_label: 'another-history',
    spec_revision: '0.3',
  });
  await app.refresh();
  for (const id of [
    'number',
    'phase',
    'clock',
    'clock-label',
    'clock-note',
    'bid-rule',
    'leader-label',
  ])
    assert.equal(app.element(id).textContent, '—', id);
  assert.equal(app.element('bid-button').disabled, true);
  assert.equal(app.requests.filter((r) => r.method === 'POST').length, 0);
});

test('session changes clear uncertainty and previously displayed facts without carrying a command', async () => {
  const app = await browser(async () => {
    throw new Error('lost');
  });
  await app.submit(command);
  const state = snapshot();
  state.current_number = 20;
  app.setState(state);
  app.changeAccess({
    role: 'bidder',
    bidder_id: 'demo-2',
    session_label: 'new-history',
    spec_revision: '0.3',
  });
  await app.refresh();
  assert.ok(app.element('identity').textContent.includes('Bidder B'));
  assert.equal(String(app.element('number').textContent), '20');
  assert.equal(app.element('uncertainty').hidden, true);
  assert.equal(app.element('last-command').textContent, 'No command submitted in this view.');
  assert.equal(app.requests.filter((r) => r.method === 'POST').length, 1);
});

test('older projections do not replace newer committed facts', async () => {
  const newer = snapshot();
  newer.current_number = 2;
  newer.last_sequence_index = 22;
  const app = await browser(async () => '', '/', newer);
  app.setState(snapshot());
  await app.refresh();
  assert.equal(String(app.element('number').textContent), '2');
});

test('a late command response cannot restore facts from the previous session', async () => {
  let release;
  const response = new Promise((resolve) => {
    release = resolve;
  });
  const app = await browser(() => response);
  const pending = app.submit(command);
  const next = {
    role: 'bidder',
    bidder_id: 'demo-2',
    session_label: 'another-history',
    spec_revision: '0.3',
  };
  const nextState = snapshot();
  nextState.current_number = 20;
  app.changeAccess(next);
  app.setState(nextState);
  app.acceptAccessNow(next);
  release(
    JSON.stringify({
      status: 'success',
      data: {
        accepted: true,
        command_records: [],
        evaluation_records: [],
        bid_record: { number: 1 },
        state: snapshot(),
      },
    }),
  );
  await pending;
  assert.equal(String(app.element('number').textContent), '20');
  assert.ok(app.element('result').textContent.includes('previous session separately'));
  assert.equal(app.element('uncertainty').hidden, true);
  assert.equal(app.requests.filter((r) => r.method === 'POST').length, 1);
});
