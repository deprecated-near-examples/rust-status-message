import { Worker, NEAR, NearAccount } from "near-workspaces";
import anyTest, { TestFn } from "ava";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // deploy contract
  const root = worker.rootAccount;
  const contract = await root.devDeploy(
    "./res/status_message.wasm",
    { initialBalance: NEAR.parse("30 N").toJSON() }
  );

  // some test accounts
  const alice = await root.createSubAccount("alice", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });
  const bob = await root.createSubAccount("bob", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });
  const charlie = await root.createSubAccount("charlie", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract, alice, bob, charlie };
});

test.afterEach(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("set get message", async (t) => {
  const { contract, alice } = t.context.accounts;
  await alice.call(contract, "set_status", { message: "hello" });
  const aliceStatus = await contract.view("get_status", { account_id: alice });
  t.is(aliceStatus, "hello");
});

test("get nonexistent message", async (t) => {
  const { root, contract } = t.context.accounts;
  const message: null = await contract.view("get_status", {
    account_id: root,
  });
  t.is(message, null);
});

test("root and alice have different statuses", async (t) => {
  const { root, contract, alice } = t.context.accounts;
  await root.call(contract, "set_status", { message: "world" });
  const rootStatus = await contract.view("get_status", { account_id: root });
  t.is(rootStatus, "world");
  const aliceStatus = await contract.view("get_status", { account_id: alice });
  t.is(aliceStatus, null);
});
