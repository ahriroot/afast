class AFastByteBuffer{constructor(size=64){this.encoder=new TextEncoder();this.buf=new ArrayBuffer(size);this.view=new DataView(this.buf);this.length=0}ensure(size){if(this.length+size>this.buf.byteLength){let newSize=this.buf.byteLength*2;while(newSize<this.length+size)newSize*=2;const newBuf=new ArrayBuffer(newSize);new Uint8Array(newBuf).set(new Uint8Array(this.buf,0,this.length));this.buf=newBuf;this.view=new DataView(this.buf)}}pBy(b){this.ensure(1);this.view.setUint8(this.length,b);this.length+=1}pBys(arr){this.ensure(arr.length);new Uint8Array(this.buf,this.length,arr.length).set(arr);this.length+=arr.length}pB(b){this.pBy(b?1:0)}pI8(n){this.ensure(1);this.view.setInt8(this.length,n);this.length+=1}pU8(n){this.ensure(1);this.view.setUint8(this.length,n);this.length+=1}pI16(n){this.ensure(2);this.view.setInt16(this.length,n,false);this.length+=2}pU16(n){this.ensure(2);this.view.setUint16(this.length,n,false);this.length+=2}pI32(n){this.ensure(4);this.view.setInt32(this.length,n,false);this.length+=4}pU32(n){this.ensure(4);this.view.setUint32(this.length,n,false);this.length+=4}pI64(n){this.ensure(8);const high=Math.floor(n/2**32);const low=n>>>0;this.view.setInt32(this.length,high,false);this.view.setInt32(this.length+4,low,false);this.length+=8}pU64(n){this.ensure(8);const high=Math.floor(n/2**32);const low=n>>>0;this.view.setUint32(this.length,high,false);this.view.setUint32(this.length+4,low,false);this.length+=8}pI128(n){this.ensure(16);const highHigh=0;const highLow=Math.floor(n/2**32);const lowHigh=n>>>0;const lowLow=0;this.view.setUint32(this.length,highHigh,false);this.view.setUint32(this.length+4,highLow,false);this.view.setUint32(this.length+8,lowHigh,false);this.view.setUint32(this.length+12,lowLow,false);this.length+=16}pU128(n){this.ensure(16);const highHigh=0;const highLow=Math.floor(n/2**32);const lowHigh=n>>>0;const lowLow=0;this.view.setUint32(this.length,highHigh,false);this.view.setUint32(this.length+4,highLow,false);this.view.setUint32(this.length+8,lowHigh,false);this.view.setUint32(this.length+12,lowLow,false);this.length+=16}pF32(n){this.ensure(4);this.view.setFloat32(this.length,n,false);this.length+=4}pF64(n){this.ensure(8);this.view.setFloat64(this.length,n,false);this.length+=8}pS(str){const bytes=this.encoder.encode(str);this.pU32(bytes.length);this.pBys(bytes)}tU8A(){return new Uint8Array(this.buf,0,this.length)}}
class AFastByteReader{constructor(uint8Array){this.buf=uint8Array.buffer;this.view=new DataView(this.buf,uint8Array.byteOffset,uint8Array.byteLength);this.offset=0}rBy(){if(this.offset>=this.view.byteLength){throw new Error('Out of range: trying to read byte, but no more bytes left');}const v=this.view.getUint8(this.offset);this.offset+=1;return v}rBys(len){if(this.offset+len>this.view.byteLength){throw new Error(`Out of range:trying to read ${len}bytes,but only ${this.view.byteLength-this.offset}left`)}const bytes=new Uint8Array(this.view.buffer,this.view.byteOffset+this.offset,len);this.offset+=len;return bytes}rB(){const v=this.view.getUint8(this.offset);this.offset+=1;return v!==0}rI8(){const v=this.view.getInt8(this.offset);this.offset+=1;return v}rU8(){const v=this.view.getUint8(this.offset);this.offset+=1;return v}rI16(){const v=this.view.getInt16(this.offset,false);this.offset+=2;return v}rU16(){const v=this.view.getUint16(this.offset,false);this.offset+=2;return v}rI32(){const v=this.view.getInt32(this.offset,false);this.offset+=4;return v}rU32(){const v=this.view.getUint32(this.offset,false);this.offset+=4;return v}rI64(){const v=this.view.getBigInt64(this.offset,false);this.offset+=8;return v}rU64(){const v=this.view.getBigUint64(this.offset,false);this.offset+=8;return v}rI128(){const high=this.view.getBigInt64(this.offset,false);const low=this.view.getBigUint64(this.offset+8,false);this.offset+=16;return(high<<64n)|low}rU128(){const high=this.view.getBigUint64(this.offset,false);const low=this.view.getBigUint64(this.offset+8,false);this.offset+=16;return(high<<64n)|low}rF32(){const v=this.view.getFloat32(this.offset,false);this.offset+=4;return v}rF64(){const v=this.view.getFloat64(this.offset,false);this.offset+=8;return v}rS(){const len=this.rU32();const bytes=this.rBys(len);const decoder=new TextDecoder();return decoder.decode(bytes)}eof(){return this.offset>=this.view.byteLength}}
class AFastValidateError extends Error{constructor(message){super(message);this.name=this.constructor.name;if(Error.captureStackTrace){Error.captureStackTrace(this,this.constructor)}}}


class AFastClient {
/**
 * Create client
 * @param {{header:()=>Promise<{id:number;}>,call:(buf:Uint8Array)=>Promise<Uint8Array>,}} options
 */
constructor(options){this._options=options;if(!options.header){throw new Error('header is required');};this._header=options.header;if(!options.call){throw new Error('call is required');};this._call=options.call}
api = {
/**
 * Get user by id
 * @param {{id:number;}} request
 * @returns {{id:number;name:string;}}
 */
get_id: async (request) => {const _b1 = new AFastByteBuffer();const _header = await this._header();_b1.pU32(_header.id);_b1.pU32(1);_b1.pI64(request.id);const _b2 = new AFastByteReader(await this._call(_b1.tU8A()));_b2.rI32();const response = {id:_b2.rI64(),name:_b2.rS(),};return response;},
user: {
/**
 * Get user information
 * @param {{id:number;name:string;age:number;hobbies:Array<{id:number;name:string;}>;tags:Array<string>;gender:boolean|null;sex:{_type:0;id:number;}|{_type:1;name:string;};}} request
 * @returns {{sex:{_type:0;id:number;}|{_type:1;name:string;};id:number;name:string;age:number;hobbies:Array<{id:number;name:string;}>;tags:Array<string>;gender:boolean|null;}}
 */
get_user: async (request) => {if (request.age === 0) throw new AFastValidateError('name is required');if (request.age < 1) throw new AFastValidateError('name must be at least 1 character long');if (request.age > 100) throw new AFastValidateError('name must be at most 10 characters long');const _b1 = new AFastByteBuffer();const _header = await this._header();_b1.pU32(_header.id);_b1.pU32(0);_b1.pI64(request.id);_b1.pS(request.name);_b1.pU32(request.age);_b1.pU32(request.hobbies.length);for(let __item_0 of request.hobbies){_b1.pI64(__item_0.id);_b1.pS(__item_0.name);}_b1.pU32(request.tags.length);for(let __item_0 of request.tags){_b1.pS(__item_0);}if(request.gender === null)_b1.pU8(0);else{_b1.pU8(1);_b1.pB(request.gender);}_b1.pU32(request.sex._type);switch (request.sex._type) {case 0:_b1.pI64(request.sex.id);break;case 1:_b1.pS(request.sex.name);break;default:throw new Error('unknown variant');}const _b2 = new AFastByteReader(await this._call(_b1.tU8A()));_b2.rI32();const response = {sex:{...(function(){switch (_b2.rU32()) {case 0: return {id:_b2.rI64(),};case 1: return {name:_b2.rS(),};default:throw new Error('unknown variant');}}())},id:_b2.rI64(),name:_b2.rS(),age:_b2.rU32(),hobbies:Array.from({length:_b2.rU32()},()=>({id:_b2.rI64(),name:_b2.rS(),})),tags:Array.from({length:_b2.rU32()},()=>(_b2.rS())),gender:_b2.rU8() === 0 ? null : _b2.rB(),};return response;}
}
}
}

const client = new AFastClient({
    header: async () => {
        return {id: 12}
    },
    call: async (buf) => {
        console.log(buf);
        const response = await fetch('http://127.0.0.1:8081/api', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/octet-stream',
            },
            body: buf,
        });
        if (!response.ok) {
            const body = await response.text();
            throw new Error(`HTTP error: ${response.status} ${response.statusText} ${body}`);
        }
        const data = await response.arrayBuffer();
        return new Uint8Array(data);
    },
});

const main = async () => {
    let res1 = await client.api.user.get_user({ id: 1, name: 'Alice', age: 20, hobbies: [{ id: 2, name: "reaading" }], tags: ["tag1", "tag2"], gender: true, sex: { _type: 1, name: "1" } })
    console.log(res1);
    let res2 = await client.api.get_id({ id: 1 });
    console.log(res2);
};

main();
