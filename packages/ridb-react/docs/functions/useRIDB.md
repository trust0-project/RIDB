[**@trust0/ridb-react**](../README.md)

***

[@trust0/ridb-react](../README.md) / useRIDB

# Function: useRIDB()

> **useRIDB**\<`T`\>(): `object`

Defined in: [index.tsx:17](https://github.com/trust0-project/RIDB/blob/132707f2a67423a7d9b542db2e8d36b2c6b917ad/packages/ridb-react/src/index.tsx#L17)

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord`

## Returns

`object`

### db

> **db**: `RIDB`\<`T`\>

### setStartOptions()

> **setStartOptions**: (`options?`) => `void`

#### Parameters

##### options?

`StartOptions`\<`T`\>

#### Returns

`void`

### start()

> **start**: (`options?`) => `Promise`\<`void`\>

#### Parameters

##### options?

`StartOptions`\<`T`\>

#### Returns

`Promise`\<`void`\>

### state

> **state**: [`DatabaseState`](../type-aliases/DatabaseState.md)

### stop()

> **stop**: () => `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>
