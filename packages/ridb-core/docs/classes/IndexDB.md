[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / IndexDB

# Class: IndexDB

Defined in: ridb\_core.js:1142

## Constructors

### Constructor

> **new IndexDB**(): `IndexDB`

#### Returns

`IndexDB`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `undefined` \| `number`

Defined in: ridb\_core.js:1154

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `undefined` \| `number`

Defined in: ridb\_core.js:1152

#### Returns

`undefined` \| `number`

***

### close()

> **close**(): `Promise`\<`any`\>

Defined in: ridb\_core.js:1265

#### Returns

`Promise`\<`any`\>

***

### count()

> **count**(`collection_name`, `query`, `options`): `Promise`\<`any`\>

Defined in: ridb\_core.js:1254

#### Parameters

##### collection\_name

`string`

##### query

`any`

##### options

[`QueryOptions`](QueryOptions.md)

#### Returns

`Promise`\<`any`\>

***

### find()

> **find**(`collection_name`, `query`, `options`): `Promise`\<`any`\>

Defined in: ridb\_core.js:1229

#### Parameters

##### collection\_name

`string`

##### query

`any`

##### options

[`QueryOptions`](QueryOptions.md)

#### Returns

`Promise`\<`any`\>

***

### findDocumentById()

> **findDocumentById**(`collection_name`, `primary_key`): `Promise`\<`any`\>

Defined in: ridb\_core.js:1242

#### Parameters

##### collection\_name

`string`

##### primary\_key

`any`

#### Returns

`Promise`\<`any`\>

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:1159

#### Returns

`void`

***

### get\_store()

> **get\_store**(`store_name`): `IDBObjectStore`

Defined in: ridb\_core.js:1185

#### Parameters

##### store\_name

`string`

#### Returns

`IDBObjectStore`

***

### get\_stores()

> **get\_stores**(): `string`[]

Defined in: ridb\_core.js:1168

Fetch documents by opening an IndexedDB cursor (on an index or store),
then apply inline filtering and offset/limit constraints.

#### Returns

`string`[]

***

### start()

> **start**(): `Promise`\<`any`\>

Defined in: ridb\_core.js:1272

#### Returns

`Promise`\<`any`\>

***

### write()

> **write**(`op`): `Promise`\<`any`\>

Defined in: ridb\_core.js:1217

#### Parameters

##### op

[`Operation`](Operation.md)

#### Returns

`Promise`\<`any`\>

***

### \_\_wrap()

> `static` **\_\_wrap**(`ptr`): `any`

Defined in: ridb\_core.js:1144

#### Parameters

##### ptr

`any`

#### Returns

`any`

***

### create()

> `static` **create**(`name`, `schemas_js`): `Promise`\<`IndexDB`\>

Defined in: ridb\_core.js:1207

#### Parameters

##### name

`string`

##### schemas\_js

`object`

#### Returns

`Promise`\<`IndexDB`\>
