import {Worker, NEAR, NearAccount} from 'near-workspaces';
import anyTest, {TestFn} from 'ava';

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async t => {
  const worker = await Worker.init();
  const root = worker.rootAccount;
  const contract = await root.createAndDeploy(
    root.getSubAccount('status-message').accountId,
    './res/status_message.wasm',
    {initialBalance: NEAR.parse('3 N').toJSON()},
  );
  const alice = await root.createSubAccount('alice', {initialBalance: NEAR.parse('3 N').toJSON()});

  t.context.worker = worker;
  t.context.accounts = {root, contract, alice};
});

test.afterEach(async t => {
  await t.context.worker.tearDown().catch(error => {
    console.log('Failed to stop the Sandbox:', error);
  });
});

test('set get message', async t => {
  const {alice, contract} = t.context.accounts;
  await alice.call(contract, 'set_status', {message: 'hello'});
  const aliceStatus = await contract.view('get_status', {account_id: alice});
  t.is(aliceStatus, 'hello');
});

test('get message not existing message', async t => {
  const {root, contract} = t.context.accounts;
  const message: null = await contract.view('get_status', {account_id: root});
  t.is(message, null);
});