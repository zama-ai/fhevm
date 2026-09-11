"""Capture real OTLP/gRPC exports from gateway_otlp_campaign_tests (requires grpcio)."""
import concurrent.futures
import os
from pathlib import Path
import subprocess
import threading
import grpc

out = Path(os.environ.get('GW_CAMPAIGN_OUTPUT', '/tmp/fhevm-validation-locked'))
out.mkdir(parents=True, exist_ok=True)
packets = []
lock = threading.Lock()

def export(request, context):
    with lock:
        packets.append(request)
    return b''  # Empty ExportTraceServiceResponse protobuf.

server = grpc.server(concurrent.futures.ThreadPoolExecutor(max_workers=2))
server.add_generic_rpc_handlers((grpc.method_handlers_generic_handler(
    'opentelemetry.proto.collector.trace.v1.TraceService',
    {'Export': grpc.unary_unary_rpc_method_handler(export,
        request_deserializer=lambda x: x, response_serializer=lambda x: x)}),))
port = server.add_insecure_port('127.0.0.1:0')
server.start()
try:
    env = dict(os.environ, OTEL_EXPORTER_OTLP_ENDPOINT=f'http://127.0.0.1:{port}',
               OTEL_BSP_SCHEDULE_DELAY='100')
    result = subprocess.run([os.environ['GW_OTLP_TEST_EXECUTABLE'], '--ignored', '--nocapture'],
                            env=env, capture_output=True, timeout=60)
    logs = result.stdout + result.stderr
    payload = b''.join(packets)
    (out / 'otlp-json.log').write_bytes(logs)
    (out / 'otlp-export.bin').write_bytes(payload)
    assert result.returncode == 0, 'OTLP test binary failed'
    for label, data in [('JSON', logs), ('OTLP', payload)]:
        assert b'campaign_export_positive_marker' in data, f'{label} positive control absent'
        assert b'SECRET' not in data, f'{label} leaked synthetic credential marker'
    print(f'PASS: JSON and {len(packets)} OTLP exports contain positive control, no credential markers')
finally:
    server.stop(0).wait()
