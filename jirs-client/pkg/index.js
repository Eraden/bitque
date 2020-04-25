import * as wasm from './index_bg.wasm';

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

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

const lTextDecoder = typeof TextDecoder === 'undefined' ? require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

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

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error('expected a boolean argument');
    }
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? require('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

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

    if (typeof(arg) !== 'string') throw new Error('expected a string argument');

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
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) wasm.__wbindgen_export_2.get(dtor)(a, state.b);
            else state.a = a;
        }
    };
    real.original = state;
    return real;
}

function logError(e) {
    let error = (function () {
        try {
            return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
        } catch(_) {
            return "<failed to stringify thrown value>";
        }
    }());
    console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
    throw e;
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}
function __wbg_adapter_22(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4bd300be2535535c(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_25(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__ha050be9ecc9bb460(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_28(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8bddde00850bf6f6(arg0, arg1, arg2);
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(dtor)(state.a, state.b);
                state.a = 0;
            }
        }
    };
    real.original = state;
    return real;
}
function __wbg_adapter_31(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb4e53ca38e9bdeab(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_34(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7e2a599b80717fce(arg0, arg1);
}

/**
* @param {string} url
*/
export function set_host_url(url) {
    var ptr0 = passStringToWasm0(url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.set_host_url(ptr0, len0);
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
* @param {any} value
*/
export function handle_ws_message(value) {
    try {
        wasm.handle_ws_message(addBorrowedObject(value));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

/**
*/
export function reconnected() {
    wasm.reconnected();
}

/**
*/
export function render() {
    wasm.render();
}

function handleError(e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
}
function __wbg_adapter_283(arg0, arg1, arg2, arg3, arg4) {
    _assertNum(arg0);
    _assertNum(arg1);
    _assertNum(arg3);
    wasm.wasm_bindgen__convert__closures__invoke3_mut__h190177d3061e64e8(arg0, arg1, addHeapObject(arg2), arg3, addHeapObject(arg4));
}

export const __wbindgen_cb_forget = function(arg0) {
    takeObject(arg0);
};

export const __wbindgen_json_parse = function(arg0, arg1) {
    var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export const __wbg_sendbincode_c00a968329bf155f = function(arg0) {
    try {
        send_bin_code(takeObject(arg0));
    } catch (e) {
        logError(e)
    }
};

export const __wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    var ret = false;
    _assertBoolean(ret);
    return ret;
};

export const __wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

export const __wbindgen_object_clone_ref = function(arg0) {
    var ret = getObject(arg0);
    return addHeapObject(ret);
};

export const __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
    try {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_new_59cb74e423758ede = function() {
    try {
        var ret = new Error();
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_stack_558ba5917b466edd = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).stack;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

export const __wbindgen_is_undefined = function(arg0) {
    var ret = getObject(arg0) === undefined;
    _assertBoolean(ret);
    return ret;
};

export const __wbg_instanceof_Window_a633dbe0900c728a = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof Window;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_document_07444f1bbea314bb = function(arg0) {
    try {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_location_e50b7d71ca1b82bb = function(arg0) {
    try {
        var ret = getObject(arg0).location;
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_history_66c8db535cd48e93 = function(arg0) {
    try {
        try {
            var ret = getObject(arg0).history;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_performance_cc98652048194dbe = function(arg0) {
    try {
        var ret = getObject(arg0).performance;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_localStorage_48f33617aec46f3f = function(arg0) {
    try {
        try {
            var ret = getObject(arg0).localStorage;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_cancelAnimationFrame_52201160d60ee66b = function(arg0, arg1) {
    try {
        try {
            getObject(arg0).cancelAnimationFrame(arg1);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_requestAnimationFrame_10a415a97fc2123f = function(arg0, arg1) {
    try {
        try {
            var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
            _assertNum(ret);
            return ret;
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_setInterval_8132b2c4bdf1d970 = function(arg0, arg1, arg2) {
    try {
        try {
            var ret = getObject(arg0).setInterval(getObject(arg1), arg2);
            _assertNum(ret);
            return ret;
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_location_bfe965ae49e868fd = function(arg0) {
    try {
        var ret = getObject(arg0).location;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_body_5f6496599a0f5214 = function(arg0) {
    try {
        var ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_activeElement_f89b19c661a64911 = function(arg0) {
    try {
        var ret = getObject(arg0).activeElement;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_createElement_5a267cb074dc073b = function(arg0, arg1, arg2) {
    try {
        try {
            var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_createElementNS_6dd6bfc8ad570e2a = function(arg0, arg1, arg2, arg3, arg4) {
    try {
        try {
            var ret = getObject(arg0).createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_createTextNode_b131e8421d578817 = function(arg0, arg1, arg2) {
    try {
        var ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_getElementById_633c94a971ae0eb9 = function(arg0, arg1, arg2) {
    try {
        var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_querySelector_2dabb5b08003bfad = function(arg0, arg1, arg2) {
    try {
        try {
            var ret = getObject(arg0).querySelector(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlBodyElement_32978d3c6c44fed1 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLBodyElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_Node_6266219467033646 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof Node;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_nodeType_7831b036570beb68 = function(arg0) {
    try {
        var ret = getObject(arg0).nodeType;
        _assertNum(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_childNodes_ad579febe85f642b = function(arg0) {
    try {
        var ret = getObject(arg0).childNodes;
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_firstChild_448c49a77e22efe0 = function(arg0) {
    try {
        var ret = getObject(arg0).firstChild;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_nextSibling_a89e92f7f3b94819 = function(arg0) {
    try {
        var ret = getObject(arg0).nextSibling;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_textContent_2f92c89d911e8458 = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).textContent;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_textContent_1fb8e2642c9c164e = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_appendChild_c1802f48577b21f6 = function(arg0, arg1) {
    try {
        try {
            var ret = getObject(arg0).appendChild(getObject(arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_insertBefore_f40a70a9913f64f5 = function(arg0, arg1, arg2) {
    try {
        try {
            var ret = getObject(arg0).insertBefore(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_removeChild_9a521558bd3fd73e = function(arg0, arg1) {
    try {
        try {
            var ret = getObject(arg0).removeChild(getObject(arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_replaceChild_78351f18f620584b = function(arg0, arg1, arg2) {
    try {
        try {
            var ret = getObject(arg0).replaceChild(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_cssText_346cceb7b061960b = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).cssText = getStringFromWasm0(arg1, arg2);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_href_fbb6c66f424097e9 = function(arg0, arg1) {
    try {
        try {
            var ret = getObject(arg1).href;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_pathname_8a2fe4c2e3a07b51 = function(arg0, arg1) {
    try {
        try {
            var ret = getObject(arg1).pathname;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_getItem_6f2992539addebe8 = function(arg0, arg1, arg2, arg3) {
    try {
        try {
            var ret = getObject(arg1).getItem(getStringFromWasm0(arg2, arg3));
            var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_removeItem_025bc9d3485b59e3 = function(arg0, arg1, arg2) {
    try {
        try {
            getObject(arg0).removeItem(getStringFromWasm0(arg1, arg2));
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_setItem_56835e22c5609ad0 = function(arg0, arg1, arg2, arg3, arg4) {
    try {
        try {
            getObject(arg0).setItem(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlProgressElement_6ef7063975a59dc6 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLProgressElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_105a70a1e15e7a53 = function(arg0) {
    try {
        var ret = getObject(arg0).value;
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_c1fea20a991169a4 = function(arg0, arg1) {
    try {
        getObject(arg0).value = arg1;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_MouseEvent_61743ce16c6f55f9 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof MouseEvent;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_DragEvent_b52d152160ee2739 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof DragEvent;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HashChangeEvent_7786d2bc58dbfeb6 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HashChangeEvent;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_newURL_b665280891f394ff = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).newURL;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlMenuItemElement_24af4c2eab7719af = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLMenuItemElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_checked_05e04ccf9a640cd7 = function(arg0, arg1) {
    try {
        getObject(arg0).checked = arg1 !== 0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlMeterElement_821b6ab5a5e53cf7 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLMeterElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_54eb4161b6fc4113 = function(arg0) {
    try {
        var ret = getObject(arg0).value;
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_eb79c2b07c010a41 = function(arg0, arg1) {
    try {
        getObject(arg0).value = arg1;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlOutputElement_a3e5c77eae401de2 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLOutputElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_8124a1c4360bf7a4 = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_1a66d388ad5a6dfb = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_pathname_c211a83f58e58cce = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).pathname;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_search_0136753784ea3612 = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).search;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_hash_c297cea76831a0a3 = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).hash;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_new_f35451690fdfd40b = function(arg0, arg1) {
    try {
        try {
            var ret = new URL(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_newwithbase_71236cbfc2601de8 = function(arg0, arg1, arg2, arg3) {
    try {
        try {
            var ret = new URL(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_Element_fc5de05fee1e0030 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof Element;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_namespaceURI_a890993882ac3334 = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).namespaceURI;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_tagName_dde9312e3e9271bf = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).tagName;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_innerHTML_ebf37095d2177081 = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_closest_5f908507d3eb767e = function(arg0, arg1, arg2) {
    try {
        try {
            var ret = getObject(arg0).closest(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_getAttribute_0cfffe0e4135c484 = function(arg0, arg1, arg2, arg3) {
    try {
        var ret = getObject(arg1).getAttribute(getStringFromWasm0(arg2, arg3));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_getAttributeNames_837465e3b012c632 = function(arg0) {
    try {
        var ret = getObject(arg0).getAttributeNames();
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_removeAttribute_518c8ed1a02058f8 = function(arg0, arg1, arg2) {
    try {
        try {
            getObject(arg0).removeAttribute(getStringFromWasm0(arg1, arg2));
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_setAttribute_3021f1b348fd14a5 = function(arg0, arg1, arg2, arg3, arg4) {
    try {
        try {
            getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlButtonElement_2b24f8a7272a06bf = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLButtonElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_5e9006eb24add22a = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_916f94d35a296fd7 = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlLiElement_f985e6b2bb7788bb = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLLIElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_114272d68d18ae59 = function(arg0) {
    try {
        var ret = getObject(arg0).value;
        _assertNum(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_e98336ce99f60753 = function(arg0, arg1) {
    try {
        getObject(arg0).value = arg1;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlTextAreaElement_a07fcbfd18542e06 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLTextAreaElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_967003eb801722ab = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_57c725aca44d9296 = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_select_0efd67657899de8b = function(arg0) {
    try {
        getObject(arg0).select();
    } catch (e) {
        logError(e)
    }
};

export const __wbg_setSelectionRange_68e12cee5dc66548 = function(arg0, arg1, arg2) {
    try {
        try {
            getObject(arg0).setSelectionRange(arg1 >>> 0, arg2 >>> 0);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_length_934cf12fc6042868 = function(arg0) {
    try {
        var ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_get_c5660d7999b12a8d = function(arg0, arg1) {
    try {
        var ret = getObject(arg0)[arg1 >>> 0];
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_error_b23efba5bfb5cec5 = function(arg0) {
    try {
        console.error(getObject(arg0));
    } catch (e) {
        logError(e)
    }
};

export const __wbg_log_c180b836187d3c94 = function(arg0) {
    try {
        console.log(getObject(arg0));
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlElement_7e7a87e33735b8a3 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_style_bef32919c604ce57 = function(arg0) {
    try {
        var ret = getObject(arg0).style;
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_focus_764ff7af03580982 = function(arg0) {
    try {
        try {
            getObject(arg0).focus();
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_addEventListener_9f325a58d77d2781 = function(arg0, arg1, arg2, arg3) {
    try {
        try {
            getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_removeEventListener_54932b092806fbeb = function(arg0, arg1, arg2, arg3) {
    try {
        try {
            getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlInputElement_5f61a3d2d3d02410 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLInputElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_checked_8f4b67dbaf90811e = function(arg0, arg1) {
    try {
        getObject(arg0).checked = arg1 !== 0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_type_280155b20a95745d = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).type;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_06af6d392334302f = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_ce3b7a6a03d76643 = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_selectionStart_5c7291272c761cfd = function(arg0, arg1) {
    try {
        try {
            var ret = getObject(arg1).selectionStart;
            if (!isLikeNone(ret)) {
                _assertNum(ret);
            }
            getInt32Memory0()[arg0 / 4 + 1] = isLikeNone(ret) ? 0 : ret;
            getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_selectionStart_e6651e6fb5a244de = function(arg0, arg1, arg2) {
    try {
        try {
            getObject(arg0).selectionStart = arg1 === 0 ? undefined : arg2 >>> 0;
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_selectionEnd_51a58669058e8ddd = function(arg0, arg1) {
    try {
        try {
            var ret = getObject(arg1).selectionEnd;
            if (!isLikeNone(ret)) {
                _assertNum(ret);
            }
            getInt32Memory0()[arg0 / 4 + 1] = isLikeNone(ret) ? 0 : ret;
            getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_selectionEnd_878e850f043e6ce5 = function(arg0, arg1, arg2) {
    try {
        try {
            getObject(arg0).selectionEnd = arg1 === 0 ? undefined : arg2 >>> 0;
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlOptionElement_b993aff294faa1eb = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLOptionElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_872ded146923434a = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_bd2d63082a07fdeb = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_now_ce4a6a89baf241c9 = function(arg0) {
    try {
        var ret = getObject(arg0).now();
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_execCommand_80f5ad7d08e14184 = function(arg0, arg1, arg2) {
    try {
        try {
            var ret = getObject(arg0).execCommand(getStringFromWasm0(arg1, arg2));
            _assertBoolean(ret);
            return ret;
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_KeyboardEvent_113766af8a69da33 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof KeyboardEvent;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_altKey_cd8189e0b7b180ac = function(arg0) {
    try {
        var ret = getObject(arg0).altKey;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_ctrlKey_d088b8345f4b52d9 = function(arg0) {
    try {
        var ret = getObject(arg0).ctrlKey;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_shiftKey_12ac38b11e05bd66 = function(arg0) {
    try {
        var ret = getObject(arg0).shiftKey;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_key_02aa4a0ffa18017e = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).key;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_PopStateEvent_872fc91f6e68893e = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof PopStateEvent;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_state_969f7e234cf15445 = function(arg0) {
    try {
        var ret = getObject(arg0).state;
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_target_7c8691623acab2b6 = function(arg0) {
    try {
        var ret = getObject(arg0).target;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_preventDefault_a94db094b84ac446 = function(arg0) {
    try {
        getObject(arg0).preventDefault();
    } catch (e) {
        logError(e)
    }
};

export const __wbg_stopPropagation_61518782238c8a3c = function(arg0) {
    try {
        getObject(arg0).stopPropagation();
    } catch (e) {
        logError(e)
    }
};

export const __wbg_pushState_5d9e642afb5b1c81 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
    try {
        try {
            getObject(arg0).pushState(getObject(arg1), getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlDataElement_3672e9e5ac16944f = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLDataElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_6a5cb29a213847d4 = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_b86273d36b4de3c9 = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlParamElement_e9beef115eddc3e1 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLParamElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_efbd1e2e0b7bdede = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_4ddeaa26d6e3a307 = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlSelectElement_c74c6fac5ac0a85e = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLSelectElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_896c16be44c622ac = function(arg0, arg1) {
    try {
        var ret = getObject(arg1).value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_value_3e7484525812fe93 = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_forEach_3fd7b3cb7d80997e = function(arg0, arg1, arg2) {
    try {
        try {
            var state0 = {a: arg1, b: arg2};
            var cb0 = (arg0, arg1, arg2) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_283(a, state0.b, arg0, arg1, arg2);
                } finally {
                    state0.a = a;
                }
            };
            getObject(arg0).forEach(cb0);
        } finally {
            state0.a = state0.b = 0;
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_newnoargs_ebdc90c3d1e4e55d = function(arg0, arg1) {
    try {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_call_804d3ad7e8acd4d5 = function(arg0, arg1) {
    try {
        try {
            var ret = getObject(arg0).call(getObject(arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_is_6494d77336bd856e = function(arg0, arg1) {
    try {
        var ret = Object.is(getObject(arg0), getObject(arg1));
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_resolve_3e5970e9c931a3c2 = function(arg0) {
    try {
        var ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_then_d797310661d9e275 = function(arg0, arg1) {
    try {
        var ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_then_e37e0b9ef0995585 = function(arg0, arg1, arg2) {
    try {
        var ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_globalThis_48a5e9494e623f26 = function() {
    try {
        try {
            var ret = globalThis.globalThis;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_self_25067cb019cade42 = function() {
    try {
        try {
            var ret = self.self;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_window_9e80200b35aa30f8 = function() {
    try {
        try {
            var ret = window.window;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_global_7583a634265a91fc = function() {
    try {
        try {
            var ret = global.global;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_new_da17c07b1fbb4a8b = function(arg0) {
    try {
        var ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_length_b7dc6aed3ca09be1 = function(arg0) {
    try {
        var ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_getindex_bfc8a3c5fa1164f3 = function(arg0, arg1) {
    try {
        var ret = getObject(arg0)[arg1 >>> 0];
        _assertNum(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbindgen_string_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    var ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbindgen_debug_string = function(arg0, arg1) {
    var ret = debugString(getObject(arg1));
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export const __wbindgen_closure_wrapper32845 = function(arg0, arg1, arg2) {
    try {
        var ret = makeMutClosure(arg0, arg1, 870, __wbg_adapter_25);
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbindgen_closure_wrapper2726 = function(arg0, arg1, arg2) {
    try {
        var ret = makeMutClosure(arg0, arg1, 54, __wbg_adapter_28);
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbindgen_closure_wrapper2728 = function(arg0, arg1, arg2) {
    try {
        var ret = makeMutClosure(arg0, arg1, 52, __wbg_adapter_22);
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbindgen_closure_wrapper28454 = function(arg0, arg1, arg2) {
    try {
        var ret = makeClosure(arg0, arg1, 822, __wbg_adapter_34);
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbindgen_closure_wrapper2730 = function(arg0, arg1, arg2) {
    try {
        var ret = makeClosure(arg0, arg1, 50, __wbg_adapter_31);
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

