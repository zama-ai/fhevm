# kms-connector proxy test certificate

TLS material served by every `kms-connector[-i]-proxy` container of the e2e stack. **Test only**: the private
key is checked in on purpose, like the other test secrets of `templates/env`.

`tls.crt` is signed by the test CA `ca.crt`, which clients trust (`NODE_EXTRA_CA_CERTS` for the e2e tests).
The server certificate is separate from the CA because rustls rejects a CA certificate presented as the server
certificate. The CA key is not kept.

The SANs of `tls.crt` cover the compose hostnames `kms-connector-proxy` and `kms-connector-{2..32}-proxy`,
so one pair serves any KMS party count the harness supports. Both certificates are valid for 100 years.

Regenerate from this directory with (the subshell stops on the first error and discards the CA key on exit):

```sh
(
set -eu
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
SANS="DNS:kms-connector-proxy"
for i in $(seq 2 32); do SANS="$SANS,DNS:kms-connector-$i-proxy"; done
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -days 36500 \
  -keyout "$WORK/ca.key" -out ca.crt -subj "/CN=fhevm e2e test CA" \
  -addext "basicConstraints=critical,CA:TRUE" \
  -addext "keyUsage=critical,keyCertSign,cRLSign"
openssl req -new -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes \
  -keyout tls.key -out "$WORK/tls.csr" -subj "/CN=kms-connector-proxy"
printf 'basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\nextendedKeyUsage=serverAuth\nsubjectAltName=%s\n' \
  "$SANS" > "$WORK/tls.ext"
openssl x509 -req -in "$WORK/tls.csr" -CA ca.crt -CAkey "$WORK/ca.key" -set_serial "0x$(openssl rand -hex 16)" \
  -days 36500 -extfile "$WORK/tls.ext" -out tls.crt
chmod 644 ca.crt tls.key tls.crt  # the container's `fhevm` user must be able to read the key
)
```
