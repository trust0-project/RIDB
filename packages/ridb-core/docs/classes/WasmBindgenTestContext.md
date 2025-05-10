[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / WasmBindgenTestContext

# Class: WasmBindgenTestContext

Defined in: ridb\_core.js:2214

Runtime test harness support instantiated in JS.

The node.js entry script instantiates a `Context` here which is used to
drive test execution.

## Constructors

### Constructor

> **new WasmBindgenTestContext**(): `WasmBindgenTestContext`

Defined in: ridb\_core.js:2234

Creates a new context ready to run tests.

A `Context` is the main structure through which test execution is
coordinated, and this will collect output and results for all executed
tests.

#### Returns

`WasmBindgenTestContext`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `number`

Defined in: ridb\_core.js:2218

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `number`

Defined in: ridb\_core.js:2216

#### Returns

`number`

***

### args()

> **args**(`args`): `void`

Defined in: ridb\_core.js:2244

Inform this context about runtime arguments passed to the test
harness.

#### Parameters

##### args

`any`[]

#### Returns

`void`

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:2223

#### Returns

`void`

***

### run()

> **run**(`tests`): `Promise`\<`any`\>

Defined in: ridb\_core.js:2262

Executes a list of tests, returning a promise representing their
eventual completion.

This is the main entry point for executing tests. All the tests passed
in are the JS `Function` object that was plucked off the
`WebAssembly.Instance` exports list.

The promise returned resolves to either `true` if all tests passed or
`false` if at least one test failed.

#### Parameters

##### tests

`any`[]

#### Returns

`Promise`\<`any`\>
