// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
import {
  assert,
  assertEquals,
  assertNotEquals,
  deferred,
} from "./test_util.ts";

Deno.test({
  sanitizeOps: false,
  sanitizeResources: false,
}, async function queueTestNoDbClose() {
  const db: Deno.Kv = await Deno.openKv(":memory:");
  const promise = deferred();
  let dequeuedMessage: unknown = null;
  db.listenQueue((msg) => {
    dequeuedMessage = msg;
    promise.resolve();
  });
  const res = await db.enqueue("test");
  assert(res.ok);
  assertNotEquals(res.versionstamp, null);
  await promise;
  assertEquals(dequeuedMessage, "test");
});
