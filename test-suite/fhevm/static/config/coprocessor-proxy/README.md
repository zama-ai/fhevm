# coprocessor-proxy test certificate

Self-signed TLS material served by every `coprocessor[N]-proxy` container of the e2e stack. **Test
only**: the private key is checked in on purpose, like the other test secrets of `templates/env`.

The certificate is its own trust anchor (`CA:TRUE`) and its SANs cover the compose hostnames
`coprocessor-proxy` and `coprocessor{1..4}-proxy`, so one pair serves any coprocessor count the
harness supports (`MAX_COPROCESSOR_INSTANCES`). It is valid for 100 years.

Regenerate with:

```sh
SANS="DNS:coprocessor-proxy"
for i in $(seq 1 4); do SANS="$SANS,DNS:coprocessor$i-proxy"; done
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -days 36500 \
  -keyout tls.key -out tls.crt -subj "/CN=coprocessor-proxy" \
  -addext "subjectAltName=$SANS" \
  -addext "basicConstraints=critical,CA:TRUE" \
  -addext "keyUsage=critical,digitalSignature,keyCertSign" \
  -addext "extendedKeyUsage=serverAuth"
chmod 644 tls.key tls.crt  # the container's `fhevm` user must be able to read the key
```
