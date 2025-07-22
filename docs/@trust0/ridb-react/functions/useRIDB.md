[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-react](../README.md) / useRIDB

# Function: useRIDB()

> **useRIDB**\<`T`\>(): `object`

Defined in: [index.tsx:17](https://github.com/trust0-project/RIDB/blob/bcb8934543fc732545664233aa8443da39141d02/packages/ridb-react/src/index.tsx#L17)

## Type Parameters

### T

`T` *extends* [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/type-aliases/SchemaTypeRecord.md)

## Returns

`object`

### db

> **db**: [`RIDB`](../../ridb/classes/RIDB.md)\<`T`\>

### setStartOptions()

> **setStartOptions**: (`options`) => `void`

#### Parameters

##### options

[`StartOptions`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb/type-aliases/StartOptions.md)\<`T`\>

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
