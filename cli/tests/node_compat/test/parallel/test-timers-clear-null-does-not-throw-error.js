// deno-fmt-ignore-file
// deno-lint-ignore-file

// Copyright Joyent and Node contributors. All rights reserved. MIT license.
// Taken from Node 18.12.1
// This file is automatically generated by `tools/node_compat/setup.ts`. Do not modify this file manually.

'use strict';
require('../common');

// This test makes sure clearing timers with
// 'null' or no input does not throw error
clearInterval(null);
clearInterval();
clearTimeout(null);
clearTimeout();
clearImmediate(null);
clearImmediate();
