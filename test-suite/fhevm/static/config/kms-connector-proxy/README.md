# kms-connector proxy test certificate

Self-signed TLS material served by every `kms-connector[-i]-proxy` container of the e2e stack
and trusted by the e2e test container through `NODE_EXTRA_CA_CERTS`. **Test only**: the private
key is checked in on purpose, like the other test secrets of `templates/env`.

The certificate is its own trust anchor (`CA:TRUE`) and its SANs cover the compose hostnames
`kms-connector-proxy` and `kms-connector-{2..32}-proxy`, so one pair serves any KMS party count
the harness supports. It is valid for 100 years.

Regenerate with:

```sh
SANS="DNS:kms-connector-proxy"
for i in $(seq 2 32); do SANS="$SANS,DNS:kms-connector-$i-proxy"; done
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -days 36500 \
  -keyout tls.key -out tls.crt -subj "/CN=kms-connector-proxy" \
  -addext "subjectAltName=$SANS" \
  -addext "basicConstraints=critical,CA:TRUE" \
  -addext "keyUsage=critical,digitalSignature,keyCertSign" \
  -addext "extendedKeyUsage=serverAuth"
chmod 644 tls.key tls.crt  # the container's `fhevm` user must be able to read the key
```
