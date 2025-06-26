[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / InMemory

# Class: InMemory

Defined in: ridb\_core.js:1041

## Constructors

### Constructor

> **new InMemory**(): `InMemory`

#### Returns

`InMemory`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `undefined` \| `number`

Defined in: ridb\_core.js:1053

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `undefined` \| `number`

Defined in: ridb\_core.js:1051

#### Returns

`undefined` \| `number`

***

### close()

> **close**(): `Promise`\<`any`\>

Defined in: ridb\_core.js:1125

#### Returns

`Promise`\<`any`\>

***

### count()

> **count**(`collection_name`, `query_js`, `options`): `Promise`\<`any`\>

Defined in: ridb\_core.js:1114

#### Parameters

##### collection\_name

`string`

##### query\_js

`any`

##### options

[`QueryOptions`](QueryOptions.md)

#### Returns

`Promise`\<`any`\>

***

### find()

> **find**(`collection_name`, `query_js`, `options`): `Promise`\<`any`\>

Defined in: ridb\_core.js:1089

#### Parameters

##### collection\_name

`string`

##### query\_js

`any`

##### options

[`QueryOptions`](QueryOptions.md)

#### Returns

`Promise`\<`any`\>

***

### findDocumentById()

> **findDocumentById**(`collection_name`, `primary_key`): `Promise`\<`any`\>

Defined in: ridb\_core.js:1102

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

Defined in: ridb\_core.js:1058

#### Returns

`void`

***

### start()

> **start**(): `Promise`\<`any`\>

Defined in: ridb\_core.js:1132

#### Returns

`Promise`\<`any`\>

***

### write()

> **write**(`op`): `Promise`\<`any`\>

Defined in: ridb\_core.js:1077

#### Parameters

##### op

[`Operation`](Operation.md)

#### Returns

`Promise`\<`any`\>

***

### \_\_wrap()

> `static` **\_\_wrap**(`ptr`): `any`

Defined in: ridb\_core.js:1043

#### Parameters

##### ptr

`any`

#### Returns

`any`

***

### create()

> `static` **create**(`name`, `schemas_js`): `Promise`\<`InMemory`\>

Defined in: ridb\_core.js:1067

#### Parameters

##### name

`string`

##### schemas\_js

`object`

#### Returns

`Promise`\<`InMemory`\>
