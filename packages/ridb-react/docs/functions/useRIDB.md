[**@trust0/ridb-react**](../README.md)

***

[@trust0/ridb-react](../README.md) / useRIDB

# Function: useRIDB()

> **useRIDB**\<`T`\>(): `object`

Defined in: [index.tsx:17](https://github.com/trust0-project/RIDB/blob/3acbe8c131d5e3c448607928fea7f7a9f9b602e4/packages/ridb-react/src/index.tsx#L17)

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord`

## Returns

`object`

### db

> **db**: `RIDB`\<`T`\>

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
