import { Pair } from "./pair";
import { typedToArray } from "./utils";

export enum Error {
    // Last operation was a success
    Ok = 0,
    // Early end of stream
    EarlyEndOfStream = 1,
    // Unexpected data encountered while decoding byte stream
    FormattingError = 2,
}

/**
 * Boxes a value which could then be nullable in any context.
 */
export class Ref<T> {
    constructor(public value: T) {}
}

export class Result<T> {
    /**
     * Creates new Result with wrapped value
     * @param value Ref-wrapped value (success) or null (error)
     * @param error Error value
     * @param position Position of input stream 
     */
    constructor(public ref: Ref<T> | null, public error: Error, public position: usize) {}

    /**
     * Assumes that reference wrapper contains a value and then returns it
     */
    get value(): T {
        assert(this.hasValue());
        let ref = <Ref<T>>this.ref;
        return ref.value;
    }

    /**
     * Checks if given Result contains a value 
     */
    hasValue(): bool {
        return this.ref !== null;
    }
    /**
     * Checks if error value is set.
     * 
     * Truth also implies !hasValue(), false value implies hasValue()
     */

    hasError(): bool {
        return this.error != Error.Ok;
    }

    /**
     * For nullable types, this returns the value itself, or a null.
     */
    ok(): T | null {
        return this.hasValue() ? this.value : null;
    }
}

export function toBytesU8(num: u8): u8[] {
    return [num];
}

export function fromBytesLoad<T>(bytes: Uint8Array): Result<T> {
    let expectedSize = changetype<i32>(sizeof<T>())
    if (bytes.length < expectedSize) {
        return new Result<T>(null, Error.EarlyEndOfStream, 0);
    }
    const value = load<T>(bytes.dataStart);
    return new Result<T>(new Ref<T>(value), Error.Ok, expectedSize);
}

export function fromBytesU8(bytes: Uint8Array): Result<u8> {
    return fromBytesLoad<u8>(bytes);
}

// Converts u32 to little endian
export function toBytesU32(num: u32): u8[] {
    let bytes = new Uint8Array(4);
    store<u32>(bytes.dataStart, num);
    let result = new Array<u8>(4);
    for (var i = 0; i < 4; i++) {
        result[i] = bytes[i];
    }
    return result;
}

export function fromBytesU32(bytes: Uint8Array): Result<u32> {
    return fromBytesLoad<u32>(bytes);
}

export function toBytesI32(num: i32): u8[] {
    let bytes = new Uint8Array(4);
    store<i32>(bytes.dataStart, num);
    let result = new Array<u8>(4);
    for (var i = 0; i < 4; i++) {
        result[i] = bytes[i];
    }
    return result;
}

export function fromBytesI32(bytes: Uint8Array): Result<i32> {
    return fromBytesLoad<i32>(bytes);
}

export function toBytesU64(num: u64): u8[] {
    let bytes = new Uint8Array(8);
    store<u64>(bytes.dataStart, num);
    let result = new Array<u8>(8);
    for (var i = 0; i < 8; i++) {
        result[i] = bytes[i];
    }
    return result;
}

export function fromBytesU64(bytes: Uint8Array): Result<u64> {
    return fromBytesLoad<u64>(bytes);
}

export function toBytesPair(key: u8[], value: u8[]): u8[] {
    return key.concat(value);
}

export function toBytesMap(pairs: u8[][]): u8[] {
    // https://github.com/AssemblyScript/docs/blob/master/standard-library/map.md#methods
    // Gets the keys contained in this map as an array, in insertion order. This is preliminary while iterators are not supported.
    // See https://github.com/AssemblyScript/assemblyscript/issues/166
    var bytes = toBytesU32(<u32>pairs.length);
    for (var i = 0; i < pairs.length; i++) {
        var pairBytes = pairs[i];
        bytes = bytes.concat(pairBytes);
    }
    return bytes;
}

export function fromBytesMap<K, V>(
    bytes: Uint8Array,
    decodeKey: (bytes1: Uint8Array) => Result<K>,
    decodeValue: (bytes2: Uint8Array) => Result<V>,
): Result<Array<Pair<K, V>>> {
    const lengthResult = fromBytesU32(bytes);
    if (lengthResult.error != Error.Ok) {
        return new Result<Array<Pair<K, V>>>(null, Error.EarlyEndOfStream, 0);
    }
    const length = lengthResult.value;

    // Tracks how many bytes are parsed
    let currentPos = lengthResult.position;

    let result = new Array<Pair<K, V>>();

    if (length == 0) {
        let ref = new Ref<Array<Pair<K, V>>>(result);
        return new Result<Array<Pair<K, V>>>(ref, Error.Ok, lengthResult.position);
    }

    let bytes = bytes.subarray(currentPos);

    for (let i = 0; i < changetype<i32>(length); i++) {
        const keyResult = decodeKey(bytes);
        if (keyResult.error != Error.Ok) {
            return new Result<Array<Pair<K, V>>>(null, keyResult.error, keyResult.position);
        }

        currentPos += keyResult.position;
        bytes = bytes.subarray(keyResult.position);

        let valueResult = decodeValue(bytes);
        if (valueResult.error != Error.Ok) {
            return new Result<Array<Pair<K, V>>>(null, valueResult.error, valueResult.position);
        }

        currentPos += valueResult.position;
        bytes = bytes.subarray(valueResult.position);

        let pair = new Pair<K, V>(keyResult.value, valueResult.value);
        result.push(pair);
    }

    let ref = new Ref<Array<Pair<K, V>>>(result);
    return new Result<Array<Pair<K, V>>>(ref, Error.Ok, currentPos);
}

export function toBytesString(s: String): u8[] {
    let bytes = toBytesU32(<u32>s.length);
    for (let i = 0; i < s.length; i++) {
        let charCode = s.charCodeAt(i);
        // Assumes ascii encoding (i.e. charCode < 0x80)
        bytes.push(<u8>charCode);
    }
    return bytes;
}

export function fromBytesString(s: Uint8Array): Result<String> {
    var lenResult = fromBytesI32(s);
    if (lenResult.error != Error.Ok) {
        return new Result<String>(null, Error.EarlyEndOfStream, 0);
    }

    let currentPos = lenResult.position;

    const leni32 = lenResult.value;
    if (s.length < leni32 + 4) {
        return new Result<String>(null, Error.EarlyEndOfStream, 0);
    }
    var result = "";
    for (var i = 0; i < leni32; i++) {
        result += String.fromCharCode(s[4 + i]);
    }
    let ref = new Ref<String>(result);
    return new Result<String>(ref, Error.Ok, currentPos + leni32);
}

export function toBytesArrayU8(arr: Array<u8>): u8[] {
    let bytes = toBytesU32(<u32>arr.length);
    return bytes.concat(arr);
}

export function fromBytesArrayU8(bytes: Uint8Array): Result<Array<u8>> {
    var lenResult = fromBytesI32(bytes);
    if (lenResult.error != Error.Ok) {
        return new Result<String>(null, Error.EarlyEndOfStream, 0);
    }

    let currentPos = lenResult.position;

    const leni32 = lenResult.value;
    if (s.length < leni32 + 4) {
        return new Result<String>(null, Error.EarlyEndOfStream, 0);
    }

    let result = typedToArray(bytes.subarray(currentPos));
    let ref = new Ref<String>(result);
    return new Result<String>(ref, Error.Ok, currentPos + leni32, currentPos + leni32);
}

export function toBytesVecT<T>(ts: T[]): Array<u8> {
    let bytes = toBytesU32(<u32>ts.length);
    for (let i = 0; i < ts.length; i++) {
        bytes = bytes.concat(ts[i].toBytes());
    }
    return bytes;
}

export function fromBytesArray<T>(bytes: Uint8Array, decodeItem: (bytes: Uint8Array) => Result<T>): Result<Array<T>> {
    var lenResult = fromBytesI32(bytes);
    if (lenResult.error != Error.Ok) {
        return new Result<Array<T>>(null, Error.EarlyEndOfStream, 0);
    }

    let len = lenResult.value;
    let currentPos = lenResult.position;
    let head = bytes.subarray(currentPos);

    let result: Array<T> = new Array<T>();

    for (let i = 0; i < len; ++i) {
        let decodeResult = decodeItem(head);
        if (decodeResult.error != Error.Ok) {
            return new Result<Array<T>>(null, decodeResult.error, 0);
        }
        currentPos += decodeResult.position;
        result.push(decodeResult.value);
        head = head.subarray(decodeResult.position);
    }

    let ref = new Ref<Array<T>>(result);
    return new Result<Array<T>>(ref, Error.Ok, currentPos);
}

export function fromBytesStringList(bytes: Uint8Array): Result<Array<String>> {
    return fromBytesArray(bytes, fromBytesString);
}

export function toBytesStringList(arr: String[]): u8[] {
    let data = toBytesU32(arr.length);
    for (let i = 0; i < arr.length; i++) {
        const strBytes = toBytesString(arr[i]);
        data = data.concat(strBytes);
    }
    return data;
}
