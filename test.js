import { strict as assert } from "assert";
import { Runner } from "near-runner"

// 1. Create testing accounts and deploy a contract
async function initRunner() {
  return await Runner.create(async ({ runtime }) => ({
    contract: await runtime.createAndDeploy(
      'status-message',
      "./res/status_message.wasm"
    ),
    alice: await runtime.createAccount('alice'),
    bob: await runtime.createAccount('bob'),
  }))
}

// 2. Alice sets then gets status
async function testAliceSetsStatus(runner) {
  await runner.run(async ({ alice, contract }) => {
    await alice.call(contract, "set_status", { message: "hello" })
    const result = await contract.view(
      "get_status",
      { account_id: alice.accountId }
    )
    assert.equal(result, "hello");
  })
}

// 3. Default status is null
async function testDefaultStatus(runner) {
  await runner.run(async ({ bob, contract }) => {
    const result = await contract.view(
      "get_status",
      { account_id: bob.accountId }
    )
    assert.equal(result, null)
  })
}

// 4. Alice and Bob have separate statuses
async function testStatusPerAccount(runner) {
  await runner.run(async ({ alice, bob, contract }) => {
    await bob.call(contract, "set_status", { message: "world" })
    const bobStatus = await contract.view(
      "get_status",
      { account_id: bob.accountId }
    )
    assert.equal(bobStatus, "world");

    const aliceStatus = await contract.view(
      "get_status",
      { account_id: alice.accountId }
    )
    assert.equal(aliceStatus, null)
  })
}

async function test() {
  const runner = await initRunner()
  await Promise.all([
    testAliceSetsStatus(runner),
    testDefaultStatus(runner),
    testStatusPerAccount(runner),
  ])
  console.log('\x1b[32mPASSED\x1b[0m')
}

test()
