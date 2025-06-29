[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-react](../README.md) / useRIDB

# Function: useRIDB()

> **useRIDB**\<`T`\>(): `object`

Defined in: [index.tsx:17](https://github.com/trust0-project/RIDB/blob/3305fbba832297c7c9cd86d5d666516d921be44e/packages/ridb-react/src/index.tsx#L17)

## Type Parameters

### T

`T` *extends* [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/docs/@trust0/ridb-core/type-aliases/SchemaTypeRecord.md)

## Returns

`object`

### db

> **db**: [`RIDB`](../../ridb/classes/RIDB.md)\<`T`\>

### setStartOptions()

> **setStartOptions**: (`options`) => `void`

#### Parameters

##### options

`StartOptions`\<`T`\>

#### Returns

`void`

### start()

> **start**: () => `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

### state

> **state**: [`DatabaseState`](../type-aliases/DatabaseState.md)

### stop()

> **stop**: () => `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>
