#!/usr/bin/env python3
"""SE07/SN01: disposable real HTTPS gateway acceptance; never provisions a host.
Run with NUMBERS_TEST_NGINX=/path/to/nginx python3 tests/gateway.py --directory /new/test/path.
The selected NGINX must include HTTP SSL and Basic authentication modules.
"""
import atexit, argparse, base64, hashlib, http.client, json, os, pathlib, secrets, shutil
import socket, sqlite3, ssl, subprocess, threading, time
from concurrent.futures import ThreadPoolExecutor

ROOT = pathlib.Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser()
parser.add_argument('--directory', required=True)
args = parser.parse_args()
OUT = pathlib.Path(args.directory).resolve()
OUT.mkdir(parents=True, exist_ok=False)
(OUT/'logs').mkdir()
NGINX = os.environ.get('NUMBERS_TEST_NGINX', shutil.which('nginx') or '')
assert NGINX, 'Set NUMBERS_TEST_NGINX to an installed test gateway; no installation is performed here'
SERVER = ROOT / 'target/debug/numbers-prototype-01'
processes, logs = [], []
fault_listener = None
fault_worker = None
fault_done = threading.Event()
checks = []
def record(name):
    checks.append(name)
    print('PASS: ' + name, flush=True)
def port():
    with socket.socket() as s:
        s.bind(('127.0.0.1', 0))
        return s.getsockname()[1]
def run(command, **kw):
    return subprocess.run(command, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, **kw)
def start(command, name):
    log = open(OUT / (name + '.log'), 'wb'); logs.append(log)
    p = subprocess.Popen(command, stdout=log, stderr=log); processes.append(p)
    return p
def wait_port(p, number):
    end = time.monotonic() + 10
    while True:
        assert p.poll() is None, 'Test process stopped; inspect its local log'
        try:
            with socket.create_connection(('127.0.0.1', number), timeout=.2): return
        except OSError:
            assert time.monotonic() < end, 'Test process did not listen'
            time.sleep(.02)
def stop(p):
    if p.poll() is None:
        p.terminate()
        try: p.wait(5)
        except subprocess.TimeoutExpired: p.kill(); p.wait()
backend, gateway = port(), port()
origin = 'https://localhost:' + str(gateway)
credentials = {p: secrets.token_urlsafe(24) for p in ('bidder-a','bidder-b','facilitator')}
lines = []
for principal, secret in credentials.items():
    digest = run(['openssl','passwd','-apr1','-stdin'], input=(secret+'\n').encode()).stdout.decode().strip()
    lines.append(principal+':'+digest)
password_file = OUT / 'credentials'
atexit.register(lambda: password_file.unlink(missing_ok=True))
atexit.register(lambda: (OUT/'key.pem').unlink(missing_ok=True))
password_file.write_text('\n'.join(lines)+'\n'); password_file.chmod(0o600)
run(['openssl','req','-x509','-newkey','rsa:2048','-nodes','-days','1','-subj','/CN=localhost','-addext','subjectAltName=DNS:localhost,IP:127.0.0.1','-keyout',str(OUT/'key.pem'),'-out',str(OUT/'cert.pem')])
(OUT/'key.pem').chmod(0o600)
context = ssl.create_default_context(cafile=str(OUT/'cert.pem'))
# Trust is restricted to this test process; no system/browser trust is changed.
config = json.loads((ROOT/'demo-config.json').read_text())
config.update(database_path='history.sqlite3',port=backend,allowed_origin=origin,session_label='gateway-test')
(OUT/'config.json').write_text(json.dumps(config)+'\n')
template = (ROOT/'ops/nginx.conf.example').read_text()
substitutions = {'RUNTIME':str(OUT),'LISTEN':'127.0.0.1:'+str(gateway),'HOSTNAME':'localhost','CERTIFICATE':str(OUT/'cert.pem'),'CERTIFICATE_KEY':str(OUT/'key.pem'),'CREDENTIAL_FILE':str(password_file),'BACKEND':'127.0.0.1:'+str(backend)}
for key,value in substitutions.items(): template = template.replace('@@'+key+'@@', value)
assert not __import__('re').search(r'@@[A-Z_]+@@', template)
(OUT/'nginx.conf').write_text(template)
run([NGINX,'-p',str(OUT)+'/', '-c',str(OUT/'nginx.conf'),'-t'])
def auth(principal): return 'Basic '+base64.b64encode((principal+':'+credentials[principal]).encode()).decode()
def request(method, path, principal=None, body=None, extra=None):
    headers = {'Origin':origin}
    if principal: headers['Authorization']=auth(principal)
    if body is not None: headers['Content-Type']='application/json'
    headers.update(extra or {})
    conn=http.client.HTTPSConnection('localhost',gateway,context=context,timeout=15)
    try:
        conn.request(method,path,body=body,headers=headers)
        response=conn.getresponse(); data=response.read(); return response.status,data
    finally: conn.close()
def api(method,path,principal,body=None,extra=None):
    status,data=request(method,path,principal,body,extra)
    return status,json.loads(data)
def journal():
    with sqlite3.connect((OUT/'history.sqlite3').as_uri()+'?mode=ro',uri=True) as db:
        return [json.loads(r[0]) for r in db.execute('SELECT record_json FROM events ORDER BY sequence_index')]
def bid(identity,amount): return json.dumps({'auction_number':1,'bidder_id':identity,'amount_rana':amount}).encode()
try:
    app=start([str(SERVER),'--config',str(OUT/'config.json'),'--initialize'],'application');wait_port(app,backend)
    proxy=start([NGINX,'-p',str(OUT)+'/', '-c',str(OUT/'nginx.conf'),'-g','daemon off; master_process off;'],'gateway');wait_port(proxy,gateway)
    assert api('GET','/access','bidder-a')[0] == 200
    before=journal()
    for path in ['/', '/record','/app.js','/style.css','/access','/state','/auction/history']:
        assert request('GET',path)[0]==401
    assert request('GET','/state','bidder-a',extra={'Authorization':'Basic '+base64.b64encode(b'bidder-a:incorrect').decode()})[0]==401
    password_file.write_text('\n'.join(l for l in lines if not l.startswith('bidder-b:'))+'\n')
    assert request('GET','/state','bidder-b')[0]==401
    password_file.write_text('\n'.join(lines)+'\n')
    assert journal()==before
    record('TLS verification, all-resource authentication and credential revocation')
    status,access=api('GET','/access','bidder-a',extra={'X-Numbers-Principal':'facilitator'})
    assert status==200 and access['data']['bidder_id']=='demo-1'
    for path,principal,body in [('/bid','bidder-a',bid('demo-2',5)),('/bid','facilitator',bid('demo-1',5)),('/demo/settlement','bidder-a',b'{"auction_id":"auc_000000000001","outcome":"expired"}')]:
        assert request('POST',path,principal,body)[0]==403
    assert request('POST','/bid','bidder-a',bid('demo-1',5),{'Origin':'https://elsewhere.test'})[0]==403
    assert request('POST','/bid','bidder-a',bid('demo-1',5),{'Origin':''})[0]==403
    assert request('GET','/state','bidder-a',extra={'Sec-Fetch-Site':'cross-site'})[0]==403
    assert request('POST','/bid','bidder-a',bid('demo-1',5),{'Content-Type':'text/plain'})[0]==415
    assert request('POST','/bid','bidder-a',bid('demo-1',5),{'Content-Encoding':'gzip'})[0]==415
    assert request('POST','/bid','bidder-a',b' '*4097)[0]==413
    assert journal()==before
    record('spoofed principal, role, origin, representation and size failures do not reach evaluation')
    # An unfinished >1 KiB body would block the old application loop if forwarded early.
    slow=context.wrap_socket(socket.create_connection(('localhost',gateway)),server_hostname='localhost')
    slow.sendall(('POST /bid HTTP/1.1\r\nHost: localhost\r\nAuthorization: '+auth('bidder-a')+'\r\nOrigin: '+origin+'\r\nContent-Type: application/json\r\nContent-Length: 4096\r\nConnection: close\r\n\r\n').encode()+b'{')
    started=time.monotonic(); assert api('GET','/state','bidder-b')[0]==200; assert time.monotonic()-started<3
    slow.settimeout(13); response=slow.recv(8192); slow.close()
    assert 8 < time.monotonic()-started < 13
    assert not response or b'408' in response.split(b'\r\n',1)[0]
    assert 'route=bid status=408' in (OUT/'access.log').read_text(); assert journal()==before
    record('body buffering keeps slow requests away from writer; ten-second inactivity limit')
    partial=context.wrap_socket(socket.create_connection(('localhost',gateway)),server_hostname='localhost')
    partial.sendall(b'POST /bid HTTP/1.1\r\nHost: localhost\r\n')
    started=time.monotonic(); assert api('GET','/state','bidder-b')[0]==200
    partial.settimeout(13); response=partial.recv(8192); partial.close()
    assert 8 < time.monotonic()-started < 13 and (not response or b'408' in response.split(b'\r\n',1)[0]); assert journal()==before
    record('ten-second header deadline leaves other requests usable')
    body=bid('demo-1',5);body+=b' '*(4096-len(body))
    status,result=api('POST','/bid','bidder-a',body);assert status==200 and result['data']['accepted']
    assert bytes.fromhex(result['data']['bid_record']['payload_json']['command_utf8_hex'])==body
    record('exact 4096-byte body accepted with original command bytes')
    # Separate client contexts see common backend facts; no client-side identity selector.
    with ThreadPoolExecutor(max_workers=2) as pool:
        results=list(pool.map(lambda pair:api('POST','/bid',pair[0],bid(pair[1],6)), [('bidder-a','demo-1'),('bidder-b','demo-2')]))
    assert sum(r[1]['data']['accepted'] for r in results)==1
    states=[api('GET','/state',p)[1]['data'] for p in ['bidder-a','bidder-b']]
    for state in states: state.pop('server_time')
    assert states[0]==states[1]
    record('competing HTTPS commands serialize once and both identities observe the same state')
    # Discard a response only after the upstream has received and committed the command.
    conn=http.client.HTTPSConnection('localhost',gateway,context=context,timeout=15)
    conn.request('POST','/bid',body=bid('demo-1',7),headers={'Authorization':auth('bidder-a'),'Origin':origin,'Content-Type':'application/json'})
    end=time.monotonic()+5
    while not any(r['record_type']=='BidRecord' and r['payload_json'].get('amount_rana')==7 for r in journal()):
        assert time.monotonic()<end;time.sleep(.02)
    conn.close()
    api('GET','/state','bidder-b');assert sum(r['record_type']=='BidRecord' and r['payload_json'].get('amount_rana')==7 for r in journal())==1
    record('lost response after commit leaves exactly one bid group')
    # Stop gateway ingress and writer before backup. Inspection uses a copy only.
    stop(proxy);stop(app)
    saved=OUT/'cold-copy';saved.mkdir();shutil.copy2(OUT/'history.sqlite3',saved/'history.sqlite3');shutil.copy2(OUT/'config.json',saved/'config.json')
    digest=hashlib.sha256((saved/'history.sqlite3').read_bytes()).hexdigest()
    run([str(ROOT/'target/debug/numbers-demo'),'inspect',str(saved)])
    with sqlite3.connect((saved/'history.sqlite3').as_uri()+'?mode=ro',uri=True) as db:assert db.execute('PRAGMA integrity_check').fetchone()[0]=='ok'
    assert hashlib.sha256((saved/'history.sqlite3').read_bytes()).hexdigest()==digest
    record('stopped-writer copy passes integrity and matching read-only reconstruction unchanged')
    # Fault upstream: count requests, close without response. There must be one attempt.
    count=[0];listener=socket.socket();listener.setsockopt(socket.SOL_SOCKET,socket.SO_REUSEADDR,1);listener.bind(('127.0.0.1',backend));listener.listen();listener.settimeout(.25)
    fault_listener=listener;seen_headers=[]
    def failing_upstream():
        while not fault_done.is_set():
            try: connection,_=listener.accept()
            except socket.timeout: continue
            with connection:
                count[0]+=1;connection.settimeout(2);seen_headers.append(connection.recv(8192))
    worker=threading.Thread(target=failing_upstream,daemon=True);fault_worker=worker;worker.start()
    proxy=start([NGINX,'-p',str(OUT)+'/', '-c',str(OUT/'nginx.conf'),'-g','daemon off; master_process off;'],'gateway-fault');wait_port(proxy,gateway)
    for method,path,body in [('POST','/bid',bid('demo-1',8)),('GET','/state',None)]:
        before_count=count[0];assert request(method,path,'bidder-a',body)[0]==502;assert count[0]==before_count+1
    assert all(b'Authorization:' not in h and b'X-Numbers-Principal: bidder-a' in h for h in seen_headers)
    fault_done.set();worker.join(3);listener.close();fault_listener=None
    record('gateway never retries POST or GET and strips external credentials')
    for file in OUT.glob('*.log'):
        text=file.read_text(errors='replace')
        assert all(secret not in text and auth(principal) not in text for principal,secret in credentials.items())
    record('operational logs contain no test credentials or Authorization values')
    (OUT/'result.json').write_text(json.dumps({'passed':checks,'nginx':run([NGINX,'-v']).stderr.decode().strip(),'journal_records':len(journal()),'cold_copy_sha256':digest},indent=2)+'\n')
finally:
    for p in reversed(processes): stop(p)
    fault_done.set()
    if fault_worker: fault_worker.join(3)
    if fault_listener: fault_listener.close()
    for log in logs: log.close()
    # Disposable credentials/keys are not deliverables, even on a failed rehearsal.
    password_file.unlink(missing_ok=True);(OUT/'key.pem').unlink(missing_ok=True)
