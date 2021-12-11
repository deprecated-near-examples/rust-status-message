import {Workspace} from 'near-workspaces-ava';

const workspace = Workspace.init(async ({root}) => {
  const alice = await root.createAccount('alice');

  // Create a subaccount of the root account, and also deploy a contract to it
  const contract = await root.createAndDeploy(
    // Subaccount name
    'status-message',
    // Relative path (from package.json location) to the compiled contract file
    // which will be deployed to this account
    '../res/status_message.wasm',
  );

  return {alice, contract};
});
workspace.test('set get message', async(test, {alice, contract, root})=>{
  await alice.call(contract, 'set_status', {message: 'hello'});
  const aliceStatus = await contract.view('get_status', {account_id: alice});

  test.is(aliceStatus, 'hello');
});

workspace.test('get nonexistent message', async (test, {alice, contract, root})=>{
  const message: null = await contract.view('get_status', {account_id: root});

  test.is(message, null);
});