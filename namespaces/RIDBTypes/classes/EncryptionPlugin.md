[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / EncryptionPlugin

# Class: EncryptionPlugin

## Extends

- [`BasePlugin`](BasePlugin.md)

## Constructors

### new EncryptionPlugin()

> **new EncryptionPlugin**(): [`EncryptionPlugin`](EncryptionPlugin.md)

#### Returns

[`EncryptionPlugin`](EncryptionPlugin.md)

#### Inherited from

[`BasePlugin`](BasePlugin.md).[`constructor`](BasePlugin.md#constructors)

## Properties

### docCreateHook()?

> `optional` **docCreateHook**: (`schema`, `doc`) => [`Doc`](../type-aliases/Doc.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

#### Parameters

• **schema**: [`Schema`](Schema.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

• **doc**: [`Doc`](../type-aliases/Doc.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

#### Returns

[`Doc`](../type-aliases/Doc.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

#### Overrides

[`BasePlugin`](BasePlugin.md).[`docCreateHook`](BasePlugin.md#doccreatehook)

#### Defined in

pkg/ridb\_rust.d.ts:532

***

### docRecoverHook()?

> `optional` **docRecoverHook**: (`schema`, `doc`) => [`Doc`](../type-aliases/Doc.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

#### Parameters

• **schema**: [`Schema`](Schema.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

• **doc**: [`Doc`](../type-aliases/Doc.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

#### Returns

[`Doc`](../type-aliases/Doc.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

#### Overrides

[`BasePlugin`](BasePlugin.md).[`docRecoverHook`](BasePlugin.md#docrecoverhook)

#### Defined in

pkg/ridb\_rust.d.ts:533

## Methods

### free()

> **free**(): `void`

Creates a new EncryptionPlugin instance

#### Returns

`void`

#### Overrides

[`BasePlugin`](BasePlugin.md).[`free`](BasePlugin.md#free)

#### Defined in

pkg/ridb\_rust.d.ts:531
