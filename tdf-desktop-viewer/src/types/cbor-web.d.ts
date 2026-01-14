declare module 'cbor-web' {
    export function decode(data: Uint8Array | ArrayBuffer): any;
    export function encode(data: any): Uint8Array;
}
