import { expect, test } from "bun:test";

test("test keygen adapter rewrites only four RPCs and preserves messages and gRPC errors", async () => {
  const child = Bun.spawn(["node", "-e", `
    const h2=require('node:http2'), assert=require('node:assert/strict');
    const {createProxy,rewritePath}=require(process.argv[1]);
    const prefix='/kms_service.v1.CoreServiceEndpoint/';
    const cases=[['KeyGen','InsecureKeyGen'],['KeyGenPreproc','InsecureKeyGenPreproc'],['GetKeyGenResult','GetInsecureKeyGenResult'],['GetKeyGenPreprocResult','GetInsecureKeyGenPreprocResult'],['PublicDecrypt','PublicDecrypt'],['UserDecrypt','UserDecrypt'],['KeyGenExtra','KeyGenExtra']];
    const frame=Buffer.from([0,0,0,0,3,1,2,255]), signature=Buffer.from([0,0,0,0,4,255,254,253,252]);
    const requests=[],observed=[];
    const upstream=h2.createServer();
    upstream.on('stream',(stream,headers)=>{
      const body=[];stream.on('data',x=>body.push(x));
      stream.on('end',()=>{
        assert.deepEqual(Buffer.concat(body),frame);requests.push(headers[':path']);
        stream.respond({':status':200,'content-type':'application/grpc'},{waitForTrailers:true});
        stream.on('wantTrailers',()=>stream.sendTrailers({'grpc-status':'7','grpc-message':'signed rejection'}));
        stream.end(signature);
      });
    });
    const listen=s=>new Promise(r=>s.listen(0,'127.0.0.1',r));
    (async()=>{
      await listen(upstream);
      const proxy=createProxy('http://127.0.0.1:'+upstream.address().port,e=>observed.push(e));await listen(proxy);
      const session=h2.connect('http://127.0.0.1:'+proxy.address().port);
      try {
        for(const [input,output] of cases){
          assert.equal(rewritePath(prefix+input),prefix+output);
          await new Promise((resolve,reject)=>{
            const request=session.request({':method':'POST',':path':prefix+input,'content-type':'application/grpc','te':'trailers'}),chunks=[];let trailers;
            request.on('data',x=>chunks.push(x));request.on('trailers',x=>trailers=x);request.on('error',reject);
            request.on('end',()=>{try{assert.deepEqual(Buffer.concat(chunks),signature);assert.equal(trailers['grpc-status'],'7');assert.equal(trailers['grpc-message'],'signed rejection');resolve();}catch(e){reject(e)}});
            request.end(frame);
          });
        }
        assert.equal(rewritePath('/other/KeyGen'),'/other/KeyGen');
        assert.deepEqual(requests,cases.map(x=>prefix+x[1]));
        assert.equal(observed.length,cases.length);assert.ok(observed.every(x=>x.grpcStatus==='7'));
      } finally {session.destroy();await new Promise(r=>proxy.close(r));await new Promise(r=>upstream.close(r));}
    })().catch(e=>{console.error(e);process.exitCode=1});
  `, new URL("../../scripts/lib/kms-test-keygen-proxy.cjs", import.meta.url).pathname], { stdout: "pipe", stderr: "pipe", timeout: 10_000 });
  const [status, stderr] = await Promise.all([child.exited, new Response(child.stderr).text()]);
  expect(stderr).toBe("");
  expect(status).toBe(0);
});
