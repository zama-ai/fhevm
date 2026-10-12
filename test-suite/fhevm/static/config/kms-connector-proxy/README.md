# kms-connector proxy test certificate

TLS material served by every `kms-connector[-i]-proxy` container of the e2e stack. **Test only**:
the private key is checked in on purpose, like the other test secrets of `templates/env`.

- `ca.crt`: a self-signed CA, the one trust anchor. The e2e test container trusts it through
  `NODE_EXTRA_CA_CERTS`, and relayer-http through `SSL_CERT_FILE`.
- `tls.crt`, `tls.key`: the server certificate the proxies serve, signed by that CA. Its SANs
  cover the compose hostnames `kms-connector-proxy` and `kms-connector-{2..32}-proxy`, so one pair
  serves any KMS party count the harness supports.

The server certificate is not a CA (`CA:FALSE`): rustls clients such as relayer-http reject a CA
certificate served as a server certificate (`CaUsedAsEndEntity`), even when they trust it. All
three are valid for 100 years. The CA key is not kept: regenerating makes all three files again.

Regenerate with OpenSSL 3 (macOS's default LibreSSL lacks options the commands use):

```sh
SANS="DNS:kms-connector-proxy"
for i in $(seq 2 32); do SANS="$SANS,DNS:kms-connector-$i-proxy"; done
scratch=$(mktemp -d)  # the CA key, the signing request and the extensions; deleted below
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -days 36500 \
  -keyout "$scratch/ca.key" -out ca.crt -subj "/CN=kms-connector-proxy test CA" \
  -addext "basicConstraints=critical,CA:TRUE" \
  -addext "keyUsage=critical,keyCertSign"
openssl req -new -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes \
  -keyout tls.key -subj "/CN=kms-connector-proxy" -out "$scratch/tls.csr"
printf '%s\n' "subjectAltName=$SANS" "basicConstraints=critical,CA:FALSE" \
  "keyUsage=critical,digitalSignature" "extendedKeyUsage=serverAuth" > "$scratch/tls.ext"
openssl x509 -req -in "$scratch/tls.csr" -CA ca.crt -CAkey "$scratch/ca.key" \
  -set_serial "0x$(openssl rand -hex 16)" -days 36500 -extfile "$scratch/tls.ext" -out tls.crt
rm -rf "$scratch"
chmod 644 ca.crt tls.key tls.crt  # the container's `fhevm` user must be able to read the key
```
