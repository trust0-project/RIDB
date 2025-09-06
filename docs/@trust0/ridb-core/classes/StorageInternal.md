[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / StorageInternal

# Class: `abstract` StorageInternal\<Schemas\>

Defined in: ridb\_core.d.ts:589

## Extended by

- [`BaseStorage`](BaseStorage.md)

## Type Parameters

### Schemas

`Schemas` *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### Constructor

> **new StorageInternal**\<`Schemas`\>(`name`, `schemas`): `StorageInternal`\<`Schemas`\>

Defined in: ridb\_core.d.ts:590

#### Parameters

##### name

`string`

##### schemas

`Schemas`

#### Returns

`StorageInternal`\<`Schemas`\>

## Methods

### close()

> `abstract` **close**(): `Promise`\<`void`\>

Defined in: ridb\_core.d.ts:595

#### Returns

`Promise`\<`void`\>

***

### count()

> `abstract` **count**(`colectionName`, `query`, `options?`): `Promise`\<`number`\>

Defined in: ridb\_core.d.ts:596

#### Parameters

##### colectionName

keyof `Schemas`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`Schemas`\[keyof `Schemas`\]\>

##### options?

[`QueryOptions`](../type-aliases/QueryOptions.md)

#### Returns

`Promise`\<`number`\>

***

### find()

> `abstract` **find**(`collectionName`, `query`, `options?`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>[]\>

Defined in: ridb\_core.d.ts:605

#### Parameters

##### collectionName

keyof `Schemas`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`Schemas`\[keyof `Schemas`\]\>

##### options?

[`QueryOptions`](../type-aliases/QueryOptions.md)

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>[]\>

***

### findDocumentById()

> `abstract` **findDocumentById**(`collectionName`, `id`): `Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: ridb\_core.d.ts:601

#### Parameters

##### collectionName

keyof `Schemas`

##### id

`string`

#### Returns

`Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

***

### start()

> `abstract` **start**(): `Promise`\<`void`\>

Defined in: ridb\_core.d.ts:594

#### Returns

`Promise`\<`void`\>

***

### write()

> `abstract` **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: ridb\_core.d.ts:610

#### Parameters

##### op

[`Operation`](../type-aliases/Operation.md)\<`Schemas`\[keyof `Schemas`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>
