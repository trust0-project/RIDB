[**@trust0/ridb-react**](../README.md)

***

[@trust0/ridb-react](../README.md) / useRIDB

# Function: useRIDB()

> **useRIDB**\<`T`\>(): `object`

Defined in: [index.tsx:16](https://github.com/trust0-project/RIDB/blob/104aa2879acd25a4cc9a5ad43a4aff29b2b5117a/packages/ridb-react/src/index.tsx#L16)

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
