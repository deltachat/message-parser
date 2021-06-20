
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}
/**
* parses to json AST
* @param {string} s
* @returns {any}
*/
export function parse(s) {
    var ptr0 = passStringToWasm0(s, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    var ret = wasm.parse(ptr0, len0);
    return takeObject(ret);
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}
/**
* Email address struct.
*
* # Examples
* ```
* use email_address_parser::EmailAddress;
*
* assert!(EmailAddress::parse("foo@-bar.com", None).is_none());
* let email = EmailAddress::parse("foo@bar.com", None);
* assert!(email.is_some());
* let email = email.unwrap();
* assert_eq!(email.get_local_part(), "foo");
* assert_eq!(email.get_domain(), "bar.com");
* assert_eq!(format!("{}", email), "foo@bar.com");
* ```
*/
export class EmailAddress {

    static __wrap(ptr) {
        const obj = Object.create(EmailAddress.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_emailaddress_free(ptr);
    }
    /**
    * This is a WASM wrapper over EmailAddress::new that panics.
    * If you are using this lib from Rust then consider using EmailAddress::new.
    *
    * # Examples
    * ```
    * use email_address_parser::EmailAddress;
    *
    * let email = EmailAddress::_new("foo", "bar.com", None);
    * ```
    *
    * # Panics
    *
    * This method panics if the local part or domain is invalid.
    *
    * ```rust,should_panic
    * use email_address_parser::EmailAddress;
    *
    * EmailAddress::_new("foo", "-bar.com", None);
    * ```
    * @param {string} local_part
    * @param {string} domain
    * @param {ParsingOptions | undefined} options
    */
    constructor(local_part, domain, options) {
        var ptr0 = passStringToWasm0(local_part, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passStringToWasm0(domain, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        let ptr2 = 0;
        if (!isLikeNone(options)) {
            _assertClass(options, ParsingOptions);
            ptr2 = options.ptr;
            options.ptr = 0;
        }
        var ret = wasm.emailaddress__new(ptr0, len0, ptr1, len1, ptr2);
        return EmailAddress.__wrap(ret);
    }
    /**
    * Parses a given string as an email address.
    *
    * Accessible from WASM.
    *
    * Returns `Some(EmailAddress)` if the parsing is successful, else `None`.
    * # Examples
    * ```
    * use email_address_parser::*;
    *
    * // strict parsing
    * let email = EmailAddress::parse("foo@bar.com", None);
    * assert!(email.is_some());
    * let email = email.unwrap();
    * assert_eq!(email.get_local_part(), "foo");
    * assert_eq!(email.get_domain(), "bar.com");
    *
    * // non-strict parsing
    * let email = EmailAddress::parse("\u{0d}\u{0a} \u{0d}\u{0a} test@iana.org", Some(ParsingOptions::new(true)));
    * assert!(email.is_some());
    *
    * // parsing invalid address
    * let email = EmailAddress::parse("test@-iana.org", Some(ParsingOptions::new(true)));
    * assert!(email.is_none());
    * let email = EmailAddress::parse("test@-iana.org", Some(ParsingOptions::new(true)));
    * assert!(email.is_none());
    * let email = EmailAddress::parse("test", Some(ParsingOptions::new(true)));
    * assert!(email.is_none());
    * let email = EmailAddress::parse("test", Some(ParsingOptions::new(true)));
    * assert!(email.is_none());
    * ```
    * @param {string} input
    * @param {ParsingOptions | undefined} options
    * @returns {EmailAddress | undefined}
    */
    static parse(input, options) {
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        let ptr1 = 0;
        if (!isLikeNone(options)) {
            _assertClass(options, ParsingOptions);
            ptr1 = options.ptr;
            options.ptr = 0;
        }
        var ret = wasm.emailaddress_parse(ptr0, len0, ptr1);
        return ret === 0 ? undefined : EmailAddress.__wrap(ret);
    }
    /**
    * Validates if the given `input` string is an email address or not.
    *
    * Returns `true` if the `input` is valid, `false` otherwise.
    * Unlike the `parse` method, it does not instantiate an `EmailAddress`.
    * # Examples
    * ```
    * use email_address_parser::*;
    *
    * // strict validation
    * assert!(EmailAddress::is_valid("foo@bar.com", None));
    *
    * // non-strict validation
    * assert!(EmailAddress::is_valid("\u{0d}\u{0a} \u{0d}\u{0a} test@iana.org", Some(ParsingOptions::new(true))));
    *
    * // invalid address
    * assert!(!EmailAddress::is_valid("test@-iana.org", Some(ParsingOptions::new(true))));
    * assert!(!EmailAddress::is_valid("test@-iana.org", Some(ParsingOptions::new(true))));
    * assert!(!EmailAddress::is_valid("test", Some(ParsingOptions::new(true))));
    * assert!(!EmailAddress::is_valid("test", Some(ParsingOptions::new(true))));
    * ```
    * @param {string} input
    * @param {ParsingOptions | undefined} options
    * @returns {boolean}
    */
    static isValid(input, options) {
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        let ptr1 = 0;
        if (!isLikeNone(options)) {
            _assertClass(options, ParsingOptions);
            ptr1 = options.ptr;
            options.ptr = 0;
        }
        var ret = wasm.emailaddress_isValid(ptr0, len0, ptr1);
        return ret !== 0;
    }
    /**
    * Returns the local part of the email address.
    *
    * Note that if you are using this library from rust, then consider using the `get_local_part` method instead.
    * This returns a cloned copy of the local part string, instead of a borrowed `&str`, and exists purely for WASM interoperability.
    *
    * # Examples
    * ```
    * use email_address_parser::EmailAddress;
    *
    * let email = EmailAddress::new("foo", "bar.com", None).unwrap();
    * assert_eq!(email.localPart(), "foo");
    *
    * let email = EmailAddress::parse("foo@bar.com", None).unwrap();
    * assert_eq!(email.localPart(), "foo");
    * ```
    * @returns {string}
    */
    get localPart() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.emailaddress_localPart(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Returns the domain of the email address.
    *
    * Note that if you are using this library from rust, then consider using the `get_domain` method instead.
    * This returns a cloned copy of the domain string, instead of a borrowed `&str`, and exists purely for WASM interoperability.
    *
    * # Examples
    * ```
    * use email_address_parser::EmailAddress;
    *
    * let email = EmailAddress::new("foo", "bar.com", None).unwrap();
    * assert_eq!(email.domain(), "bar.com");
    *
    * let email = EmailAddress::parse("foo@bar.com", None).unwrap();
    * assert_eq!(email.domain(), "bar.com");
    * ```
    * @returns {string}
    */
    get domain() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.emailaddress_domain(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Returns the formatted EmailAddress.
    * This exists purely for WASM interoperability.
    * @returns {string}
    */
    toString() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.emailaddress_toString(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
/**
* Options for parsing.
*
* The is only one available option so far `is_lax` which can be set to
* `true` or `false` to  enable/disable obsolete parts parsing.
* The default is `false`.
*/
export class ParsingOptions {

    static __wrap(ptr) {
        const obj = Object.create(ParsingOptions.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_parsingoptions_free(ptr);
    }
    /**
    * @returns {boolean}
    */
    get is_lax() {
        var ret = wasm.__wbg_get_parsingoptions_is_lax(this.ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set is_lax(arg0) {
        wasm.__wbg_set_parsingoptions_is_lax(this.ptr, arg0);
    }
    /**
    * @param {boolean} is_lax
    */
    constructor(is_lax) {
        var ret = wasm.parsingoptions_new(is_lax);
        return ParsingOptions.__wrap(ret);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('message_parser_wasm_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_json_parse = function(arg0, arg1) {
        var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_59cb74e423758ede = function() {
        var ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
        var ret = getObject(arg1).stack;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

