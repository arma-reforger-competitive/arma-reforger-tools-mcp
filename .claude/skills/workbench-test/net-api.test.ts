import assert from "node:assert/strict";
import { test } from "node:test";
import { answer, defaultGateway, request } from "./net-api.ts";

test("a request has the version, the client ID, the content type, and the payload, as the wiki shows", () => {
  // Counted by hand from the wiki page "Workbench NET API": a 4-byte little-endian integer, then
  // three strings, each with a 4-byte little-endian length.
  const expected = Buffer.concat([
    Buffer.from([1, 0, 0, 0]),
    Buffer.from([13, 0, 0, 0]),
    Buffer.from("reforger-test"),
    Buffer.from([7, 0, 0, 0]),
    Buffer.from("JsonRPC"),
    Buffer.from([2, 0, 0, 0]),
    Buffer.from("{}"),
  ]);
  assert.deepEqual(request("{}"), expected);
});

test("a string length counts bytes, not characters", () => {
  // "é" is 2 bytes in UTF-8.
  assert.deepEqual(request("é").subarray(-6), Buffer.from([2, 0, 0, 0, 0xc3, 0xa9]));
});

test("an answer gives the status and the payload that Workbench sent", () => {
  // The bytes that Workbench 1.6 sent to GetLoadedProjects with the BridgeTest mod open.
  const bytes = Buffer.from('\x02\x00\x00\x00Ok1\x00\x00\x00{"Loaded Projects":["ArmaReforger","BridgeTest"]}', "latin1");
  assert.deepEqual(answer(bytes), { status: "Ok", payload: '{"Loaded Projects":["ArmaReforger","BridgeTest"]}' });
});

test("an error status with no payload is a whole answer", () => {
  const bytes = Buffer.from("\x12\x00\x00\x00Undefined API func", "latin1");
  assert.deepEqual(answer(bytes), { status: "Undefined API func", payload: "" });
});

test("an answer that ends too early or has extra bytes is refused", () => {
  for (const text of ["", "\x02\x00", "\x09\x00\x00\x00Ok", "\x02\x00\x00\x00Ok\x05\x00"])
    assert.throws(() => answer(Buffer.from(text, "latin1")), /ends too early/, JSON.stringify(text));
  assert.throws(() => answer(Buffer.from("\x02\x00\x00\x00Ok\x02\x00\x00\x00{}xyz", "latin1")), /3 bytes after the payload/);
});

test("the Windows host under WSL is the gateway of the default route", () => {
  // The route table of a WSL 2 computer. `ip route show default` gives 172.17.240.1 for it.
  const routes =
    "Iface\tDestination\tGateway \tFlags\tRefCnt\tUse\tMetric\tMask\t\tMTU\tWindow\tIRTT\n" +
    "eth0\t00F011AC\t00000000\t0001\t0\t0\t0\t00F0FFFF\t0\t0\t0\n" +
    "eth0\t00000000\t01F011AC\t0003\t0\t0\t0\t00000000\t0\t0\t0\n";
  assert.equal(defaultGateway(routes), "172.17.240.1");
  assert.equal(defaultGateway("Iface\tDestination\tGateway\n"), undefined);
});
