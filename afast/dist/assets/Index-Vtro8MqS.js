import{a as e,c as t,i as n,n as r,o as i,r as a,s as o,t as s}from"./Tabs-CYqQQC3b.js";import{Bn as c,Dn as l,In as u,Nn as d,Rn as f,Sn as p,fn as m,hn as h,i as g,in as _,kn as v,ln as y,mn as b,pn as x,r as S}from"./index-DstwmKNe.js";var C={class:`index`},w={class:`container`},T=[`onClick`],E=`import { AFastClient } from 'xxx';

const client = new AFastClient({
    header: async () => {
        return {
            token: () => { return localStorage.getItem('token') },
        }
    },
    hook: async (header) => {
        console.log('hook:', header);
    },
    call: async (buf) => {
        console.log(buf);
        const response = await fetch('http://host/api', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/octet-stream',
            },
            body: buf,
        });
        if (!response.ok) {
            const body = await response.text();
            throw new Error(\`HTTP error: \${response.status} \${response.statusText} \${body}\`);
        }
        const data = await response.arrayBuffer();
        return new Uint8Array(data);
    },
});

const response = await client[.namespace1.namespace2].handle({});`,D=`import { AFastClient } from 'xxx';

const client = new AFastClient({
    header: async () => {
        return {
            token: () => { return localStorage.getItem('token') },
        }
    },
    hook: async (header: {token: string}) => {
        console.log('hook:', header);
    },
    call: async (buf: Uint8Array) => {
        console.log(buf);
        const response = await fetch('http://host/api', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/octet-stream',
            },
            body: buf as any,
        });
        if (!response.ok) {
            const body = await response.text();
            throw new Error(\`HTTP error: \${response.status} \${response.statusText} \${body}\`);
        }
        const data = await response.arrayBuffer();
        return new Uint8Array(data);
    },
});

const response = await client[.namespace1.namespace2].handle({});`,O=g(h({__name:`Index`,setup(h){let g=S(),O=u({});p(async()=>{O.value=await(await fetch(`/doc`)).json()});let k=e=>{g.push({name:`Service`,params:{name:e.name}})},A=u(!1);return(u,p)=>(l(),m(`div`,C,[b(f(a),null,{default:d(()=>[y(`div`,w,[b(f(e),{show:A.value,style:{width:`900px`},onClose:p[0]||=e=>A.value=!1},{default:d(()=>[b(f(o),null,{default:d(()=>[p[2]||=y(`h1`,null,`1: Get client code`,-1),p[3]||=y(`p`,null,`http://host/code/{service}/{lang}`,-1),p[4]||=y(`p`,null,`example: http://host/code/service1/js`,-1),p[5]||=y(`h1`,null,`2: Create client`,-1),b(f(s),{type:`segment`,animated:``},{default:d(()=>[b(f(r),{name:`js`,tab:`js`},{default:d(()=>[b(f(i),{code:E,language:`javascript`})]),_:1}),b(f(r),{name:`ts`,tab:`ts`},{default:d(()=>[b(f(i),{code:D,language:`javascript`})]),_:1})]),_:1})]),_:1})]),_:1},8,[`show`]),b(f(n),{justify:`end`},{default:d(()=>[b(f(t),{onClick:p[1]||=e=>A.value=!0,secondary:``},{default:d(()=>[...p[6]||=[x(`Example`,-1)]]),_:1})]),_:1}),p[7]||=y(`br`,null,null,-1),(l(!0),m(_,null,v(O.value.services,e=>(l(),m(`div`,{class:`service`,onClick:t=>k(e)},[b(f(o),null,{default:d(()=>[b(f(n),{align:`center`},{default:d(()=>[y(`h2`,null,c(e.name),1),y(`p`,null,c(e.desc),1),y(`span`,null,c(e.count)+` Handlers`,1)]),_:2},1024)]),_:2},1024)],8,T))),256))])]),_:1})]))}}),[[`__scopeId`,`data-v-e85c090d`]]);export{O as default};