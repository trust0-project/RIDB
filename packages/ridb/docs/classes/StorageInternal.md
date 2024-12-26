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

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:645

## Methods

### close()

> `abstract` **close**(): `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:650

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

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:651

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

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:659

***

### findDocumentById()

> `abstract` **findDocumentById**(`collectionName`, `id`): `Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Parameters

##### collectionName

keyof `Schemas`

##### id

`string`

#### Returns

`Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:655

***

### start()

> `abstract` **start**(): `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:649

***

### write()

> `abstract` **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Parameters

##### op

[`Operation`](../type-aliases/Operation.md)\<`Schemas`\[keyof `Schemas`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:663
