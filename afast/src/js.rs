pub const JS: &str = r#"
// Exampe usage:
// const client = new AFastClient({
//     call: async (buf) => {
//         console.log(buf);
//         const response = await fetch('http://127.0.0.1:8081/api', {
//             method: 'POST',
//             headers: {
//                 'Content-Type': 'application/octet-stream',
//             },
//             body: buf,
//         });
//         if (!response.ok) {
//             const body = await response.text();
//             throw new Error(`HTTP error: ${response.status} ${response.statusText} ${body}`);
//         }
//         const data = await response.arrayBuffer();
//         return new Uint8Array(data);
//     },
// });
// 
// const main = async () => {
//     let res1 = await client.get_user({ id: 1, name: 'Alice', age: 20, hobbies: [{ id: 2, name: "reaading" }], tags: ["tag1", "tag2"], gender: true, sex: { _type: 1, name: "1" } })
//     console.log(res1);
//     let res2 = await client.get_id({ id: 1 });
//     console.log(res2);
// };
// 
// main();

class AFastByteBuffer{constructor(size=64){this.encoder=new TextEncoder();this.buf=new ArrayBuffer(size);this.view=new DataView(this.buf);this.length=0}ensure(size){if(this.length+size>this.buf.byteLength){let newSize=this.buf.byteLength*2;while(newSize<this.length+size)newSize*=2;const newBuf=new ArrayBuffer(newSize);new Uint8Array(newBuf).set(new Uint8Array(this.buf,0,this.length));this.buf=newBuf;this.view=new DataView(this.buf)}}pBy(b){this.ensure(1);this.view.setUint8(this.length,b);this.length+=1}pBys(arr){this.ensure(arr.length);new Uint8Array(this.buf,this.length,arr.length).set(arr);this.length+=arr.length}pB(b){this.pBy(b?1:0)}pI8(n){this.ensure(1);this.view.setInt8(this.length,n);this.length+=1}pU8(n){this.ensure(1);this.view.setUint8(this.length,n);this.length+=1}pI16(n){this.ensure(2);this.view.setInt16(this.length,n,false);this.length+=2}pU16(n){this.ensure(2);this.view.setUint16(this.length,n,false);this.length+=2}pI32(n){this.ensure(4);this.view.setInt32(this.length,n,false);this.length+=4}pU32(n){this.ensure(4);this.view.setUint32(this.length,n,false);this.length+=4}pI64(n){this.ensure(8);const high=Math.floor(n/2**32);const low=n>>>0;this.view.setInt32(this.length,high,false);this.view.setInt32(this.length+4,low,false);this.length+=8}pU64(n){this.ensure(8);const high=Math.floor(n/2**32);const low=n>>>0;this.view.setUint32(this.length,high,false);this.view.setUint32(this.length+4,low,false);this.length+=8}pI128(n){this.ensure(16);const highHigh=0;const highLow=Math.floor(n/2**32);const lowHigh=n>>>0;const lowLow=0;this.view.setUint32(this.length,highHigh,false);this.view.setUint32(this.length+4,highLow,false);this.view.setUint32(this.length+8,lowHigh,false);this.view.setUint32(this.length+12,lowLow,false);this.length+=16}pU128(n){this.ensure(16);const highHigh=0;const highLow=Math.floor(n/2**32);const lowHigh=n>>>0;const lowLow=0;this.view.setUint32(this.length,highHigh,false);this.view.setUint32(this.length+4,highLow,false);this.view.setUint32(this.length+8,lowHigh,false);this.view.setUint32(this.length+12,lowLow,false);this.length+=16}pF32(n){this.ensure(4);this.view.setFloat32(this.length,n,false);this.length+=4}pF64(n){this.ensure(8);this.view.setFloat64(this.length,n,false);this.length+=8}pS(str){const bytes=this.encoder.encode(str);this.pU32(bytes.length);this.pBys(bytes)}tU8A(){return new Uint8Array(this.buf,0,this.length)}}
class AFastByteReader{constructor(uint8Array){this.buf=uint8Array.buffer;this.view=new DataView(this.buf,uint8Array.byteOffset,uint8Array.byteLength);this.offset=0}rBy(){if(this.offset>=this.view.byteLength){throw new Error('Out of range: trying to read byte, but no more bytes left');}const v=this.view.getUint8(this.offset);this.offset+=1;return v}rBys(len){if(this.offset+len>this.view.byteLength){throw new Error(`Out of range:trying to read ${len}bytes,but only ${this.view.byteLength-this.offset}left`)}const bytes=new Uint8Array(this.view.buffer,this.view.byteOffset+this.offset,len);this.offset+=len;return bytes}rB(){const v=this.view.getUint8(this.offset);this.offset+=1;return v!==0}rI8(){const v=this.view.getInt8(this.offset);this.offset+=1;return v}rU8(){const v=this.view.getUint8(this.offset);this.offset+=1;return v}rI16(){const v=this.view.getInt16(this.offset,false);this.offset+=2;return v}rU16(){const v=this.view.getUint16(this.offset,false);this.offset+=2;return v}rI32(){const v=this.view.getInt32(this.offset,false);this.offset+=4;return v}rU32(){const v=this.view.getUint32(this.offset,false);this.offset+=4;return v}rI64(){const v=this.view.getBigInt64(this.offset,false);this.offset+=8;return v}rU64(){const v=this.view.getBigUint64(this.offset,false);this.offset+=8;return v}rI128(){const high=this.view.getBigInt64(this.offset,false);const low=this.view.getBigUint64(this.offset+8,false);this.offset+=16;return(high<<64n)|low}rU128(){const high=this.view.getBigUint64(this.offset,false);const low=this.view.getBigUint64(this.offset+8,false);this.offset+=16;return(high<<64n)|low}rF32(){const v=this.view.getFloat32(this.offset,false);this.offset+=4;return v}rF64(){const v=this.view.getFloat64(this.offset,false);this.offset+=8;return v}rS(){const len=this.rU32();const bytes=this.rBys(len);const decoder=new TextDecoder();return decoder.decode(bytes)}eof(){return this.offset>=this.view.byteLength}}
class AFastValidateError extends Error{constructor(message){super(message);this.name=this.constructor.name;if(Error.captureStackTrace){Error.captureStackTrace(this,this.constructor)}}}
"#;

pub const TS: &str = r#"
// @ts-nocheck

// Example usage:
// const client = new AFastClient({
//     call: async (buf: Uint8Array):Promise<Uint8Array> => {
//         console.log(buf);
//         const response = await fetch('http://127.0.0.1:8081/api', {
//             method: 'POST',
//             headers: {
//                 'Content-Type': 'application/octet-stream',
//             },
//             body: buf as any,
//         });
//         if (!response.ok) {
//             const body = await response.text();
//             throw new Error(`HTTP error: ${response.status} ${response.statusText} ${body}`);
//         }
//         const data = await response.arrayBuffer();
//         return new Uint8Array(data);
//     },
// });
// 
// const main = async () => {
//     let res1 = await client.get_user({ id: 1, name: 'Alice', age: 20, hobbies: [{ id: 2, name: "reaading" }], tags: ["tag1", "tag2"], gender: true, sex: { _type: 1, name: "1" } })
//     console.log(res1);
//     let res2 = await client.get_id({ id: 1 });
//     console.log(res2);
// };
// 
// main();

class AFastByteBuffer{private encoder:TextEncoder;private buf:ArrayBuffer;private view:DataView;private length:number;constructor(size:number=64){this.encoder=new TextEncoder();this.buf=new ArrayBuffer(size);this.view=new DataView(this.buf);this.length=0}private ensure(size:number):void{if(this.length+size>this.buf.byteLength){let newSize=this.buf.byteLength*2;while(newSize<this.length+size)newSize*=2;const newBuf=new ArrayBuffer(newSize);new Uint8Array(newBuf).set(new Uint8Array(this.buf,0,this.length));this.buf=newBuf;this.view=new DataView(this.buf)}}pBy(b:number):void{this.ensure(1);this.view.setUint8(this.length,b);this.length+=1}pBys(arr:Uint8Array):void{this.ensure(arr.length);new Uint8Array(this.buf,this.length,arr.length).set(arr);this.length+=arr.length}pB(b:boolean):void{this.pBy(b?1:0)}pI8(n:number):void{this.ensure(1);this.view.setInt8(this.length,n);this.length+=1}pU8(n:number):void{this.ensure(1);this.view.setUint8(this.length,n);this.length+=1}pI16(n:number):void{this.ensure(2);this.view.setInt16(this.length,n,false);this.length+=2}pU16(n:number):void{this.ensure(2);this.view.setUint16(this.length,n,false);this.length+=2}pI32(n:number):void{this.ensure(4);this.view.setInt32(this.length,n,false);this.length+=4}pU32(n:number):void{this.ensure(4);this.view.setUint32(this.length,n,false);this.length+=4}pI64(n:number):void{this.ensure(8);const high=Math.floor(n/2**32);const low=n>>>0;this.view.setInt32(this.length,high,false);this.view.setInt32(this.length+4,low,false);this.length+=8}pU64(n:number):void{this.ensure(8);const high=Math.floor(n/2**32);const low=n>>>0;this.view.setUint32(this.length,high,false);this.view.setUint32(this.length+4,low,false);this.length+=8}pI128(n:number):void{this.ensure(16);const highHigh=0;const highLow=Math.floor(n/2**32);const lowHigh=n>>>0;const lowLow=0;this.view.setUint32(this.length,highHigh,false);this.view.setUint32(this.length+4,highLow,false);this.view.setUint32(this.length+8,lowHigh,false);this.view.setUint32(this.length+12,lowLow,false);this.length+=16}pU128(n:number):void{this.ensure(16);const highHigh=0;const highLow=Math.floor(n/2**32);const lowHigh=n>>>0;const lowLow=0;this.view.setUint32(this.length,highHigh,false);this.view.setUint32(this.length+4,highLow,false);this.view.setUint32(this.length+8,lowHigh,false);this.view.setUint32(this.length+12,lowLow,false);this.length+=16}pF32(n:number):void{this.ensure(4);this.view.setFloat32(this.length,n,false);this.length+=4}pF64(n:number):void{this.ensure(8);this.view.setFloat64(this.length,n,false);this.length+=8}pS(str:string):void{const bytes=this.encoder.encode(str);this.pU32(bytes.length);this.pBys(bytes)}tU8A():Uint8Array{return new Uint8Array(this.buf,0,this.length)}}
class AFastByteReader{private buf:ArrayBuffer;private view:DataView;private offset:number;constructor(uint8Array:Uint8Array){this.buf=uint8Array.buffer as ArrayBuffer;this.view=new DataView(this.buf,uint8Array.byteOffset,uint8Array.byteLength);this.offset=0}private check(len:number):void{if(this.offset+len>this.view.byteLength){throw new Error(`Out of range:trying to read ${len}bytes,but only ${this.view.byteLength-this.offset}left`)}}rBy():number{this.check(1);const v=this.view.getUint8(this.offset);this.offset+=1;return v}rBys(len:number):Uint8Array{this.check(len);const bytes=new Uint8Array(this.view.buffer,this.view.byteOffset+this.offset,len);this.offset+=len;return bytes}rB():boolean{const v=this.rBy();return v!==0}rI8():number{this.check(1);const v=this.view.getInt8(this.offset);this.offset+=1;return v}rU8():number{this.check(1);const v=this.view.getUint8(this.offset);this.offset+=1;return v}rI16():number{this.check(2);const v=this.view.getInt16(this.offset,false);this.offset+=2;return v}rU16():number{this.check(2);const v=this.view.getUint16(this.offset,false);this.offset+=2;return v}rI32():number{this.check(4);const v=this.view.getInt32(this.offset,false);this.offset+=4;return v}rU32():number{this.check(4);const v=this.view.getUint32(this.offset,false);this.offset+=4;return v}rI64():number{this.check(8);const v=this.view.getBigInt64(this.offset,false);this.offset+=8;return Number(v)}rU64():number{this.check(8);const v=this.view.getBigUint64(this.offset,false);this.offset+=8;return Number(v)}rI128():number{this.check(16);const high=this.view.getBigInt64(this.offset,false);const low=this.view.getBigUint64(this.offset+8,false);this.offset+=16;return Number((high<<64n)|low)}rU128():number{this.check(16);const high=this.view.getBigUint64(this.offset,false);const low=this.view.getBigUint64(this.offset+8,false);this.offset+=16;return Number((high<<64n)|low)}rF32():number{this.check(4);const v=this.view.getFloat32(this.offset,false);this.offset+=4;return v}rF64():number{this.check(8);const v=this.view.getFloat64(this.offset,false);this.offset+=8;return v}rS():string{const len=this.rU32();const bytes=this.rBys(len);const decoder=new TextDecoder();return decoder.decode(bytes)}eof():boolean{return this.offset>=this.view.byteLength}}
class AFastValidateError extends Error{constructor(message:string){super(message);this.name=new.target.name;if((Error as any).captureStackTrace){(Error as any).captureStackTrace(this,new.target)}}}
type ClientCall=(data: Uint8Array)=>Promise<Uint8Array>;
type ClientOptions={call:ClientCall;[key: string]:any;}
"#;
