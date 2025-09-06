[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb](../README.md) / RIDBAbstract

# Interface: RIDBAbstract\<T\>

Defined in: [types.ts:104](https://github.com/trust0-project/RIDB/blob/9786676f4132a55aaec34d1edb0da16200ab0eba/packages/ridb/src/types.ts#L104)

Abstract interface for RIDB implementations.

Defines the core operations that any RIDB adapter must implement.

## Type Parameters

### T

`T` *extends* [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/type-aliases/SchemaTypeRecord.md)

The schema type record defining the database structure

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [types.ts:118](https://github.com/trust0-project/RIDB/blob/9786676f4132a55aaec34d1edb0da16200ab0eba/packages/ridb/src/types.ts#L118)

Close the database connection.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the database has been successfully closed

***

### getCollections()

> **getCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [types.ts:125](https://github.com/trust0-project/RIDB/blob/9786676f4132a55aaec34d1edb0da16200ab0eba/packages/ridb/src/types.ts#L125)

Get the collections for this database.

#### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

An object containing all collections defined in the schema

***

### isStarted()

> **isStarted**(): `boolean`

Defined in: [types.ts:132](https://github.com/trust0-project/RIDB/blob/9786676f4132a55aaec34d1edb0da16200ab0eba/packages/ridb/src/types.ts#L132)

Check if the database has been started.

#### Returns

`boolean`

True if the database is started, false otherwise

***

### start()

> **start**(`options?`): `Promise`\<`void`\>

Defined in: [types.ts:111](https://github.com/trust0-project/RIDB/blob/9786676f4132a55aaec34d1edb0da16200ab0eba/packages/ridb/src/types.ts#L111)

Start the database with the given options.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

Optional configuration for startup

#### Returns

`Promise`\<`void`\>

A promise that resolves when the database has successfully started
