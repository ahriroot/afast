import{a as e,c as t,i as n,n as r,o as i,r as a,s as o,t as s}from"./Tabs-DRxzYkuC.js";import{An as c,Cn as l,En as u,Kt as d,On as f,Zt as p,dn as m,en as h,gn as g,i as _,nn as v,r as y,rn as b,tn as x,vn as S}from"./index-CpWB-UPI.js";var C={class:`index`},w={class:`container`},T=[`onClick`],E=`import { AFastClient } from 'xxx';

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
    call: async (buf) => {
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

const response = await client[.namespace1.namespace2].handle({});`,O=_(b({__name:`Index`,setup(_){let b=y(),O=u({});m(async()=>{O.value=await(await fetch(`/doc`)).json()});let k=e=>{b.push({name:`Service`,params:{name:e.name}})},A=u(!1);return(u,m)=>(g(),h(`div`,C,[v(f(a),null,{default:l(()=>[p(`div`,w,[v(f(e),{show:A.value,style:{width:`900px`},onClose:m[0]||=e=>A.value=!1},{default:l(()=>[v(f(o),null,{default:l(()=>[m[2]||=p(`h1`,null,`1: Get client code`,-1),m[3]||=p(`p`,null,`http://host/code/{service}/{lang}`,-1),m[4]||=p(`p`,null,`example: http://host/code/service1/js`,-1),m[5]||=p(`h1`,null,`2: Create client`,-1),v(f(s),{type:`segment`,animated:``},{default:l(()=>[v(f(r),{name:`js`,tab:`js`},{default:l(()=>[v(f(i),{code:E,language:`javascript`})]),_:1}),v(f(r),{name:`ts`,tab:`ts`},{default:l(()=>[v(f(i),{code:D,language:`javascript`})]),_:1})]),_:1})]),_:1})]),_:1},8,[`show`]),v(f(n),{justify:`end`},{default:l(()=>[v(f(t),{onClick:m[1]||=e=>A.value=!0},{default:l(()=>[...m[6]||=[x(`Example`,-1)]]),_:1})]),_:1}),m[7]||=p(`br`,null,null,-1),(g(!0),h(d,null,S(O.value.services,e=>(g(),h(`div`,{class:`service`,onClick:t=>k(e)},[v(f(o),null,{default:l(()=>[v(f(n),{align:`center`},{default:l(()=>[p(`h2`,null,c(e.name),1),p(`p`,null,c(e.desc),1),p(`span`,null,c(e.count)+` Handlers`,1)]),_:2},1024)]),_:2},1024)],8,T))),256))])]),_:1})]))}}),[[`__scopeId`,`data-v-54593cb7`]]);export{O as default};