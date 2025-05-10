[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / Property

# Class: Property

Defined in: ridb\_core.js:1412

Represents a property within a schema, including type, items, length constraints, and other attributes.

## Constructors

### Constructor

> **new Property**(): `Property`

#### Returns

`Property`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `undefined` \| `number`

Defined in: ridb\_core.js:1416

## Accessors

### items

#### Get Signature

> **get** **items**(): `any`

Defined in: ridb\_core.js:1468

Retrieves the items of the property.

# Returns

* `Result<JsValue, JsValue>` - A result containing the items as a `JsValue` or an error.

##### Returns

`any`

***

### maxItems

#### Get Signature

> **get** **maxItems**(): `any`

Defined in: ridb\_core.js:1491

Retrieves the maximum number of items of the property.

# Returns

* `Result<JsValue, JsValue>` - A result containing the maximum number of items as a `JsValue` or an error.

##### Returns

`any`

***

### maxLength

#### Get Signature

> **get** **maxLength**(): `any`

Defined in: ridb\_core.js:1537

Retrieves the maximum length of the property.

# Returns

* `Result<JsValue, JsValue>` - A result containing the maximum length as a `JsValue` or an error.

##### Returns

`any`

***

### minItems

#### Get Signature

> **get** **minItems**(): `any`

Defined in: ridb\_core.js:1514

Retrieves the minimum number of items of the property.

# Returns

* `Result<JsValue, JsValue>` - A result containing the minimum number of items as a `JsValue` or an error.

##### Returns

`any`

***

### minLength

#### Get Signature

> **get** **minLength**(): `any`

Defined in: ridb\_core.js:1560

Retrieves the minimum length of the property.

# Returns

* `Result<JsValue, JsValue>` - A result containing the minimum length as a `JsValue` or an error.

##### Returns

`any`

***

### properties

#### Get Signature

> **get** **properties**(): `any`

Defined in: ridb\_core.js:1583

Retrieves the nested properties of the property.

# Returns

* `Result<JsValue, JsValue>` - A result containing the nested properties as a `JsValue` or an error.

##### Returns

`any`

***

### type

#### Get Signature

> **get** **type**(): `any`

Defined in: ridb\_core.js:1456

Retrieves the type of the property.

# Returns

* `PropertyType` - The type of the property.

##### Returns

`any`

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `undefined` \| `number`

Defined in: ridb\_core.js:1414

#### Returns

`undefined` \| `number`

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:1421

#### Returns

`void`

***

### is\_valid()

> **is\_valid**(): `boolean`

Defined in: ridb\_core.js:1433

Checks is the schema is valid.

# Returns

Throws exception if not valid

#### Returns

`boolean`
