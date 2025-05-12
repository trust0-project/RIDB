[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDBAbstract

# Interface: RIDBAbstract\<T\>

Defined in: [types.ts:46](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/types.ts#L46)

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord`

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [types.ts:55](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/types.ts#L55)

Close the database connection

#### Returns

`Promise`\<`void`\>

***

### getCollections()

> **getCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [types.ts:60](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/types.ts#L60)

Get the collections for this database

#### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

***

### isStarted()

> **isStarted**(): `boolean`

Defined in: [types.ts:65](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/types.ts#L65)

Check if the database has been started

#### Returns

`boolean`

***

### start()

> **start**(`options?`): `Promise`\<`void`\>

Defined in: [types.ts:50](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/types.ts#L50)

Start the database with the given options

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>
