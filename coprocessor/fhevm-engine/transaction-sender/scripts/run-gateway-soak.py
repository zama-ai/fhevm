import os, pathlib, subprocess, json, urllib.request, re, hashlib
ROOT=pathlib.Path(__file__).resolve().parents[4]
OUT=pathlib.Path(os.environ.get('GW_CAMPAIGN_OUTPUT','/tmp/fhevm-validation-locked'))
OUT.mkdir(parents=True,exist_ok=True)
ENVFILE=pathlib.Path(os.environ.get('GW_ENV_FILE',str(pathlib.Path.home()/'.config/fhevm-gw-test.env')))
url=None; addresses=[]
for line in ENVFILE.read_text().splitlines():
    if 'wss://' in line: url=line[line.index('wss://'):].strip().replace('wss://','https://',1)
    if ':' in line:
        k,v=line.split(':',1)
        if k.strip().lower() in ['address','gw_address']: addresses.append(v.strip())
def rpc(m,p):
    try:
        req=urllib.request.Request(url,json.dumps(dict(jsonrpc='2.0',id=1,method=m,params=p)).encode(),{'Content-Type':'application/json','User-Agent':'reqwest'})
        return json.load(urllib.request.urlopen(req,timeout=15))['result']
    except Exception: raise RuntimeError('Gateway preflight failed; raw diagnostic withheld') from None
def snapshot():
    rows=[]
    for i,a in enumerate(addresses):
        latest=int(rpc('eth_getTransactionCount',[a,'latest']),16)
        pending=int(rpc('eth_getTransactionCount',[a,'pending']),16)
        if latest!=pending: raise RuntimeError('Unmined account work: stop campaign')
        rows.append(dict(account=i,latest=latest,pending=pending,balance_wei=int(rpc('eth_getBalance',[a,'latest']),16)))
    return rows
cases=[('http-soak-complete','https',10,2,1,1,20,600)]
with (OUT/'performance-resume-status.log').open('w',buffering=1) as status:
    try:
        executable=os.environ.get('GW_CAMPAIGN_EXECUTABLE')
        if not executable:
            raise RuntimeError('Set GW_CAMPAIGN_EXECUTABLE to the built gw_patch_campaign test binary')
        unit=subprocess.run([executable,'reservation_refunds_only_confirmed_costs','--exact'],capture_output=True)
        if unit.returncode or b'1 passed; 0 failed' not in unit.stdout:
            raise RuntimeError('Reservation accounting test did not pass exactly one test; stale executable?')
        (OUT/'executable-sha256.txt').write_text(hashlib.sha256(pathlib.Path(executable).read_bytes()).hexdigest()+'\n')
        if int(rpc('eth_chainId',[]),16)!=10900: raise RuntimeError('Unexpected chain')
        initial=snapshot(); (OUT/'gateway-initial.json').write_text(json.dumps(initial,indent=2))
        budget=int(os.environ.get('GW_CAMPAIGN_SPEND_WEI','28000000000000000'))
        for name,transport,batch,ops,accounts,offset,warm,measure in cases:
            before=snapshot(); spent=sum(max(0,a['balance_wei']-b['balance_wei']) for a,b in zip(initial,before))
            available=min(budget-spent,30_000_000_000_000_000,min(row['balance_wei']*8//10 for row in before[offset:offset+accounts]))
            if available<=0: raise RuntimeError('Campaign spend budget exhausted')
            env=dict(os.environ,GW_ENV_FILE=str(ENVFILE),GW_RECIPIENT=addresses[offset],GW_TRANSPORT=transport,GW_SWEEP=str(batch),GW_OPS=str(ops),GW_ACCOUNTS=str(accounts),GW_ACCOUNT_OFFSET=str(offset),GW_WARMUP=str(warm),GW_MEASURE=str(measure),GW_MAX_ATTEMPTS='180000',GW_MAX_SPEND_WEI=str(available),GW_MAX_UNRESOLVED='320',GW_MAX_SECS=str(warm+measure+120))
            status.write(f'START {name}, spent so far {spent/1e18:.8f} ETH\n')
            with (OUT/(name+'.log')).open('w') as log:
                proc=subprocess.run([executable,'--ignored','--nocapture'],env=env,stdout=log,stderr=log,timeout=warm+measure+180)
            after=snapshot(); (OUT/(name+'-accounts.json')).write_text(json.dumps(after,indent=2))
            status.write(f'END {name}: exit {proc.returncode}\n')
            if proc.returncode: raise RuntimeError('Performance case failed; stop before next case')
            # A passing load driver can still report RPC failures (useful for
            # WSS comparisons). The HTTP soak has the stricter zero-error gate.
            output=(OUT/(name+'.log')).read_text()
            line=next((line for line in output.splitlines() if 'outcomes:' in line), '')
            counts={key:int(value) for key,value in re.findall(r'(\w+)=(\d+)',line)}
            required={'ok','reverted','send_timeout','nonce_low','nonce_high','already_known','null_response','underpriced','estimate_failed','other'}
            if not required.issubset(counts) or counts.get('ok',0)<=0 or any(value for key,value in counts.items() if key!='ok'):
                raise RuntimeError('HTTP soak had errors, incomplete accounting, or no successful receipts')
            mined=sum(after[i]['latest']-before[i]['latest'] for i in range(offset,offset+accounts))
            if mined!=counts['ok']: raise RuntimeError('Confirmed nonce progress differs from successful receipt count')
        status.write('COMPLETE: all performance cases finished\n')
    except Exception as error:
        status.write('STOPPED: '+str(error)+'\n')
        raise SystemExit(1)
