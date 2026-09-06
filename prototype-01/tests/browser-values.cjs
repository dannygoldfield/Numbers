const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');
const path = require('node:path');

// Test the actual presentation parser without a browser or a second implementation.
const source = fs.readFileSync(path.join(__dirname, '../static/app.js'), 'utf8');
const helpers = source.slice(0, source.indexOf('const element ='));
const sandbox = vm.createContext({});
vm.runInContext(helpers + '\nthis.api = { parseExact, prettyExact };', sandbox);
const { parseExact, prettyExact } = sandbox.api;

test('economic integers beyond Number precision survive display unchanged', () => {
  const raw = '{"amount":10000000000000000000000000000000000000003,"balance":0}';
  assert.equal(String(parseExact(raw).amount), '10000000000000000000000000000000000000003');
  assert.equal(prettyExact(parseExact(raw)).replace(/\s/g, ''), raw);
});
test('numeric-looking strings remain strings, including quoted commands', () => {
  const raw = '{"bidder_id":"123","command":"{\\"amount\\":10}","array":[10,"10",null,true]}';
  const rendered = prettyExact(parseExact(raw));
  assert.deepEqual(JSON.parse(rendered), JSON.parse(raw));
  assert.equal(typeof parseExact(raw).bidder_id, 'string');
});
test('Unicode, escapes, and negative integers preserve their display meaning', () => {
  const raw = '{"text":"é/\\n\\t\\u0001\\\"","negative":-123,"nested":{"sequence":31}}';
  assert.deepEqual(JSON.parse(prettyExact(parseExact(raw))), JSON.parse(raw));
});
