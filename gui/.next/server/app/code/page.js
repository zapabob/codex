(()=>{var e={};e.id=497,e.ids=[497],e.modules={72934:e=>{"use strict";e.exports=require("next/dist/client/components/action-async-storage.external.js")},54580:e=>{"use strict";e.exports=require("next/dist/client/components/request-async-storage.external.js")},45869:e=>{"use strict";e.exports=require("next/dist/client/components/static-generation-async-storage.external.js")},20399:e=>{"use strict";e.exports=require("next/dist/compiled/next-server/app-page.runtime.prod.js")},55315:e=>{"use strict";e.exports=require("path")},17360:e=>{"use strict";e.exports=require("url")},61699:(e,t,r)=>{"use strict";r.r(t),r.d(t,{GlobalError:()=>s.a,__next_app__:()=>x,originalPathname:()=>p,pages:()=>d,routeModule:()=>u,tree:()=>c}),r(35777),r(73888),r(35866);var a=r(23191),i=r(88716),n=r(37922),s=r.n(n),o=r(95231),l={};for(let e in o)0>["default","tree","pages","GlobalError","originalPathname","__next_app__","routeModule"].indexOf(e)&&(l[e]=()=>o[e]);r.d(t,l);let c=["",{children:["code",{children:["__PAGE__",{},{page:[()=>Promise.resolve().then(r.bind(r,35777)),"C:\\Users\\downl\\Desktop\\codex\\gui\\src\\app\\code\\page.tsx"]}]},{metadata:{icon:[async e=>(await Promise.resolve().then(r.bind(r,73881))).default(e)],apple:[],openGraph:[],twitter:[],manifest:void 0}}]},{layout:[()=>Promise.resolve().then(r.bind(r,73888)),"C:\\Users\\downl\\Desktop\\codex\\gui\\src\\app\\layout.tsx"],"not-found":[()=>Promise.resolve().then(r.t.bind(r,35866,23)),"next/dist/client/components/not-found-error"],metadata:{icon:[async e=>(await Promise.resolve().then(r.bind(r,73881))).default(e)],apple:[],openGraph:[],twitter:[],manifest:void 0}}],d=["C:\\Users\\downl\\Desktop\\codex\\gui\\src\\app\\code\\page.tsx"],p="/code/page",x={require:r,loadChunk:()=>Promise.resolve()},u=new a.AppPageRouteModule({definition:{kind:i.x.APP_PAGE,page:"/code/page",pathname:"/code",bundlePath:"",filename:"",appPaths:[]},userland:{loaderTree:c}})},63693:(e,t,r)=>{Promise.resolve().then(r.bind(r,56557))},94893:(e,t,r)=>{"use strict";r.d(t,{Z:()=>a});let a=(0,r(62881).Z)("play",[["path",{d:"M5 5a2 2 0 0 1 3.008-1.728l11.997 6.998a2 2 0 0 1 .003 3.458l-12 7A2 2 0 0 1 5 19z",key:"10ikf1"}]])},56557:(e,t,r)=>{"use strict";r.r(t),r.d(t,{default:()=>b});var a=r(10326),i=r(17577),n=r(42118),s=r(94893),o=r(62881);let l=(0,o.Z)("save",[["path",{d:"M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z",key:"1c8476"}],["path",{d:"M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7",key:"1ydtos"}],["path",{d:"M7 3v4a1 1 0 0 0 1 1h7",key:"t51u73"}]]),c=(0,o.Z)("folder-open",[["path",{d:"m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2",key:"usdka0"}]]);var d=r(3634),p=r(92498);let x=(0,o.Z)("terminal",[["path",{d:"M12 19h8",key:"baeox8"}],["path",{d:"m4 17 6-6-6-6",key:"1yngyt"}]]);var u=r(33891),h=r(55030),m=r(2494);let f=[{value:"javascript",label:"JavaScript",extension:"js"},{value:"typescript",label:"TypeScript",extension:"ts"},{value:"python",label:"Python",extension:"py"},{value:"rust",label:"Rust",extension:"rs"},{value:"go",label:"Go",extension:"go"},{value:"bash",label:"Bash",extension:"sh"},{value:"powershell",label:"PowerShell",extension:"ps1"}],g={javascript:`// JavaScript Code Example
function fibonacci(n) {
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}

console.log('Fibonacci of 10:', fibonacci(10));`,typescript:`// TypeScript Code Example
interface User {
  id: number;
  name: string;
  email: string;
}

function createUser(id: number, name: string, email: string): User {
  return { id, name, email };
}

const user = createUser(1, 'John Doe', 'john@example.com');
console.log('Created user:', user);`,python:`# Python Code Example
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

if __name__ == "__main__":
    result = fibonacci(10)
    print(f"Fibonacci of 10: {result}")`,rust:`// Rust Code Example
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let result = fibonacci(10);
    println!("Fibonacci of 10: {}", result);
}`,go:`// Go Code Example
package main

import "fmt"

func fibonacci(n int) int {
    if n <= 1 {
        return n
    }
    return fibonacci(n-1) + fibonacci(n-2)
}

func main() {
    result := fibonacci(10)
    fmt.Printf("Fibonacci of 10: %d\\n", result)
}`,bash:`#!/bin/bash

# Bash Script Example
echo "Current directory: $(pwd)"
echo "Files in directory:"
ls -la

# Simple loop
for i in {1..5}; do
    echo "Count: $i"
done`,powershell:`# PowerShell Script Example
Write-Host "Current directory: $(Get-Location)"
Write-Host "Files in directory:"
Get-ChildItem -Force

# Simple loop
for ($i = 1; $i -le 5; $i++) {
    Write-Host "Count: $i"
}`};function b(){let{executeCommand:e,state:t}=(0,m.F)(),[r,o]=(0,i.useState)(g.javascript),[b,y]=(0,i.useState)("javascript"),[j,v]=(0,i.useState)("script.js"),[C,w]=(0,i.useState)(!1),[k,S]=(0,i.useState)(null),[P,_]=(0,i.useState)(null),[$,z]=(0,i.useState)([]),E=e=>{y(e),o(g[e]||""),S(null),_(null)},F=async()=>{if(!r.trim()){_("コードを入力してください");return}w(!0),S(null),_(null);let t=Date.now();try{let a;switch(b){case"javascript":a=`node -e "${r.replace(/"/g,'\\"')}"`;break;case"typescript":a=`npx ts-node -e "${r.replace(/"/g,'\\"')}"`;break;case"python":a=`python3 -c "${r.replace(/"/g,'\\"')}"`;break;case"rust":a=`echo "${r}" > /tmp/temp.rs && rustc /tmp/temp.rs -o /tmp/temp && /tmp/temp`;break;case"go":a=`echo "${r}" > /tmp/temp.go && go run /tmp/temp.go`;break;case"bash":a=`bash -c "${r.replace(/"/g,'\\"')}"`;break;case"powershell":a=`powershell -Command "${r.replace(/"/g,'\\"')}"`;break;default:throw Error(`Unsupported language: ${b}`)}let i=await e(a),n=Date.now()-t;S({exitCode:i.exitCode,stdout:i.stdout,stderr:i.stderr,executionTime:n})}catch(e){_(e instanceof Error?e.message:"実行中にエラーが発生しました")}finally{w(!1)}},B=async()=>{try{let t=`echo "${r.replace(/"/g,'\\"')}" > ${j}`;await e(t),z(e=>[...e,j]),_(null)}catch(e){_(e instanceof Error?e.message:"保存中にエラーが発生しました")}},G=async()=>{try{let t=`cat ${j}`,r=await e(t);0===r.exitCode?(o(r.stdout),_(null)):_("ファイルの読み込みに失敗しました")}catch(e){_(e instanceof Error?e.message:"ファイルの読み込み中にエラーが発生しました")}};return a.jsx(u.c,{title:"コード実行",children:(0,a.jsxs)(n.Box,{sx:{height:"calc(100vh - 200px)",display:"flex",flexDirection:"column",gap:2},children:[(0,a.jsxs)(h.Z,{header:"コード実行設定",children:[(0,a.jsxs)(n.Grid,{container:!0,spacing:2,alignItems:"center",children:[a.jsx(n.Grid,{item:!0,xs:12,md:3,children:(0,a.jsxs)(n.FormControl,{fullWidth:!0,size:"small",children:[a.jsx(n.InputLabel,{children:"言語"}),a.jsx(n.Select,{value:b,label:"言語",onChange:e=>E(e.target.value),children:f.map(e=>a.jsx(n.MenuItem,{value:e.value,children:e.label},e.value))})]})}),a.jsx(n.Grid,{item:!0,xs:12,md:3,children:a.jsx(n.TextField,{fullWidth:!0,size:"small",label:"ファイル名",value:j,onChange:e=>v(e.target.value),placeholder:`script.${f.find(e=>e.value===b)?.extension}`})}),a.jsx(n.Grid,{item:!0,xs:12,md:6,children:(0,a.jsxs)(n.Box,{sx:{display:"flex",gap:1,flexWrap:"wrap"},children:[a.jsx(n.Button,{variant:"contained",startIcon:C?a.jsx(n.CircularProgress,{size:16}):a.jsx(s.Z,{}),onClick:F,disabled:C||!t.isConnected,sx:{background:"linear-gradient(45deg, #0061a4, #1976d2)","&:hover":{background:"linear-gradient(45deg, #004d8f, #1565c0)"}},children:C?"実行中...":"実行"}),a.jsx(n.Button,{variant:"outlined",startIcon:a.jsx(l,{}),onClick:B,disabled:!r.trim(),children:"保存"}),a.jsx(n.Button,{variant:"outlined",startIcon:a.jsx(c,{}),onClick:G,children:"読み込み"}),a.jsx(n.Tooltip,{title:"クイック実行",children:a.jsx(n.IconButton,{color:"secondary",onClick:F,disabled:C||!t.isConnected,children:a.jsx(d.Z,{size:20})})})]})})]}),!t.isConnected&&a.jsx(n.Alert,{severity:"warning",sx:{mt:2},children:"Codexサーバーに接続されていません。コード実行にはサーバー接続が必要です。"})]}),a.jsx(h.Z,{header:(0,a.jsxs)(n.Box,{sx:{display:"flex",alignItems:"center",gap:1},children:[a.jsx(p.Z,{size:20}),a.jsx(n.Typography,{variant:"h6",children:"コードエディタ"}),a.jsx(n.Chip,{label:f.find(e=>e.value===b)?.label,size:"small",color:"primary",variant:"outlined"})]}),children:a.jsx(n.TextField,{fullWidth:!0,multiline:!0,minRows:15,maxRows:25,value:r,onChange:e=>o(e.target.value),placeholder:"ここにコードを入力してください...",sx:{"& .MuiInputBase-root":{fontFamily:"monospace",fontSize:"14px",lineHeight:1.5}}})}),P&&a.jsx(n.Alert,{severity:"error",sx:{mb:2},children:P}),k&&(0,a.jsxs)(h.Z,{header:(0,a.jsxs)(n.Box,{sx:{display:"flex",alignItems:"center",gap:1},children:[a.jsx(x,{size:20}),a.jsx(n.Typography,{variant:"h6",children:"実行結果"}),a.jsx(n.Chip,{label:`終了コード: ${k.exitCode}`,size:"small",color:0===k.exitCode?"success":"error"}),a.jsx(n.Chip,{label:`${k.executionTime}ms`,size:"small",variant:"outlined"})]}),children:[k.stdout&&(0,a.jsxs)(n.Box,{sx:{mb:k.stderr?2:0},children:[a.jsx(n.Typography,{variant:"subtitle2",sx:{mb:1,color:"success.main"},children:"標準出力:"}),a.jsx(n.Paper,{sx:{p:2,backgroundColor:"grey.900",color:"grey.100",fontFamily:"monospace",fontSize:"14px",maxHeight:"300px",overflow:"auto"},children:a.jsx("pre",{style:{margin:0,whiteSpace:"pre-wrap"},children:k.stdout})})]}),k.stderr&&(0,a.jsxs)(n.Box,{children:[a.jsx(n.Typography,{variant:"subtitle2",sx:{mb:1,color:"error.main"},children:"標準エラー出力:"}),a.jsx(n.Paper,{sx:{p:2,backgroundColor:"error.dark",color:"error.contrastText",fontFamily:"monospace",fontSize:"14px",maxHeight:"300px",overflow:"auto"},children:a.jsx("pre",{style:{margin:0,whiteSpace:"pre-wrap"},children:k.stderr})})]})]}),$.length>0&&a.jsx(h.Z,{header:"保存されたファイル",children:a.jsx(n.Box,{sx:{display:"flex",flexWrap:"wrap",gap:1},children:$.map(e=>a.jsx(n.Chip,{label:e,variant:"outlined",onClick:()=>v(e)},e))})})]})})}},55030:(e,t,r)=>{"use strict";r.d(t,{Z:()=>o});var a=r(10326),i=r(17577),n=r(42118);let s=(0,r(28302).E)(n.Card),o=(0,i.forwardRef)(({children:e,animated:t=!1,hover:r=!1,header:i,actions:o,sx:l,...c},d)=>{let p={borderRadius:3,transition:"all 0.2s cubic-bezier(0.4, 0, 0.2, 1)",...r&&{cursor:"pointer","&:hover":{transform:"translateY(-2px)",boxShadow:"0 8px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.1)"}},...l},x=(0,a.jsxs)(a.Fragment,{children:[i&&a.jsx(n.CardHeader,{sx:{pb:1,"& .MuiCardHeader-title":{fontSize:"1.125rem",fontWeight:600},"& .MuiCardHeader-subheader":{fontSize:"0.875rem",color:"text.secondary"}},title:"string"==typeof i?i:void 0,children:"string"!=typeof i&&i}),a.jsx(n.CardContent,{sx:{pb:o?1:2},children:e}),o&&a.jsx(n.CardActions,{sx:{px:2,pt:0,pb:2},children:o})]});return t?a.jsx(s,{ref:d,sx:p,initial:{opacity:0,y:20},animate:{opacity:1,y:0},exit:{opacity:0,y:-20},transition:{type:"spring",stiffness:300,damping:30},...c,children:x}):a.jsx(n.Card,{ref:d,sx:p,...c,children:x})});o.displayName="Card"},35777:(e,t,r)=>{"use strict";r.r(t),r.d(t,{default:()=>a});let a=(0,r(68570).createProxy)(String.raw`C:\Users\downl\Desktop\codex\gui\src\app\code\page.tsx#default`)}};var t=require("../../webpack-runtime.js");t.C(e);var r=e=>t(t.s=e),a=t.X(0,[948,757,471,75,740],()=>r(61699));module.exports=a})();