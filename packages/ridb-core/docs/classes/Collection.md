[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / Collection

# Class: Collection

Defined in: ridb\_core.js:664

## Constructors

### Constructor

> **new Collection**(): `Collection`

#### Returns

`Collection`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `undefined` \| `number`

Defined in: ridb\_core.js:676

## Accessors

### name

#### Get Signature

> **get** **name**(): `string`

Defined in: ridb\_core.js:688

##### Returns

`string`

***

### schema

#### Get Signature

> **get** **schema**(): [`Schema`](Schema.md)

Defined in: ridb\_core.js:707

##### Returns

[`Schema`](Schema.md)

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `undefined` \| `number`

Defined in: ridb\_core.js:674

#### Returns

`undefined` \| `number`

***

### count()

> **count**(`query_js`, `options_js`): `Promise`\<`any`\>

Defined in: ridb\_core.js:763

counts and returns all documents in the collection.

This function is asynchronous and returns a `Schema` representing
the documents found in the collection.

#### Parameters

##### query\_js

`any`

##### options\_js

`any`

#### Returns

`Promise`\<`any`\>

***

### create()

> **create**(`document`): `Promise`\<`any`\>

Defined in: ridb\_core.js:804

Creates a new document in the collection.

This function is asynchronous and returns a `Result` indicating success or failure.

# Arguments

* `document` - A `JsValue` representing the document to create.

#### Parameters

##### document

`any`

#### Returns

`Promise`\<`any`\>

***

### delete()

> **delete**(`primary_key`): `Promise`\<`any`\>

Defined in: ridb\_core.js:815

Deletes a document from the collection by its ID.

This function is asynchronous.

#### Parameters

##### primary\_key

`any`

#### Returns

`Promise`\<`any`\>

***

### find()

> **find**(`query_js`, `options_js`): `Promise`\<`any`\>

Defined in: ridb\_core.js:731

Finds and returns all documents in the collection.

This function is asynchronous and returns a `JsValue` representing
the documents found in the collection.

#### Parameters

##### query\_js

`any`

##### options\_js

`any`

#### Returns

`Promise`\<`any`\>

***

### findById()

> **findById**(`primary_key`): `Promise`\<`any`\>

Defined in: ridb\_core.js:774

Finds and returns a single document in the collection by its ID.

This function is asynchronous.

#### Parameters

##### primary\_key

`any`

#### Returns

`Promise`\<`any`\>

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:681

#### Returns

`void`

***

### parse\_query\_options()

> **parse\_query\_options**(`options`): [`QueryOptions`](QueryOptions.md)

Defined in: ridb\_core.js:739

#### Parameters

##### options

`any`

#### Returns

[`QueryOptions`](QueryOptions.md)

***

### update()

> **update**(`document`): `Promise`\<`any`\>

Defined in: ridb\_core.js:789

Updates a document in the collection with the given data.

This function is asynchronous and returns a `Result` indicating success or failure.

# Arguments

* `document` - A `JsValue` representing the partial document to update.

#### Parameters

##### document

`any`

#### Returns

`Promise`\<`any`\>

***

### \_\_wrap()

> `static` **\_\_wrap**(`ptr`): `any`

Defined in: ridb\_core.js:666

#### Parameters

##### ptr

`any`

#### Returns

`any`
