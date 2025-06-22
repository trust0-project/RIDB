[**@trust0/ridb-react**](../README.md)

***

[@trust0/ridb-react](../README.md) / useRIDB

# Function: useRIDB()

> **useRIDB**\<`T`\>(): `object`

Defined in: [index.tsx:16](https://github.com/trust0-project/RIDB/blob/4b79ddca2ee05b225072db9bc2dce55017bb2d2e/packages/ridb-react/src/index.tsx#L16)

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord`

## Returns

`object`

### db

> **db**: `RIDB`\<`T`\>

### start()

> **start**: (`options`) => `Promise`\<`void`\>

#### Parameters

##### options

`StartOptions`\<`T`\>

#### Returns

`Promise`\<`void`\>

### state

> **state**: [`DatabaseState`](../type-aliases/DatabaseState.md)

### stop()

> **stop**: () => `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>
