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
    title: null,
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
async function browser(reply) {
  const elements = new Map(),
    requests = [];
  let current = snapshot();
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
    location: { pathname: '/' },
    performance: { now: () => 0 },
    setInterval() {},
    fetch: async (url, options = {}) => {
      requests.push({ url, ...options });
      if (options.method === 'POST') {
        const raw = await reply();
        const parsed = JSON.parse(raw);
        if (parsed.data?.state) current = parsed.data.state;
        return { text: async () => raw };
      }
      const data = url === '/state' ? current : { records: [], pagination: { next_offset: null } };
      return { text: async () => JSON.stringify({ status: 'success', data }) };
    },
  });
  vm.runInContext(source, context);
  await new Promise((resolve) => setImmediate(resolve));
  return {
    element,
    requests,
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
      data: { accepted: true, bid_record: { number: 1 }, state },
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
