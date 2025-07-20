[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb](../README.md) / RIDBAbstract

# Interface: RIDBAbstract\<T\>

Defined in: [types.ts:112](https://github.com/trust0-project/RIDB/blob/602ba5be0c26a543e979344cf0da5288a98fa66c/packages/ridb/src/types.ts#L112)

Abstract interface for RIDB implementations.

Defines the core operations that any RIDB adapter must implement.

## Type Parameters

### T

`T` *extends* [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/type-aliases/SchemaTypeRecord.md)

The schema type record defining the database structure

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [types.ts:126](https://github.com/trust0-project/RIDB/blob/602ba5be0c26a543e979344cf0da5288a98fa66c/packages/ridb/src/types.ts#L126)

Close the database connection.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the database has been successfully closed

***

### getCollections()

> **getCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [types.ts:133](https://github.com/trust0-project/RIDB/blob/602ba5be0c26a543e979344cf0da5288a98fa66c/packages/ridb/src/types.ts#L133)

Get the collections for this database.

#### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

An object containing all collections defined in the schema

***

### isStarted()

> **isStarted**(): `boolean`

Defined in: [types.ts:140](https://github.com/trust0-project/RIDB/blob/602ba5be0c26a543e979344cf0da5288a98fa66c/packages/ridb/src/types.ts#L140)

Check if the database has been started.

#### Returns

`boolean`

True if the database is started, false otherwise

***

### start()

> **start**(`options?`): `Promise`\<`void`\>

Defined in: [types.ts:119](https://github.com/trust0-project/RIDB/blob/602ba5be0c26a543e979344cf0da5288a98fa66c/packages/ridb/src/types.ts#L119)

Start the database with the given options.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

Optional configuration for startup

#### Returns

`Promise`\<`void`\>

A promise that resolves when the database has successfully started
