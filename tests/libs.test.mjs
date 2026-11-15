import assert from 'node:assert/strict';
import test from 'node:test';
import { serviceCatalog, validateLead, makeEvent, classifyPriority } from '../src/index.mjs';

test('catalog exposes org metadata', () => { assert.equal(serviceCatalog.org, "apostille-me"); assert.ok(serviceCatalog.integrations.length >= 4); });
test('lead validation normalizes email', () => { const result = validateLead({ name: 'Alex', email: ' ALEX@EXAMPLE.COM ' }); assert.equal(result.ok, true); assert.equal(result.value.email, 'alex@example.com'); });
test('event factory creates namespaced events', () => { const event = makeEvent('demo.created', { value: 1 }, { id: 'evt_test', occurredAt: '2026-08-04T00:00:00.000Z' }); assert.equal(event.id, 'evt_test'); assert.equal(event.product, "apostille-me"); assert.throws(() => makeEvent('Bad Event')); });
test('priority classification is stable', () => { assert.equal(classifyPriority({ severity: 'critical' }), 'urgent'); assert.equal(classifyPriority({ severity: 'SEV2' }), 'high'); assert.equal(classifyPriority({ severity: 'warn' }), 'normal'); assert.equal(classifyPriority({}), 'low'); });


test('events require nonempty, lowercase namespace segments', () => {
  const valid = makeEvent('case_record.created-v2', {}, { id: 'evt_contract', occurredAt: '2026-01-01T00:00:00Z' });
  assert.equal(valid.type, 'case_record.created-v2');
  for (const invalid of ['created', '.case', 'case.', 'case..created', 'case.1created', 'Case.created', 'case. CREATED']) {
    assert.throws(() => makeEvent(invalid), TypeError, invalid);
  }
});
