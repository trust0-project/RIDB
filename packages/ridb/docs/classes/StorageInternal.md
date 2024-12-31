[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / StorageInternal

# Class: `abstract` StorageInternal\<Schemas\>

## Extended by

- [`BaseStorage`](BaseStorage.md)

## Type Parameters

• **Schemas** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### new StorageInternal()

> **new StorageInternal**\<`Schemas`\>(`name`, `schemas`): [`StorageInternal`](StorageInternal.md)\<`Schemas`\>

#### Parameters

##### name

`string`

##### schemas

`Schemas`

#### Returns

[`StorageInternal`](StorageInternal.md)\<`Schemas`\>

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:222

## Methods

### close()

> `abstract` **close**(): `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:227

***

### count()

> `abstract` **count**(`colectionName`, `query`): `Promise`\<`number`\>

#### Parameters

##### colectionName

keyof `Schemas`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`Schemas`\[keyof `Schemas`\]\>

#### Returns

`Promise`\<`number`\>

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:228

***

### find()

> `abstract` **find**(`collectionName`, `query`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>[]\>

#### Parameters

##### collectionName

keyof `Schemas`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`Schemas`\[keyof `Schemas`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>[]\>

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:236

***

### findDocumentById()

> `abstract` **findDocumentById**(`collectionName`, `id`): `Promise`\<`undefined` \| `null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Parameters

##### collectionName

keyof `Schemas`

##### id

`string`

#### Returns

`Promise`\<`undefined` \| `null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:232

***

### start()

> `abstract` **start**(): `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:226

***

### write()

> `abstract` **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Parameters

##### op

[`Operation`](../type-aliases/Operation.md)\<`Schemas`\[keyof `Schemas`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:240
