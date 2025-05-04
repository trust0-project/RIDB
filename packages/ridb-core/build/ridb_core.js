if (typeof Buffer === 'undefined') {
global.Buffer = require('buffer').Buffer;
}


// pkg/ridb_core.js
var wasm;
var heap = new Array(128).fill(void 0);
heap.push(void 0, null, true, false);
function getObject(idx) {
  return heap[idx];
}
var heap_next = heap.length;
function dropObject(idx) {
  if (idx < 132) return;
  heap[idx] = heap_next;
  heap_next = idx;
}
function takeObject(idx) {
  const ret = getObject(idx);
  dropObject(idx);
  return ret;
}
var cachedTextDecoder = typeof TextDecoder !== "undefined" ? new TextDecoder("utf-8", { ignoreBOM: true, fatal: true }) : { decode: () => {
  throw Error("TextDecoder not available");
} };
if (typeof TextDecoder !== "undefined") {
  cachedTextDecoder.decode();
}
var cachedUint8Memory0 = null;
function getUint8Memory0() {
  if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
    cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8Memory0;
}
function getStringFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
function addHeapObject(obj) {
  if (heap_next === heap.length) heap.push(heap.length + 1);
  const idx = heap_next;
  heap_next = heap[idx];
  heap[idx] = obj;
  return idx;
}
var WASM_VECTOR_LEN = 0;
var cachedTextEncoder = typeof TextEncoder !== "undefined" ? new TextEncoder("utf-8") : { encode: () => {
  throw Error("TextEncoder not available");
} };
var encodeString = typeof cachedTextEncoder.encodeInto === "function" ? function(arg, view) {
  return cachedTextEncoder.encodeInto(arg, view);
} : function(arg, view) {
  const buf = cachedTextEncoder.encode(arg);
  view.set(buf);
  return {
    read: arg.length,
    written: buf.length
  };
};
function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === void 0) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr2 = malloc(buf.length, 1) >>> 0;
    getUint8Memory0().subarray(ptr2, ptr2 + buf.length).set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr2;
  }
  let len = arg.length;
  let ptr = malloc(len, 1) >>> 0;
  const mem = getUint8Memory0();
  let offset = 0;
  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 127) break;
    mem[ptr + offset] = code;
  }
  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
    const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
    const ret = encodeString(arg, view);
    offset += ret.written;
    ptr = realloc(ptr, len, offset, 1) >>> 0;
  }
  WASM_VECTOR_LEN = offset;
  return ptr;
}
function isLikeNone(x) {
  return x === void 0 || x === null;
}
var cachedInt32Memory0 = null;
function getInt32Memory0() {
  if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
    cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
  }
  return cachedInt32Memory0;
}
var cachedFloat64Memory0 = null;
function getFloat64Memory0() {
  if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
    cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
  }
  return cachedFloat64Memory0;
}
var cachedBigInt64Memory0 = null;
function getBigInt64Memory0() {
  if (cachedBigInt64Memory0 === null || cachedBigInt64Memory0.byteLength === 0) {
    cachedBigInt64Memory0 = new BigInt64Array(wasm.memory.buffer);
  }
  return cachedBigInt64Memory0;
}
function debugString(val) {
  const type = typeof val;
  if (type == "number" || type == "boolean" || val == null) {
    return `${val}`;
  }
  if (type == "string") {
    return `"${val}"`;
  }
  if (type == "symbol") {
    const description = val.description;
    if (description == null) {
      return "Symbol";
    } else {
      return `Symbol(${description})`;
    }
  }
  if (type == "function") {
    const name = val.name;
    if (typeof name == "string" && name.length > 0) {
      return `Function(${name})`;
    } else {
      return "Function";
    }
  }
  if (Array.isArray(val)) {
    const length = val.length;
    let debug = "[";
    if (length > 0) {
      debug += debugString(val[0]);
    }
    for (let i = 1; i < length; i++) {
      debug += ", " + debugString(val[i]);
    }
    debug += "]";
    return debug;
  }
  const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
  let className;
  if (builtInMatches.length > 1) {
    className = builtInMatches[1];
  } else {
    return toString.call(val);
  }
  if (className == "Object") {
    try {
      return "Object(" + JSON.stringify(val) + ")";
    } catch (_) {
      return "Object";
    }
  }
  if (val instanceof Error) {
    return `${val.name}: ${val.message}
${val.stack}`;
  }
  return className;
}
var CLOSURE_DTORS = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((state) => {
  wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
});
function makeMutClosure(arg0, arg1, dtor, f) {
  const state = { a: arg0, b: arg1, cnt: 1, dtor };
  const real = (...args) => {
    state.cnt++;
    const a = state.a;
    state.a = 0;
    try {
      return f(a, state.b, ...args);
    } finally {
      if (--state.cnt === 0) {
        wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);
        CLOSURE_DTORS.unregister(state);
      } else {
        state.a = a;
      }
    }
  };
  real.original = state;
  CLOSURE_DTORS.register(real, state, state);
  return real;
}
function __wbg_adapter_56(arg0, arg1, arg2) {
  wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h2ac93f1c2af0bde9(arg0, arg1, addHeapObject(arg2));
}
function __wbg_adapter_61(arg0, arg1, arg2) {
  const ret = wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9b3a888f37401eda(arg0, arg1, addHeapObject(arg2));
  return takeObject(ret);
}
function makeClosure(arg0, arg1, dtor, f) {
  const state = { a: arg0, b: arg1, cnt: 1, dtor };
  const real = (...args) => {
    state.cnt++;
    try {
      return f(state.a, state.b, ...args);
    } finally {
      if (--state.cnt === 0) {
        wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
        state.a = 0;
        CLOSURE_DTORS.unregister(state);
      }
    }
  };
  real.original = state;
  CLOSURE_DTORS.register(real, state, state);
  return real;
}
function __wbg_adapter_64(arg0, arg1, arg2, arg3, arg4) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    wasm._dyn_core__ops__function__Fn__A_B_C___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4814c8631e98bfb6(retptr, arg0, arg1, addHeapObject(arg2), addHeapObject(arg3), addHeapObject(arg4));
    var r0 = getInt32Memory0()[retptr / 4 + 0];
    var r1 = getInt32Memory0()[retptr / 4 + 1];
    var r2 = getInt32Memory0()[retptr / 4 + 2];
    if (r2) {
      throw takeObject(r1);
    }
    return takeObject(r0);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
  }
}
function __wbg_adapter_67(arg0, arg1, arg2) {
  wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h27f03e771f4393f9(arg0, arg1, addHeapObject(arg2));
}
function _assertClass(instance, klass) {
  if (!(instance instanceof klass)) {
    throw new Error(`expected instance of ${klass.name}`);
  }
  return instance.ptr;
}
var cachedUint32Memory0 = null;
function getUint32Memory0() {
  if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
    cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
  }
  return cachedUint32Memory0;
}
function getArrayJsValueFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  const mem = getUint32Memory0();
  const slice = mem.subarray(ptr / 4, ptr / 4 + len);
  const result = [];
  for (let i = 0; i < slice.length; i++) {
    result.push(takeObject(slice[i]));
  }
  return result;
}
var stack_pointer = 128;
function addBorrowedObject(obj) {
  if (stack_pointer == 1) throw new Error("out of js stack");
  heap[--stack_pointer] = obj;
  return stack_pointer;
}
function main_js() {
  wasm.main_js();
}
function is_debug_mode() {
  const ret = wasm.is_debug_mode();
  return ret !== 0;
}
function handleError(f, args) {
  try {
    return f.apply(this, args);
  } catch (e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
  }
}
function passArrayJsValueToWasm0(array, malloc) {
  const ptr = malloc(array.length * 4, 4) >>> 0;
  const mem = getUint32Memory0();
  for (let i = 0; i < array.length; i++) {
    mem[ptr / 4 + i] = addHeapObject(array[i]);
  }
  WASM_VECTOR_LEN = array.length;
  return ptr;
}
function __wbgtest_console_log(args) {
  try {
    wasm.__wbgtest_console_log(addBorrowedObject(args));
  } finally {
    heap[stack_pointer++] = void 0;
  }
}
function __wbgtest_console_debug(args) {
  try {
    wasm.__wbgtest_console_debug(addBorrowedObject(args));
  } finally {
    heap[stack_pointer++] = void 0;
  }
}
function __wbgtest_console_info(args) {
  try {
    wasm.__wbgtest_console_info(addBorrowedObject(args));
  } finally {
    heap[stack_pointer++] = void 0;
  }
}
function __wbgtest_console_warn(args) {
  try {
    wasm.__wbgtest_console_warn(addBorrowedObject(args));
  } finally {
    heap[stack_pointer++] = void 0;
  }
}
function __wbgtest_console_error(args) {
  try {
    wasm.__wbgtest_console_error(addBorrowedObject(args));
  } finally {
    heap[stack_pointer++] = void 0;
  }
}
function __wbg_adapter_292(arg0, arg1) {
  wasm.wasm_bindgen__convert__closures__invoke0_mut__h0f5b26648d09e4b0(arg0, arg1);
}
function __wbg_adapter_335(arg0, arg1, arg2, arg3, arg4) {
  wasm.wasm_bindgen__convert__closures__invoke3_mut__h447a9f4e2970c0cf(arg0, arg1, addHeapObject(arg2), arg3, addHeapObject(arg4));
}
function __wbg_adapter_390(arg0, arg1, arg2, arg3) {
  wasm.wasm_bindgen__convert__closures__invoke2_mut__h36f949ecffe8079d(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}
var Errors = Object.freeze({ Error: 0, "0": "Error", HookError: 1, "1": "HookError", QueryError: 2, "2": "QueryError", SerializationError: 3, "3": "SerializationError", ValidationError: 4, "4": "ValidationError", AuthenticationError: 5, "5": "AuthenticationError" });
var OpType = Object.freeze({
  /**
  * Create operation.
  */
  CREATE: 0,
  "0": "CREATE",
  /**
  * Update operation.
  */
  UPDATE: 1,
  "1": "UPDATE",
  /**
  * Delete operation.
  */
  DELETE: 2,
  "2": "DELETE",
  /**
  * Query Operation.
  */
  QUERY: 3,
  "3": "QUERY",
  /**
  * Count Operation.
  */
  COUNT: 4,
  "4": "COUNT"
});
var BasePluginFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_baseplugin_free(ptr >>> 0));
var BasePlugin = class {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    BasePluginFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_baseplugin_free(ptr);
  }
  /**
  * @param {string} name
  */
  constructor(name) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
      const len0 = WASM_VECTOR_LEN;
      wasm.baseplugin_new(retptr, ptr0, len0);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      this.__wbg_ptr = r0 >>> 0;
      return this;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @returns {any}
  */
  get name() {
    const ret = wasm.baseplugin_name(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * @returns {any}
  */
  get docCreateHook() {
    const ret = wasm.baseplugin_get_doc_create_hook(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * @returns {any}
  */
  get docRecoverHook() {
    const ret = wasm.baseplugin_get_doc_recover_hook(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * @param {any} hook
  */
  set docCreateHook(hook) {
    wasm.baseplugin_set_doc_create_hook(this.__wbg_ptr, addHeapObject(hook));
  }
  /**
  * @param {any} hook
  */
  set docRecoverHook(hook) {
    wasm.baseplugin_set_doc_recover_hook(this.__wbg_ptr, addHeapObject(hook));
  }
};
var BaseStorageFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_basestorage_free(ptr >>> 0));
var BaseStorage = class {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    BaseStorageFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_basestorage_free(ptr);
  }
  /**
  * Creates a new `BaseStorage` instance with the provided name and schema type.
  *
  * # Arguments
  *
  * * `name` - The name of the storage.
  * * `schema_type` - The schema type in `JsValue` format.
  *
  * # Returns
  *
  * * `Result<BaseStorage, JsValue>` - A result containing the new `BaseStorage` instance or an error.
  * @param {string} name
  * @param {object} schemas_js
  * @param {object | undefined} [options]
  */
  constructor(name, schemas_js, options) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
      const len0 = WASM_VECTOR_LEN;
      wasm.basestorage_new(retptr, ptr0, len0, addHeapObject(schemas_js), isLikeNone(options) ? 0 : addHeapObject(options));
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      this.__wbg_ptr = r0 >>> 0;
      return this;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @returns {any}
  */
  addIndexSchemas() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.basestorage_addIndexSchemas(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @param {string} name
  * @returns {any}
  */
  getOption(name) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
      const len0 = WASM_VECTOR_LEN;
      wasm.basestorage_getOption(retptr, this.__wbg_ptr, ptr0, len0);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @param {string} name
  * @returns {Schema}
  */
  getSchema(name) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
      const len0 = WASM_VECTOR_LEN;
      wasm.basestorage_getSchema(retptr, this.__wbg_ptr, ptr0, len0);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return Schema.__wrap(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @returns {CoreStorage}
  */
  get core() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.basestorage_core(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return CoreStorage.__wrap(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
};
var CollectionFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_collection_free(ptr >>> 0));
var Collection = class _Collection {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(_Collection.prototype);
    obj.__wbg_ptr = ptr;
    CollectionFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CollectionFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_collection_free(ptr);
  }
  /**
  * @returns {string}
  */
  get name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.collection_name(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      deferred1_0 = r0;
      deferred1_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
  * @returns {Schema}
  */
  get schema() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.collection_schema(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return Schema.__wrap(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * Finds and returns all documents in the collection.
  *
  * This function is asynchronous and returns a `JsValue` representing
  * the documents found in the collection.
  * @param {any} query_js
  * @param {any} options_js
  * @returns {Promise<any>}
  */
  find(query_js, options_js) {
    const ret = wasm.collection_find(this.__wbg_ptr, addHeapObject(query_js), addHeapObject(options_js));
    return takeObject(ret);
  }
  /**
  * @param {any} options
  * @returns {QueryOptions}
  */
  parse_query_options(options) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.collection_parse_query_options(retptr, this.__wbg_ptr, addHeapObject(options));
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return QueryOptions.__wrap(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * counts and returns all documents in the collection.
  *
  * This function is asynchronous and returns a `Schema` representing
  * the documents found in the collection.
  * @param {any} query_js
  * @param {any} options_js
  * @returns {Promise<any>}
  */
  count(query_js, options_js) {
    const ret = wasm.collection_count(this.__wbg_ptr, addHeapObject(query_js), addHeapObject(options_js));
    return takeObject(ret);
  }
  /**
  * Finds and returns a single document in the collection by its ID.
  *
  * This function is asynchronous.
  * @param {any} primary_key
  * @returns {Promise<any>}
  */
  findById(primary_key) {
    const ret = wasm.collection_findById(this.__wbg_ptr, addHeapObject(primary_key));
    return takeObject(ret);
  }
  /**
  * Updates a document in the collection with the given data.
  *
  * This function is asynchronous and returns a `Result` indicating success or failure.
  *
  * # Arguments
  *
  * * `document` - A `JsValue` representing the partial document to update.
  * @param {any} document
  * @returns {Promise<any>}
  */
  update(document2) {
    const ret = wasm.collection_update(this.__wbg_ptr, addHeapObject(document2));
    return takeObject(ret);
  }
  /**
  * Creates a new document in the collection.
  *
  * This function is asynchronous and returns a `Result` indicating success or failure.
  *
  * # Arguments
  *
  * * `document` - A `JsValue` representing the document to create.
  * @param {any} document
  * @returns {Promise<any>}
  */
  create(document2) {
    const ret = wasm.collection_create(this.__wbg_ptr, addHeapObject(document2));
    return takeObject(ret);
  }
  /**
  * Deletes a document from the collection by its ID.
  *
  * This function is asynchronous.
  * @param {any} primary_key
  * @returns {Promise<any>}
  */
  delete(primary_key) {
    const ret = wasm.collection_delete(this.__wbg_ptr, addHeapObject(primary_key));
    return takeObject(ret);
  }
};
var CoreStorageFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_corestorage_free(ptr >>> 0));
var CoreStorage = class _CoreStorage {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(_CoreStorage.prototype);
    obj.__wbg_ptr = ptr;
    CoreStorageFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    CoreStorageFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_corestorage_free(ptr);
  }
  /**
  */
  constructor() {
    const ret = wasm.corestorage_new();
    this.__wbg_ptr = ret >>> 0;
    return this;
  }
  /**
  * @param {any} value
  * @returns {string}
  */
  getPrimaryKeyTyped(value) {
    let deferred2_0;
    let deferred2_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.corestorage_getPrimaryKeyTyped(retptr, this.__wbg_ptr, addHeapObject(value));
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      var r3 = getInt32Memory0()[retptr / 4 + 3];
      var ptr1 = r0;
      var len1 = r1;
      if (r3) {
        ptr1 = 0;
        len1 = 0;
        throw takeObject(r2);
      }
      deferred2_0 = ptr1;
      deferred2_1 = len1;
      return getStringFromWasm0(ptr1, len1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
  }
  /**
  * @param {Schema} schema
  * @param {Operation} op
  * @returns {(string)[]}
  */
  getIndexes(schema, op) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      _assertClass(schema, Schema);
      _assertClass(op, Operation);
      wasm.corestorage_getIndexes(retptr, this.__wbg_ptr, schema.__wbg_ptr, op.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      var r3 = getInt32Memory0()[retptr / 4 + 3];
      if (r3) {
        throw takeObject(r2);
      }
      var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
      wasm.__wbindgen_free(r0, r1 * 4, 4);
      return v1;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @param {any} document
  * @param {Query} query
  * @returns {boolean}
  */
  matchesQuery(document2, query) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      _assertClass(query, Query);
      var ptr0 = query.__destroy_into_raw();
      wasm.corestorage_matchesQuery(retptr, this.__wbg_ptr, addBorrowedObject(document2), ptr0);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return r0 !== 0;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      heap[stack_pointer++] = void 0;
    }
  }
};
var DatabaseFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_database_free(ptr >>> 0));
var Database = class _Database {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(_Database.prototype);
    obj.__wbg_ptr = ptr;
    DatabaseFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    DatabaseFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_database_free(ptr);
  }
  /**
  * @returns {Promise<any>}
  */
  start() {
    const ret = wasm.database_start(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * @returns {Promise<any>}
  */
  close() {
    const ptr = this.__destroy_into_raw();
    const ret = wasm.database_close(ptr);
    return takeObject(ret);
  }
  /**
  * @returns {boolean}
  */
  get started() {
    const ret = wasm.database_started(this.__wbg_ptr);
    return ret !== 0;
  }
  /**
  * @param {string} password
  * @returns {Promise<boolean>}
  */
  authenticate(password) {
    const ptr0 = passStringToWasm0(password, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.database_authenticate(this.__wbg_ptr, ptr0, len0);
    return takeObject(ret);
  }
  /**
  * Retrieves the collections in the database.
  *
  * This function returns an `Object` containing the collections.
  *
  * # Returns
  *
  * * `Result<Object, JsValue>` - A result containing an `Object` with the collections or an error.
  * @returns {object}
  */
  get collections() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.database_collections(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @param {string} db_name
  * @param {object} schemas_js
  * @param {object} migrations_js
  * @param {Array<any>} plugins
  * @param {any} module
  * @param {string | undefined} [password]
  * @param {any | undefined} [storage]
  * @returns {Promise<Database>}
  */
  static create(db_name, schemas_js, migrations_js, plugins, module2, password, storage) {
    const ptr0 = passStringToWasm0(db_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    var ptr1 = isLikeNone(password) ? 0 : passStringToWasm0(password, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    const ret = wasm.database_create(ptr0, len0, addHeapObject(schemas_js), addHeapObject(migrations_js), addHeapObject(plugins), addHeapObject(module2), ptr1, len1, isLikeNone(storage) ? 0 : addHeapObject(storage));
    return takeObject(ret);
  }
};
var InMemoryFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_inmemory_free(ptr >>> 0));
var InMemory = class _InMemory {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(_InMemory.prototype);
    obj.__wbg_ptr = ptr;
    InMemoryFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    InMemoryFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_inmemory_free(ptr);
  }
  /**
  * @param {string} name
  * @param {object} schemas_js
  * @returns {Promise<InMemory>}
  */
  static create(name, schemas_js) {
    const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.inmemory_create(ptr0, len0, addHeapObject(schemas_js));
    return takeObject(ret);
  }
  /**
  * @param {Operation} op
  * @returns {Promise<any>}
  */
  write(op) {
    _assertClass(op, Operation);
    var ptr0 = op.__destroy_into_raw();
    const ret = wasm.inmemory_write(this.__wbg_ptr, ptr0);
    return takeObject(ret);
  }
  /**
  * @param {string} collection_name
  * @param {any} query_js
  * @param {QueryOptions} options
  * @returns {Promise<any>}
  */
  find(collection_name, query_js, options) {
    const ptr0 = passStringToWasm0(collection_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(options, QueryOptions);
    var ptr1 = options.__destroy_into_raw();
    const ret = wasm.inmemory_find(this.__wbg_ptr, ptr0, len0, addHeapObject(query_js), ptr1);
    return takeObject(ret);
  }
  /**
  * @param {string} collection_name
  * @param {any} primary_key
  * @returns {Promise<any>}
  */
  findDocumentById(collection_name, primary_key) {
    const ptr0 = passStringToWasm0(collection_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.inmemory_findDocumentById(this.__wbg_ptr, ptr0, len0, addHeapObject(primary_key));
    return takeObject(ret);
  }
  /**
  * @param {string} collection_name
  * @param {any} query_js
  * @param {QueryOptions} options
  * @returns {Promise<any>}
  */
  count(collection_name, query_js, options) {
    const ptr0 = passStringToWasm0(collection_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(options, QueryOptions);
    var ptr1 = options.__destroy_into_raw();
    const ret = wasm.inmemory_count(this.__wbg_ptr, ptr0, len0, addHeapObject(query_js), ptr1);
    return takeObject(ret);
  }
  /**
  * @returns {Promise<any>}
  */
  close() {
    const ret = wasm.inmemory_close(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * @returns {Promise<any>}
  */
  start() {
    const ret = wasm.inmemory_start(this.__wbg_ptr);
    return takeObject(ret);
  }
};
var IndexDBFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_indexdb_free(ptr >>> 0));
var IndexDB = class _IndexDB {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(_IndexDB.prototype);
    obj.__wbg_ptr = ptr;
    IndexDBFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    IndexDBFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_indexdb_free(ptr);
  }
  /**
  * Fetch documents by opening an IndexedDB cursor (on an index or store),
  * then apply inline filtering and offset/limit constraints.
  * @returns {(string)[]}
  */
  get_stores() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.indexdb_get_stores(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
      wasm.__wbindgen_free(r0, r1 * 4, 4);
      return v1;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @param {string} store_name
  * @returns {IDBObjectStore}
  */
  get_store(store_name) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      const ptr0 = passStringToWasm0(store_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
      const len0 = WASM_VECTOR_LEN;
      wasm.indexdb_get_store(retptr, this.__wbg_ptr, ptr0, len0);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @param {string} name
  * @param {object} schemas_js
  * @returns {Promise<IndexDB>}
  */
  static create(name, schemas_js) {
    const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.indexdb_create(ptr0, len0, addHeapObject(schemas_js));
    return takeObject(ret);
  }
  /**
  * @param {Operation} op
  * @returns {Promise<any>}
  */
  write(op) {
    _assertClass(op, Operation);
    var ptr0 = op.__destroy_into_raw();
    const ret = wasm.indexdb_write(this.__wbg_ptr, ptr0);
    return takeObject(ret);
  }
  /**
  * @param {string} collection_name
  * @param {any} query
  * @param {QueryOptions} options
  * @returns {Promise<any>}
  */
  find(collection_name, query, options) {
    const ptr0 = passStringToWasm0(collection_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(options, QueryOptions);
    var ptr1 = options.__destroy_into_raw();
    const ret = wasm.indexdb_find(this.__wbg_ptr, ptr0, len0, addHeapObject(query), ptr1);
    return takeObject(ret);
  }
  /**
  * @param {string} collection_name
  * @param {any} primary_key
  * @returns {Promise<any>}
  */
  findDocumentById(collection_name, primary_key) {
    const ptr0 = passStringToWasm0(collection_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.indexdb_findDocumentById(this.__wbg_ptr, ptr0, len0, addHeapObject(primary_key));
    return takeObject(ret);
  }
  /**
  * @param {string} collection_name
  * @param {any} query
  * @param {QueryOptions} options
  * @returns {Promise<any>}
  */
  count(collection_name, query, options) {
    const ptr0 = passStringToWasm0(collection_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(options, QueryOptions);
    var ptr1 = options.__destroy_into_raw();
    const ret = wasm.indexdb_count(this.__wbg_ptr, ptr0, len0, addHeapObject(query), ptr1);
    return takeObject(ret);
  }
  /**
  * @returns {Promise<any>}
  */
  close() {
    const ret = wasm.indexdb_close(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * @returns {Promise<any>}
  */
  start() {
    const ret = wasm.indexdb_start(this.__wbg_ptr);
    return takeObject(ret);
  }
};
var OperationFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_operation_free(ptr >>> 0));
var Operation = class _Operation {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(_Operation.prototype);
    obj.__wbg_ptr = ptr;
    OperationFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    OperationFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_operation_free(ptr);
  }
  /**
  * Retrieves the name of the collection.
  *
  * # Returns
  *
  * * `String` - The name of the collection.
  * @returns {string}
  */
  get collection() {
    let deferred1_0;
    let deferred1_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.operation_collection(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      deferred1_0 = r0;
      deferred1_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
  * Retrieves the type of operation.
  *
  * # Returns
  *
  * * `OpType` - The type of operation.
  * @returns {OpType}
  */
  get opType() {
    const ret = wasm.operation_opType(this.__wbg_ptr);
    return ret;
  }
  /**
  * Retrieves the data involved in the operation.
  *
  * # Returns
  *
  * * `JsValue` - The data involved in the operation.
  * @returns {any}
  */
  get data() {
    const ret = wasm.operation_data(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * Retrieves the primary key field of the current collection.
  *
  * # Returns
  *
  * * `Option<String>` - The primary key field of the current collection.
  * @returns {any}
  */
  get primaryKeyField() {
    const ret = wasm.operation_primaryKeyField(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * Retrieves the primary key value of the current data.
  *
  * # Returns
  *
  * * `Option<JsValue>` - The primary key value of the current data.
  * @returns {any}
  */
  get primaryKey() {
    const ret = wasm.operation_primaryKey(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * @returns {string}
  */
  get primaryKeyIndex() {
    let deferred2_0;
    let deferred2_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.operation_primaryKeyIndex(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      var r3 = getInt32Memory0()[retptr / 4 + 3];
      var ptr1 = r0;
      var len1 = r1;
      if (r3) {
        ptr1 = 0;
        len1 = 0;
        throw takeObject(r2);
      }
      deferred2_0 = ptr1;
      deferred2_1 = len1;
      return getStringFromWasm0(ptr1, len1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
  }
};
var PropertyFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_property_free(ptr >>> 0));
var Property = class {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    PropertyFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_property_free(ptr);
  }
  /**
  * Checks is the schema is valid.
  *
  * # Returns
  *
  * Throws exception if not valid
  * @returns {boolean}
  */
  is_valid() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.property_is_valid(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return r0 !== 0;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * Retrieves the type of the property.
  *
  * # Returns
  *
  * * `PropertyType` - The type of the property.
  * @returns {any}
  */
  get type() {
    const ret = wasm.property_type(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * Retrieves the items of the property.
  *
  * # Returns
  *
  * * `Result<JsValue, JsValue>` - A result containing the items as a `JsValue` or an error.
  * @returns {any}
  */
  get items() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.property_items(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * Retrieves the maximum number of items of the property.
  *
  * # Returns
  *
  * * `Result<JsValue, JsValue>` - A result containing the maximum number of items as a `JsValue` or an error.
  * @returns {any}
  */
  get maxItems() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.property_maxItems(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * Retrieves the minimum number of items of the property.
  *
  * # Returns
  *
  * * `Result<JsValue, JsValue>` - A result containing the minimum number of items as a `JsValue` or an error.
  * @returns {any}
  */
  get minItems() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.property_minItems(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * Retrieves the maximum length of the property.
  *
  * # Returns
  *
  * * `Result<JsValue, JsValue>` - A result containing the maximum length as a `JsValue` or an error.
  * @returns {any}
  */
  get maxLength() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.property_maxLength(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * Retrieves the minimum length of the property.
  *
  * # Returns
  *
  * * `Result<JsValue, JsValue>` - A result containing the minimum length as a `JsValue` or an error.
  * @returns {any}
  */
  get minLength() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.property_minLength(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * Retrieves the nested properties of the property.
  *
  * # Returns
  *
  * * `Result<JsValue, JsValue>` - A result containing the nested properties as a `JsValue` or an error.
  * @returns {any}
  */
  get properties() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.property_properties(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
};
var QueryFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_query_free(ptr >>> 0));
var Query = class {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    QueryFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_query_free(ptr);
  }
  /**
  * @param {any} query
  * @param {Schema} schema
  */
  constructor(query, schema) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      _assertClass(schema, Schema);
      var ptr0 = schema.__destroy_into_raw();
      wasm.query_new(retptr, addHeapObject(query), ptr0);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      this.__wbg_ptr = r0 >>> 0;
      return this;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @returns {any}
  */
  get query() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.query_query(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * Returns the schema properties (fields) that are used in the query.
  * The query may contain operators like $and, $or, $gt, $lt, etc.
  * @returns {(string)[]}
  */
  get_properties() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.query_get_properties(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      var r3 = getInt32Memory0()[retptr / 4 + 3];
      if (r3) {
        throw takeObject(r2);
      }
      var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
      wasm.__wbindgen_free(r0, r1 * 4, 4);
      return v1;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @returns {any}
  */
  parse() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.query_parse(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @param {any} query
  * @returns {any}
  */
  process_query(query) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.query_process_query(retptr, this.__wbg_ptr, addBorrowedObject(query));
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      heap[stack_pointer++] = void 0;
    }
  }
  /**
  * Returns the value of a property from the (normalized) query by its name.
  * This will scan the normalized query structure (including arrays, $and/$or blocks, etc.)
  * to find the first occurrence of the given property name and return its corresponding value.
  *
  * If not found, an error is returned.
  *
  * Example:
  *   let val = query.get("age")?;
  *   // val is a JsValue that might be a number, string, boolean, array, or object (e.g., { "$gt": 30 })
  * @param {string} property_name
  * @returns {any}
  */
  get(property_name) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      const ptr0 = passStringToWasm0(property_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
      const len0 = WASM_VECTOR_LEN;
      wasm.query_get(retptr, this.__wbg_ptr, ptr0, len0);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
};
var QueryOptionsFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_queryoptions_free(ptr >>> 0));
var QueryOptions = class _QueryOptions {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(_QueryOptions.prototype);
    obj.__wbg_ptr = ptr;
    QueryOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    QueryOptionsFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_queryoptions_free(ptr);
  }
  /**
  * @returns {any}
  */
  get limit() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.queryoptions_limit(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @returns {any}
  */
  get offset() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.queryoptions_offset(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
};
var RIDBErrorFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_ridberror_free(ptr >>> 0));
var RIDBError = class _RIDBError {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(_RIDBError.prototype);
    obj.__wbg_ptr = ptr;
    RIDBErrorFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  toJSON() {
    return {
      type: this.type,
      code: this.code,
      message: this.message
    };
  }
  toString() {
    return JSON.stringify(this);
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    RIDBErrorFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_ridberror_free(ptr);
  }
  /**
  * @param {string} err_type
  * @param {string} message
  * @param {number} code
  */
  constructor(err_type, message, code) {
    const ptr0 = passStringToWasm0(err_type, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(message, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.ridberror_new(ptr0, len0, ptr1, len1, code);
    this.__wbg_ptr = ret >>> 0;
    return this;
  }
  /**
  * @returns {string}
  */
  get type() {
    let deferred1_0;
    let deferred1_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.ridberror_type(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      deferred1_0 = r0;
      deferred1_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
  * @returns {any}
  */
  get code() {
    const ret = wasm.ridberror_code(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
  * @returns {string}
  */
  get message() {
    let deferred1_0;
    let deferred1_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.ridberror_message(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      deferred1_0 = r0;
      deferred1_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
  * @param {any} err
  * @returns {RIDBError}
  */
  static from(err) {
    const ret = wasm.ridberror_from(addHeapObject(err));
    return _RIDBError.__wrap(ret);
  }
  /**
  * @param {string} err
  * @param {number} code
  * @returns {RIDBError}
  */
  static error(err, code) {
    const ptr0 = passStringToWasm0(err, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.ridberror_error(ptr0, len0, code);
    return _RIDBError.__wrap(ret);
  }
  /**
  * @param {string} err
  * @param {number} code
  * @returns {RIDBError}
  */
  static query(err, code) {
    const ptr0 = passStringToWasm0(err, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.ridberror_query(ptr0, len0, code);
    return _RIDBError.__wrap(ret);
  }
  /**
  * @param {string} err
  * @param {number} code
  * @returns {RIDBError}
  */
  static authentication(err, code) {
    const ptr0 = passStringToWasm0(err, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.ridberror_authentication(ptr0, len0, code);
    return _RIDBError.__wrap(ret);
  }
  /**
  * @param {string} err
  * @param {number} code
  * @returns {RIDBError}
  */
  static serialisation(err, code) {
    const ptr0 = passStringToWasm0(err, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.ridberror_serialisation(ptr0, len0, code);
    return _RIDBError.__wrap(ret);
  }
  /**
  * @param {string} err
  * @param {number} code
  * @returns {RIDBError}
  */
  static validation(err, code) {
    const ptr0 = passStringToWasm0(err, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.ridberror_validation(ptr0, len0, code);
    return _RIDBError.__wrap(ret);
  }
  /**
  * @param {string} err
  * @param {number} code
  * @returns {RIDBError}
  */
  static hook(err, code) {
    const ptr0 = passStringToWasm0(err, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.ridberror_hook(ptr0, len0, code);
    return _RIDBError.__wrap(ret);
  }
};
var SchemaFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_schema_free(ptr >>> 0));
var Schema = class _Schema {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(_Schema.prototype);
    obj.__wbg_ptr = ptr;
    SchemaFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    SchemaFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_schema_free(ptr);
  }
  /**
  * @param {any} document
  */
  validate(document2) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.schema_validate(retptr, this.__wbg_ptr, addHeapObject(document2));
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      if (r1) {
        throw takeObject(r0);
      }
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @returns {boolean}
  */
  is_valid() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.schema_is_valid(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return r0 !== 0;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * Creates a new `Schema` instance from a given `JsValue`.
  *
  * # Arguments
  *
  * * `schema` - A `JsValue` representing the schema.
  *
  * # Returns
  *
  * * `Result<Schema, JsValue>` - A result containing the new `Schema` instance or an error.
  * @param {any} schema
  * @returns {Schema}
  */
  static create(schema) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.schema_create(retptr, addHeapObject(schema));
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return _Schema.__wrap(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * Retrieves the version of the schema.
  *
  * # Returns
  *
  * * `i32` - The version of the schema.
  * @returns {number}
  */
  get version() {
    const ret = wasm.schema_version(this.__wbg_ptr);
    return ret;
  }
  /**
  * Retrieves the primary key of the schema.
  *
  * # Returns
  *
  * * `String` - The primary key of the schema.
  * @returns {string}
  */
  get primaryKey() {
    let deferred1_0;
    let deferred1_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.schema_primaryKey(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      deferred1_0 = r0;
      deferred1_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
  * Retrieves the type of the schema.
  *
  * # Returns
  *
  * * `String` - The type of the schema.
  * @returns {string}
  */
  get type() {
    let deferred1_0;
    let deferred1_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.schema_type(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      deferred1_0 = r0;
      deferred1_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
  * Retrieves the indexes of the schema, if any.
  *
  * # Returns
  *
  * * `Option<Vec<String>>` - The indexes of the schema, if any.
  * @returns {(string)[] | undefined}
  */
  get indexes() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.schema_indexes(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      let v1;
      if (r0 !== 0) {
        v1 = getArrayJsValueFromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 4, 4);
      }
      return v1;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * @returns {(string)[] | undefined}
  */
  get encrypted() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.schema_encrypted(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      let v1;
      if (r0 !== 0) {
        v1 = getArrayJsValueFromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 4, 4);
      }
      return v1;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
  * Retrieves the properties of the schema.
  *
  * # Returns
  *
  * * `Result<JsValue, JsValue>` - A result containing the properties as a `JsValue` or an error.
  * @returns {any}
  */
  get properties() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.schema_properties(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeObject(r1);
      }
      return takeObject(r0);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
};
var WasmBindgenTestContextFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_wasmbindgentestcontext_free(ptr >>> 0));
var WasmBindgenTestContext = class {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    WasmBindgenTestContextFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_wasmbindgentestcontext_free(ptr);
  }
  /**
  * Creates a new context ready to run tests.
  *
  * A `Context` is the main structure through which test execution is
  * coordinated, and this will collect output and results for all executed
  * tests.
  */
  constructor() {
    const ret = wasm.wasmbindgentestcontext_new();
    this.__wbg_ptr = ret >>> 0;
    return this;
  }
  /**
  * Inform this context about runtime arguments passed to the test
  * harness.
  * @param {any[]} args
  */
  args(args) {
    const ptr0 = passArrayJsValueToWasm0(args, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.wasmbindgentestcontext_args(this.__wbg_ptr, ptr0, len0);
  }
  /**
  * Executes a list of tests, returning a promise representing their
  * eventual completion.
  *
  * This is the main entry point for executing tests. All the tests passed
  * in are the JS `Function` object that was plucked off the
  * `WebAssembly.Instance` exports list.
  *
  * The promise returned resolves to either `true` if all tests passed or
  * `false` if at least one test failed.
  * @param {any[]} tests
  * @returns {Promise<any>}
  */
  run(tests) {
    const ptr0 = passArrayJsValueToWasm0(tests, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasmbindgentestcontext_run(this.__wbg_ptr, ptr0, len0);
    return takeObject(ret);
  }
};
async function __wbg_load(module2, imports) {
  if (typeof Response === "function" && module2 instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === "function") {
      try {
        return await WebAssembly.instantiateStreaming(module2, imports);
      } catch (e) {
        if (module2.headers.get("Content-Type") != "application/wasm") {
          console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
        } else {
          throw e;
        }
      }
    }
    const bytes = await module2.arrayBuffer();
    return await WebAssembly.instantiate(bytes, imports);
  } else {
    const instance = await WebAssembly.instantiate(module2, imports);
    if (instance instanceof WebAssembly.Instance) {
      return { instance, module: module2 };
    } else {
      return instance;
    }
  }
}
function __wbg_get_imports() {
  const imports = {};
  imports.wbg = {};
  imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
  };
  imports.wbg.__wbindgen_is_undefined = function(arg0) {
    const ret = getObject(arg0) === void 0;
    return ret;
  };
  imports.wbg.__wbindgen_is_null = function(arg0) {
    const ret = getObject(arg0) === null;
    return ret;
  };
  imports.wbg.__wbg_ridberror_new = function(arg0) {
    const ret = RIDBError.__wrap(arg0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_number_new = function(arg0) {
    const ret = arg0;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_close_6384ed3c27ef25c1 = function() {
    return handleError(function(arg0) {
      const ret = getObject(arg0).close();
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_find_567c5c9f064fe3d2 = function() {
    return handleError(function(arg0, arg1, arg2, arg3, arg4) {
      const ret = getObject(arg0).find(getStringFromWasm0(arg1, arg2), takeObject(arg3), QueryOptions.__wrap(arg4));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_count_19db4c3174d573d5 = function() {
    return handleError(function(arg0, arg1, arg2, arg3, arg4) {
      const ret = getObject(arg0).count(getStringFromWasm0(arg1, arg2), takeObject(arg3), QueryOptions.__wrap(arg4));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_start_76c138c3b73ae6f8 = function() {
    return handleError(function(arg0) {
      const ret = getObject(arg0).start();
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof obj === "string" ? obj : void 0;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
  };
  imports.wbg.__wbg_apply_9f557eba1534d597 = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = getObject(arg1).apply(takeObject(arg2));
      const ptr1 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_malloc);
      const len1 = WASM_VECTOR_LEN;
      getInt32Memory0()[arg0 / 4 + 1] = len1;
      getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    }, arguments);
  };
  imports.wbg.__wbg_inmemory_new = function(arg0) {
    const ret = InMemory.__wrap(arg0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof obj === "number" ? obj : void 0;
    getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
    getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
  };
  imports.wbg.__wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
      obj.a = 0;
      return true;
    }
    const ret = false;
    return ret;
  };
  imports.wbg.__wbg_findDocumentById_2edf7350e5f12657 = function() {
    return handleError(function(arg0, arg1, arg2, arg3) {
      const ret = getObject(arg0).findDocumentById(getStringFromWasm0(arg1, arg2), takeObject(arg3));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_write_1159c67c07f62020 = function() {
    return handleError(function(arg0, arg1) {
      const ret = getObject(arg0).write(Operation.__wrap(arg1));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_indexdb_new = function(arg0) {
    const ret = IndexDB.__wrap(arg0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_database_new = function(arg0) {
    const ret = Database.__wrap(arg0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_collection_new = function(arg0) {
    const ret = Collection.__wrap(arg0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_boolean_get = function(arg0) {
    const v = getObject(arg0);
    const ret = typeof v === "boolean" ? v ? 1 : 0 : 2;
    return ret;
  };
  imports.wbg.__wbindgen_is_function = function(arg0) {
    const ret = typeof getObject(arg0) === "function";
    return ret;
  };
  imports.wbg.__wbindgen_is_string = function(arg0) {
    const ret = typeof getObject(arg0) === "string";
    return ret;
  };
  imports.wbg.__wbindgen_is_bigint = function(arg0) {
    const ret = typeof getObject(arg0) === "bigint";
    return ret;
  };
  imports.wbg.__wbindgen_is_array = function(arg0) {
    const ret = Array.isArray(getObject(arg0));
    return ret;
  };
  imports.wbg.__wbindgen_is_object = function(arg0) {
    const val = getObject(arg0);
    const ret = typeof val === "object" && val !== null;
    return ret;
  };
  imports.wbg.__wbindgen_is_falsy = function(arg0) {
    const ret = !getObject(arg0);
    return ret;
  };
  imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
    const ret = getObject(arg0) === getObject(arg1);
    return ret;
  };
  imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
    const ret = new Error(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_in = function(arg0, arg1) {
    const ret = getObject(arg0) in getObject(arg1);
    return ret;
  };
  imports.wbg.__wbindgen_bigint_from_i64 = function(arg0) {
    const ret = arg0;
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_bigint_from_u64 = function(arg0) {
    const ret = BigInt.asUintN(64, arg0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_crypto_1d1f22824a6a080c = function(arg0) {
    const ret = getObject(arg0).crypto;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_process_4a72847cc503995b = function(arg0) {
    const ret = getObject(arg0).process;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_versions_f686565e586dd935 = function(arg0) {
    const ret = getObject(arg0).versions;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_node_104a2ff8d6ea03a2 = function(arg0) {
    const ret = getObject(arg0).node;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_require_cca90b1a94a0255b = function() {
    return handleError(function() {
      const ret = module.require;
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_msCrypto_eb05e62b530a1508 = function(arg0) {
    const ret = getObject(arg0).msCrypto;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_randomFillSync_5c9c955aa56b6049 = function() {
    return handleError(function(arg0, arg1) {
      getObject(arg0).randomFillSync(takeObject(arg1));
    }, arguments);
  };
  imports.wbg.__wbg_getRandomValues_3aa56aa6edec874c = function() {
    return handleError(function(arg0, arg1) {
      getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments);
  };
  imports.wbg.__wbg_instanceof_Window_f401953a2cf86220 = function(arg0) {
    let result;
    try {
      result = getObject(arg0) instanceof Window;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_indexedDB_7c51d9056667f4e0 = function() {
    return handleError(function(arg0) {
      const ret = getObject(arg0).indexedDB;
      return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_instanceof_WorkerGlobalScope_46b577f151fad960 = function(arg0) {
    let result;
    try {
      result = getObject(arg0) instanceof WorkerGlobalScope;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_indexedDB_d312f95759a15fdc = function() {
    return handleError(function(arg0) {
      const ret = getObject(arg0).indexedDB;
      return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_openCursor_425aba9cbe1d4d39 = function() {
    return handleError(function(arg0) {
      const ret = getObject(arg0).openCursor();
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_openCursor_e3042770817a8d57 = function() {
    return handleError(function(arg0, arg1) {
      const ret = getObject(arg0).openCursor(getObject(arg1));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_createIndex_b8da1f5571f644be = function() {
    return handleError(function(arg0, arg1, arg2, arg3, arg4, arg5) {
      const ret = getObject(arg0).createIndex(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4), getObject(arg5));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_delete_f60bba7d0ae59a4f = function() {
    return handleError(function(arg0, arg1) {
      const ret = getObject(arg0).delete(getObject(arg1));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_get_5361b64cac0d0826 = function() {
    return handleError(function(arg0, arg1) {
      const ret = getObject(arg0).get(getObject(arg1));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_index_383b6812c1508030 = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = getObject(arg0).index(getStringFromWasm0(arg1, arg2));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_openCursor_30d58ae27a327629 = function() {
    return handleError(function(arg0) {
      const ret = getObject(arg0).openCursor();
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_openCursor_611b1e488c393dd8 = function() {
    return handleError(function(arg0, arg1) {
      const ret = getObject(arg0).openCursor(getObject(arg1));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_put_22792e17580ca18b = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = getObject(arg0).put(getObject(arg1), getObject(arg2));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_target_2fc177e386c8b7b0 = function(arg0) {
    const ret = getObject(arg0).target;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
  };
  imports.wbg.__wbg_instanceof_IdbDatabase_db671cf2454a9542 = function(arg0) {
    let result;
    try {
      result = getObject(arg0) instanceof IDBDatabase;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_objectStoreNames_588b5924274239fd = function(arg0) {
    const ret = getObject(arg0).objectStoreNames;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_close_6bfe4ca2fe67cb67 = function(arg0) {
    getObject(arg0).close();
  };
  imports.wbg.__wbg_createObjectStore_882f2f6b1b1ef040 = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = getObject(arg0).createObjectStore(getStringFromWasm0(arg1, arg2));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_transaction_c32bb10c9c692f4b = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = getObject(arg0).transaction(getStringFromWasm0(arg1, arg2));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_transaction_1e282a79e9bb7387 = function() {
    return handleError(function(arg0, arg1, arg2, arg3) {
      const ret = getObject(arg0).transaction(getStringFromWasm0(arg1, arg2), takeObject(arg3));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_length_9ae5daf9a690cba9 = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
  };
  imports.wbg.__wbg_contains_c65b44400b549286 = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).contains(getStringFromWasm0(arg1, arg2));
    return ret;
  };
  imports.wbg.__wbg_get_910bbb94abdcf488 = function(arg0, arg1, arg2) {
    const ret = getObject(arg1)[arg2 >>> 0];
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
  };
  imports.wbg.__wbg_continue_f1c3e0815924de62 = function() {
    return handleError(function(arg0) {
      getObject(arg0).continue();
    }, arguments);
  };
  imports.wbg.__wbg_instanceof_IdbCursorWithValue_abeb44d13d947bc2 = function(arg0) {
    let result;
    try {
      result = getObject(arg0) instanceof IDBCursorWithValue;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_value_86d3334f5075b232 = function() {
    return handleError(function(arg0) {
      const ret = getObject(arg0).value;
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_open_f0d7259fd7e689ce = function() {
    return handleError(function(arg0, arg1, arg2, arg3) {
      const ret = getObject(arg0).open(getStringFromWasm0(arg1, arg2), arg3 >>> 0);
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_instanceof_IdbOpenDbRequest_3f4a166bc0340578 = function(arg0) {
    let result;
    try {
      result = getObject(arg0) instanceof IDBOpenDBRequest;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_setonupgradeneeded_ad7645373c7d5e1b = function(arg0, arg1) {
    getObject(arg0).onupgradeneeded = getObject(arg1);
  };
  imports.wbg.__wbg_instanceof_IdbRequest_93249da04f5370b6 = function(arg0) {
    let result;
    try {
      result = getObject(arg0) instanceof IDBRequest;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_result_6cedf5f78600a79c = function() {
    return handleError(function(arg0) {
      const ret = getObject(arg0).result;
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_error_685b20024dc2d6ca = function() {
    return handleError(function(arg0) {
      const ret = getObject(arg0).error;
      return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_setonsuccess_632ce0a1460455c2 = function(arg0, arg1) {
    getObject(arg0).onsuccess = getObject(arg1);
  };
  imports.wbg.__wbg_setonerror_8479b33e7568a904 = function(arg0, arg1) {
    getObject(arg0).onerror = getObject(arg1);
  };
  imports.wbg.__wbg_setoncomplete_d8e4236665cbf1e2 = function(arg0, arg1) {
    getObject(arg0).oncomplete = getObject(arg1);
  };
  imports.wbg.__wbg_setonerror_da071ec94e148397 = function(arg0, arg1) {
    getObject(arg0).onerror = getObject(arg1);
  };
  imports.wbg.__wbg_objectStore_da468793bd9df17b = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = getObject(arg0).objectStore(getStringFromWasm0(arg1, arg2));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_only_cacf767244bdc280 = function() {
    return handleError(function(arg0) {
      const ret = IDBKeyRange.only(getObject(arg0));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
    const ret = getObject(arg0) == getObject(arg1);
    return ret;
  };
  imports.wbg.__wbindgen_as_number = function(arg0) {
    const ret = +getObject(arg0);
    return ret;
  };
  imports.wbg.__wbg_String_389b54bd9d25375f = function(arg0, arg1) {
    const ret = String(getObject(arg1));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
  };
  imports.wbg.__wbg_getwithrefkey_4a92a5eca60879b9 = function(arg0, arg1) {
    const ret = getObject(arg0)[getObject(arg1)];
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_set_9182712abebf82ef = function(arg0, arg1, arg2) {
    getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
  };
  imports.wbg.__wbg_log_28eee4e6432efd24 = function(arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
  };
  imports.wbg.__wbg_String_55b8bdc4bc243677 = function(arg0, arg1) {
    const ret = String(getObject(arg1));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
  };
  imports.wbg.__wbg_getElementById_8458f2a6c28467dc = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_settextcontent_fc3ff485b96fcb1d = function(arg0, arg1, arg2) {
    getObject(arg0).textContent = getStringFromWasm0(arg1, arg2);
  };
  imports.wbg.__wbg_wbgtestinvoke_8c20f4132af2bded = function() {
    return handleError(function(arg0, arg1) {
      try {
        var state0 = { a: arg0, b: arg1 };
        var cb0 = () => {
          const a = state0.a;
          state0.a = 0;
          try {
            return __wbg_adapter_292(a, state0.b);
          } finally {
            state0.a = a;
          }
        };
        __wbg_test_invoke(cb0);
      } finally {
        state0.a = state0.b = 0;
      }
    }, arguments);
  };
  imports.wbg.__wbg_wbgtestoutputwriteln_4db3bd64914ec955 = function(arg0) {
    __wbg_test_output_writeln(takeObject(arg0));
  };
  imports.wbg.__wbg_stack_436273c21658169b = function(arg0) {
    const ret = getObject(arg0).stack;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_static_accessor_document_d4b6ae7f5578480f = function() {
    const ret = document;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_stack_17c77e9f5bfe6714 = function(arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
  };
  imports.wbg.__wbg_self_55106357ec10ecd4 = function(arg0) {
    const ret = getObject(arg0).self;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
  };
  imports.wbg.__wbg_constructor_fd0d22d60b7dfd72 = function(arg0) {
    const ret = getObject(arg0).constructor;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_name_7f439d24ff7ba1d3 = function(arg0, arg1) {
    const ret = getObject(arg1).name;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
  };
  imports.wbg.__wbg_textcontent_67e4e811cbdf00fc = function(arg0, arg1) {
    const ret = getObject(arg1).textContent;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
  };
  imports.wbg.__wbg_stack_44743fb7d71926a0 = function(arg0) {
    const ret = getObject(arg0).stack;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
    const ret = new Error();
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
  };
  imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
      deferred0_0 = arg0;
      deferred0_1 = arg1;
      console.error(getStringFromWasm0(arg0, arg1));
    } finally {
      wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
  };
  imports.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6 = function(arg0) {
    const ret = getObject(arg0).queueMicrotask;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_queueMicrotask_481971b0d87f3dd4 = function(arg0) {
    queueMicrotask(getObject(arg0));
  };
  imports.wbg.__wbg_get_bd8e338fbd5f5cc8 = function(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_length_cd7af8117672b8b8 = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
  };
  imports.wbg.__wbg_new_16b304a2cfa7ff4a = function() {
    const ret = new Array();
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_newnoargs_e258087cd0daa0ea = function(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_new_d9bc3a0147634640 = function() {
    const ret = /* @__PURE__ */ new Map();
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_next_40fc327bfc8770e6 = function(arg0) {
    const ret = getObject(arg0).next;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_next_196c84450b364254 = function() {
    return handleError(function(arg0) {
      const ret = getObject(arg0).next();
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_done_298b57d23c0fc80c = function(arg0) {
    const ret = getObject(arg0).done;
    return ret;
  };
  imports.wbg.__wbg_value_d93c65011f51a456 = function(arg0) {
    const ret = getObject(arg0).value;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_iterator_2cee6dadfd956dfa = function() {
    const ret = Symbol.iterator;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_get_e3c254076557e348 = function() {
    return handleError(function(arg0, arg1) {
      const ret = Reflect.get(getObject(arg0), getObject(arg1));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_call_27c0f87801dedf93 = function() {
    return handleError(function(arg0, arg1) {
      const ret = getObject(arg0).call(getObject(arg1));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_new_72fb9a18b5ae2624 = function() {
    const ret = new Object();
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_self_ce0dbfc45cf2f5be = function() {
    return handleError(function() {
      const ret = self.self;
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_window_c6fb939a7f436783 = function() {
    return handleError(function() {
      const ret = window.window;
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_globalThis_d1e6af4856ba331b = function() {
    return handleError(function() {
      const ret = globalThis.globalThis;
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_global_207b558942527489 = function() {
    return handleError(function() {
      const ret = global.global;
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_set_d4638f722068f043 = function(arg0, arg1, arg2) {
    getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
  };
  imports.wbg.__wbg_from_89e3fc3ba5e6fb48 = function(arg0) {
    const ret = Array.from(getObject(arg0));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_forEach_2be8de7347d63332 = function(arg0, arg1, arg2) {
    try {
      var state0 = { a: arg1, b: arg2 };
      var cb0 = (arg02, arg12, arg22) => {
        const a = state0.a;
        state0.a = 0;
        try {
          return __wbg_adapter_335(a, state0.b, arg02, arg12, arg22);
        } finally {
          state0.a = a;
        }
      };
      getObject(arg0).forEach(cb0);
    } finally {
      state0.a = state0.b = 0;
    }
  };
  imports.wbg.__wbg_isArray_2ab64d95e09ea0ae = function(arg0) {
    const ret = Array.isArray(getObject(arg0));
    return ret;
  };
  imports.wbg.__wbg_of_4a2b313a453ec059 = function(arg0) {
    const ret = Array.of(getObject(arg0));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_push_a5b05aedc7234f9f = function(arg0, arg1) {
    const ret = getObject(arg0).push(getObject(arg1));
    return ret;
  };
  imports.wbg.__wbg_instanceof_ArrayBuffer_836825be07d4c9d2 = function(arg0) {
    let result;
    try {
      result = getObject(arg0) instanceof ArrayBuffer;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_message_5bf28016c2b49cfb = function(arg0) {
    const ret = getObject(arg0).message;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_name_e7429f0dda6079e2 = function(arg0) {
    const ret = getObject(arg0).name;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_call_b3ca7c6051f9bec1 = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_call_938992c832f74314 = function() {
    return handleError(function(arg0, arg1, arg2, arg3, arg4) {
      const ret = getObject(arg0).call(getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_set_8417257aaedc936b = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_isSafeInteger_f7b04ef02296c4d2 = function(arg0) {
    const ret = Number.isSafeInteger(getObject(arg0));
    return ret;
  };
  imports.wbg.__wbg_getTime_2bc4375165f02d15 = function(arg0) {
    const ret = getObject(arg0).getTime();
    return ret;
  };
  imports.wbg.__wbg_new0_7d84e5b2cd9fdc73 = function() {
    const ret = /* @__PURE__ */ new Date();
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_assign_496d2d14fecafbcf = function(arg0, arg1) {
    const ret = Object.assign(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_entries_95cc2c823b285a09 = function(arg0) {
    const ret = Object.entries(getObject(arg0));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_is_010fdc0f4ab96916 = function(arg0, arg1) {
    const ret = Object.is(getObject(arg0), getObject(arg1));
    return ret;
  };
  imports.wbg.__wbg_keys_91e412b4b222659f = function(arg0) {
    const ret = Object.keys(getObject(arg0));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_new_81740750da40724f = function(arg0, arg1) {
    try {
      var state0 = { a: arg0, b: arg1 };
      var cb0 = (arg02, arg12) => {
        const a = state0.a;
        state0.a = 0;
        try {
          return __wbg_adapter_390(a, state0.b, arg02, arg12);
        } finally {
          state0.a = a;
        }
      };
      const ret = new Promise(cb0);
      return addHeapObject(ret);
    } finally {
      state0.a = state0.b = 0;
    }
  };
  imports.wbg.__wbg_resolve_b0083a7967828ec8 = function(arg0) {
    const ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_then_0c86a60e8fcfe9f6 = function(arg0, arg1) {
    const ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_then_a73caa9a87991566 = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_buffer_12d079cc21e14bdb = function(arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb = function(arg0, arg1, arg2) {
    const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_new_63b92bc8671ed464 = function(arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_set_a47bac70306a19a7 = function(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
  };
  imports.wbg.__wbg_length_c20a40f15020d68a = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
  };
  imports.wbg.__wbg_instanceof_Uint8Array_2b3bbecd033d19f6 = function(arg0) {
    let result;
    try {
      result = getObject(arg0) instanceof Uint8Array;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_newwithlength_e9b4878cebadb3d3 = function(arg0) {
    const ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_subarray_a1f73cd4b5b42fe1 = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_apply_0a5aa603881e6d79 = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = Reflect.apply(getObject(arg0), getObject(arg1), getObject(arg2));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_deleteProperty_13e721a56f19e842 = function() {
    return handleError(function(arg0, arg1) {
      const ret = Reflect.deleteProperty(getObject(arg0), getObject(arg1));
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_has_0af94d20077affa2 = function() {
    return handleError(function(arg0, arg1) {
      const ret = Reflect.has(getObject(arg0), getObject(arg1));
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_set_1f9b04f170055d33 = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_parse_66d1801634e099ac = function() {
    return handleError(function(arg0, arg1) {
      const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_stringify_8887fe74e1c50d81 = function() {
    return handleError(function(arg0) {
      const ret = JSON.stringify(getObject(arg0));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbindgen_bigint_get_as_i64 = function(arg0, arg1) {
    const v = getObject(arg1);
    const ret = typeof v === "bigint" ? v : void 0;
    getBigInt64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? BigInt(0) : ret;
    getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
  };
  imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
  };
  imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
  };
  imports.wbg.__wbindgen_memory = function() {
    const ret = wasm.memory;
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_closure_wrapper419 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 174, __wbg_adapter_56);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_closure_wrapper421 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 174, __wbg_adapter_56);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_closure_wrapper423 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 174, __wbg_adapter_61);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_closure_wrapper425 = function(arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 174, __wbg_adapter_64);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_closure_wrapper1668 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 472, __wbg_adapter_67);
    return addHeapObject(ret);
  };
  return imports;
}
function __wbg_init_memory(imports, maybe_memory) {
}
function __wbg_finalize_init(instance, module2) {
  wasm = instance.exports;
  __wbg_init.__wbindgen_wasm_module = module2;
  cachedBigInt64Memory0 = null;
  cachedFloat64Memory0 = null;
  cachedInt32Memory0 = null;
  cachedUint32Memory0 = null;
  cachedUint8Memory0 = null;
  wasm.__wbindgen_start();
  return wasm;
}
function initSync(module2) {
  if (wasm !== void 0) return wasm;
  const imports = __wbg_get_imports();
  __wbg_init_memory(imports);
  if (!(module2 instanceof WebAssembly.Module)) {
    module2 = new WebAssembly.Module(module2);
  }
  const instance = new WebAssembly.Instance(module2, imports);
  return __wbg_finalize_init(instance, module2);
}
async function __wbg_init(input) {
  if (wasm !== void 0) return wasm;
  const imports = __wbg_get_imports();
  if (typeof input === "string" || typeof Request === "function" && input instanceof Request || typeof URL === "function" && input instanceof URL) {
    input = fetch(input);
  }
  __wbg_init_memory(imports);
  const { instance, module: module2 } = await __wbg_load(await input, imports);
  return __wbg_finalize_init(instance, module2);
}
var ridb_core_default = __wbg_init;
export {
  BasePlugin,
  BaseStorage,
  Collection,
  CoreStorage,
  Database,
  Errors,
  InMemory,
  IndexDB,
  OpType,
  Operation,
  Property,
  Query,
  QueryOptions,
  RIDBError,
  Schema,
  WasmBindgenTestContext,
  __wbgtest_console_debug,
  __wbgtest_console_error,
  __wbgtest_console_info,
  __wbgtest_console_log,
  __wbgtest_console_warn,
  ridb_core_default as default,
  initSync,
  is_debug_mode,
  main_js
};
