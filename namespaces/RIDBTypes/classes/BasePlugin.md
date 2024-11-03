[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / BasePlugin

# Class: BasePlugin

## Extended by

- [`EncryptionPlugin`](EncryptionPlugin.md)

## Constructors

### new BasePlugin()

> **new BasePlugin**(): [`BasePlugin`](BasePlugin.md)

#### Returns

[`BasePlugin`](BasePlugin.md)

## Properties

### docCreateHook()?

> `optional` **docCreateHook**: (`schema`, `doc`) => [`Doc`](../type-aliases/Doc.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

#### Parameters

• **schema**: [`Schema`](Schema.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

• **doc**: [`Doc`](../type-aliases/Doc.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

#### Returns

[`Doc`](../type-aliases/Doc.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

#### Defined in

pkg/ridb\_rust.d.ts:548

***

### docRecoverHook()?

> `optional` **docRecoverHook**: (`schema`, `doc`) => [`Doc`](../type-aliases/Doc.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

#### Parameters

• **schema**: [`Schema`](Schema.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

• **doc**: [`Doc`](../type-aliases/Doc.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

#### Returns

[`Doc`](../type-aliases/Doc.md)\<[`SchemaType`](../type-aliases/SchemaType.md)\>

#### Defined in

pkg/ridb\_rust.d.ts:549

## Methods

### free()

> **free**(): `void`

Frees the resources used by the plugin.

#### Returns

`void`

#### Defined in

pkg/ridb\_rust.d.ts:547
