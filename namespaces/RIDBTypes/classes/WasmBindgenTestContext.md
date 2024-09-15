[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / WasmBindgenTestContext

# Class: WasmBindgenTestContext

Runtime test harness support instantiated in JS.

The node.js entry script instantiates a `Context` here which is used to
drive test execution.

## Constructors

### new WasmBindgenTestContext()

> **new WasmBindgenTestContext**(): [`WasmBindgenTestContext`](WasmBindgenTestContext.md)

Creates a new context ready to run tests.

A `Context` is the main structure through which test execution is
coordinated, and this will collect output and results for all executed
tests.

#### Returns

[`WasmBindgenTestContext`](WasmBindgenTestContext.md)

#### Defined in

pkg/ridb\_rust.d.ts:663

## Methods

### args()

> **args**(`args`): `void`

Inform this context about runtime arguments passed to the test
harness.

#### Parameters

• **args**: `any`[]

#### Returns

`void`

#### Defined in

pkg/ridb\_rust.d.ts:669

***

### free()

> **free**(): `void`

#### Returns

`void`

#### Defined in

pkg/ridb\_rust.d.ts:655

***

### run()

> **run**(`tests`): `Promise`\<`any`\>

Executes a list of tests, returning a promise representing their
eventual completion.

This is the main entry point for executing tests. All the tests passed
in are the JS `Function` object that was plucked off the
`WebAssembly.Instance` exports list.

The promise returned resolves to either `true` if all tests passed or
`false` if at least one test failed.

#### Parameters

• **tests**: `any`[]

#### Returns

`Promise`\<`any`\>

#### Defined in

pkg/ridb\_rust.d.ts:683
