{{/*
Shared building blocks for the coprocessor workloads. Env and args helpers
return YAML sequence items at column 0, each prefixed by a newline, so they
can be concatenated and indented once by the caller.
*/}}

{{/*
Deep merge of .override onto .base, returned as YAML. Maps merge
recursively; any other override value (including false, 0, "" and lists)
replaces the base value; null keeps the base value.
*/}}
{{- define "coprocessor.deepMerge" -}}
{{- $out := deepCopy (.base | default dict) -}}
{{- range $k, $v := (.override | default dict) -}}
  {{- $current := get $out $k -}}
  {{- if and (kindIs "map" $v) (kindIs "map" $current) -}}
    {{- $_ := set $out $k (include "coprocessor.deepMerge" (dict "base" $current "override" $v) | fromYaml) -}}
  {{- else if not (kindIs "invalid" $v) -}}
    {{- $_ := set $out $k $v -}}
  {{- end -}}
{{- end -}}
{{- toYaml $out -}}
{{- end -}}

{{/*
Probe spec without the "enabled" switch. A probe may carry one handler only,
so an exec, tcpSocket or grpc handler drops the httpGet inherited from
defaults.probes.
*/}}
{{- define "coprocessor.probe" -}}
{{- $probe := omit . "enabled" -}}
{{- if or .exec .tcpSocket .grpc -}}
{{- $probe = omit $probe "httpGet" -}}
{{- end -}}
{{- toYaml $probe -}}
{{- end -}}

{{/*
Image reference "<registry>/<repository>:<tag>" from a merged image block
(defaults.image overlaid with the component image). The tag falls back to
the chart appVersion.
Usage: include "coprocessor.image" (dict "root" $ "image" $c.image)
*/}}
{{- define "coprocessor.image" -}}
{{- $repository := required "image.repository is required" .image.repository -}}
{{- $tag := required "image.tag is required (set defaults.image.tag or <component>.image.tag)" (.image.tag | default .root.Chart.AppVersion) -}}
{{- with .image.registry -}}
{{- printf "%s/%s:%s" (trimSuffix "/" .) $repository (toString $tag) -}}
{{- else -}}
{{- printf "%s:%s" $repository (toString $tag) -}}
{{- end -}}
{{- end -}}

{{/*
One env entry from a {value, valueFrom} spec; renders nothing when neither is
set. valueFrom is rendered through tpl (see coprocessor.valueFrom).
Usage: include "coprocessor.envVar" (dict "root" $ "name" "X" "spec" .Values.commonConfig.databaseUser)
*/}}
{{- define "coprocessor.envVar" -}}
{{- $spec := .spec | default dict -}}
{{- if $spec.value }}
- name: {{ .name }}
  value: {{ $spec.value | quote }}
{{- else if $spec.valueFrom }}
- name: {{ .name }}
  valueFrom:
    {{- include "coprocessor.valueFrom" (dict "root" .root "valueFrom" $spec.valueFrom) | nindent 4 }}
{{- end }}
{{- end -}}

{{/* DATABASE_USER, DATABASE_ENDPOINT, DATABASE_PASSWORD (password auth) and DATABASE_URL. */}}
{{- define "coprocessor.env.dbConnection" -}}
{{- $cc := .Values.commonConfig -}}
{{- include "coprocessor.envVar" (dict "root" . "name" "DATABASE_USER" "spec" $cc.databaseUser) }}
{{- include "coprocessor.envVar" (dict "root" . "name" "DATABASE_ENDPOINT" "spec" $cc.databaseEndpoint) }}
{{- if eq (include "coprocessorDatabaseAuthMode" .) "password" }}
{{- include "coprocessor.envVar" (dict "root" . "name" "DATABASE_PASSWORD" "spec" $cc.databasePassword) }}
{{- end }}
{{- with $cc.databaseUrl }}
- name: DATABASE_URL
  value: {{ . | quote }}
{{- end }}
{{- end -}}

{{/* IAM auth env (databaseAuthMode=iam only): flag, region and RDS CA path. */}}
{{- define "coprocessor.env.dbIam" -}}
{{- if eq (include "coprocessorDatabaseAuthMode" .) "iam" }}
- name: DATABASE_IAM_AUTH_ENABLED
  value: "true"
{{- include "coprocessor.envVar" (dict "root" . "name" "DATABASE_IAM_REGION" "spec" .Values.commonConfig.databaseIamRegion) }}
{{- if eq (include "coprocessor.databaseSslRootCertEnvEnabled" .) "true" }}
{{ include "coprocessor.databaseSslRootCertEnv" . }}
{{- end }}
{{- end }}
{{- end -}}

{{- define "coprocessor.env.database" -}}
{{- include "coprocessor.env.dbConnection" . }}
{{- include "coprocessor.env.dbIam" . }}
{{- end -}}

{{- define "coprocessor.env.forceLegacyServerKey" -}}
{{- if .Values.commonConfig.forceLegacyServerKey }}
- name: FORCE_LEGACY_SERVER_KEY
  value: "true"
{{- end }}
{{- end -}}

{{- define "coprocessor.env.tenant" -}}
{{- include "coprocessor.envVar" (dict "root" . "name" "TENANT_API_KEY" "spec" .Values.commonConfig.tenantApiKey) }}
{{- end -}}

{{/*
OTEL env when tracing is enabled for the component (component flag, falling
back to commonConfig.tracing.enabled). OTEL_SERVICE_NAME falls back to the pod
name when the component sets no tracing.service.
Usage: include "coprocessor.env.tracing" (dict "root" $ "c" $componentValues)
*/}}
{{- define "coprocessor.env.tracing" -}}
{{- $common := .root.Values.commonConfig.tracing -}}
{{- $tracing := .c.tracing | default dict -}}
{{- if default $common.enabled $tracing.enabled }}
- name: OTEL_EXPORTER_OTLP_ENDPOINT
  value: {{ $common.endpoint | quote }}
- name: OTEL_SERVICE_NAME
{{- if $tracing.service }}
  value: {{ $tracing.service | quote }}
{{- else }}
  valueFrom:
    fieldRef:
      fieldPath: metadata.name
{{- end }}
{{- end }}
{{- end -}}

{{/*
Host chain env: the RPC URL ("ws" -> ETHEREUM_RPC_URL from wsUrl, "http" ->
ETHEREUM_RPC_HTTP_URL from httpUrl, "" -> none), the chain contract
addresses, and CANONICAL_PROTOCOL_CONFIG_CHAIN_ID.
Usage: include "coprocessor.env.chain" (dict "root" $ "chain" $chain "rpc" "ws")
*/}}
{{- define "coprocessor.env.chain" -}}
{{- $root := .root -}}
{{- $chain := .chain -}}
{{- if eq .rpc "ws" }}
{{- include "coprocessor.envVar" (dict "root" $root "name" "ETHEREUM_RPC_URL" "spec" (dict "value" $chain.wsUrl "valueFrom" $chain.wsUrlValueFrom)) }}
{{- else if eq .rpc "http" }}
{{- include "coprocessor.envVar" (dict "root" $root "name" "ETHEREUM_RPC_HTTP_URL" "spec" (dict "value" $chain.httpUrl "valueFrom" $chain.httpUrlValueFrom)) }}
{{- end }}
{{- include "coprocessor.envVar" (dict "root" $root "name" "ACL_CONTRACT_ADDRESS" "spec" (dict "value" $chain.aclContractAddress "valueFrom" $chain.aclContractAddressValueFrom)) }}
{{- include "coprocessor.envVar" (dict "root" $root "name" "FHEVM_EXECUTOR_CONTRACT_ADDRESS" "spec" (dict "value" $chain.fhevmExecutorContractAddress "valueFrom" $chain.fhevmExecutorContractAddressValueFrom)) }}
{{- include "coprocessor.envVar" (dict "root" $root "name" "KMS_GENERATION_ADDRESS" "spec" (dict "value" $chain.kmsGenerationContractAddress "valueFrom" $chain.kmsGenerationContractAddressValueFrom)) }}
{{- include "coprocessor.envVar" (dict "root" $root "name" "PROTOCOL_CONFIG_ADDRESS" "spec" (dict "value" $chain.protocolConfigContractAddress "valueFrom" $chain.protocolConfigContractAddressValueFrom)) }}
{{- with $root.Values.commonConfig.canonicalProtocolConfigChainId }}
- name: CANONICAL_PROTOCOL_CONFIG_CHAIN_ID
  value: {{ . | quote }}
{{- end }}
{{- end -}}

{{/*
Gateway contract address env for the given fields of
coprocessor.gatewayContractAddresses, in the order given.
Usage: include "coprocessor.env.gateway" (dict "root" $ "fields" (list "inputVerification" "ciphertextCommits"))
*/}}
{{- define "coprocessor.env.gateway" -}}
{{- $root := .root -}}
{{- $addrs := include "coprocessor.gatewayContractAddresses" $root | fromYaml -}}
{{- $names := dict "inputVerification" "INPUT_VERIFICATION_ADDRESS" "ciphertextCommits" "CIPHERTEXT_COMMITS_ADDRESS" "gatewayConfig" "GATEWAY_CONFIG_ADDRESS" "multichainAcl" "MULTICHAIN_ACL_ADDRESS" -}}
{{- range $field := .fields }}
{{- include "coprocessor.envVar" (dict "root" $root "name" (get $names $field) "spec" (get $addrs $field)) }}
{{- end }}
{{- end -}}

{{/* Signer env for a wallet block: AWS_KEY_ID (awsKms) or TX_SENDER_PRIVATE_KEY. */}}
{{- define "coprocessor.env.wallet" -}}
{{- $w := .wallet -}}
{{- if $w.awsKms.enabled }}
- name: AWS_KEY_ID
  valueFrom:
    configMapKeyRef:
      name: {{ $w.awsKms.configmap.name | quote }}
      key: {{ $w.awsKms.configmap.key | quote }}
{{- else if $w.secret.value }}
- name: TX_SENDER_PRIVATE_KEY
  value: {{ $w.secret.value | quote }}
{{- else }}
- name: TX_SENDER_PRIVATE_KEY
  valueFrom:
    secretKeyRef:
      name: {{ $w.secret.name | quote }}
      key: {{ $w.secret.key | quote }}
{{- end }}
{{- end -}}

{{/* Signer args for a wallet block. */}}
{{- define "coprocessor.args.wallet" -}}
{{- if .wallet.awsKms.enabled }}
- --signer-type=aws-kms
{{- else }}
- --signer-type=private-key
- --private-key=$(TX_SENDER_PRIVATE_KEY)
{{- end }}
{{- end -}}

{{/*
commonConfig.env followed by each list in .lists (later entries win), rendered
through tpl.
Usage: include "coprocessor.env.extra" (dict "root" $ "lists" (list $c.env))
*/}}
{{- define "coprocessor.env.extra" -}}
{{- $root := .root -}}
{{- range $envList := prepend (.lists | default list) $root.Values.commonConfig.env }}
{{- with $envList }}
{{ include "coprocessor.valueFrom" (dict "root" $root "valueFrom" .) }}
{{- end }}
{{- end }}
{{- end -}}

{{/*
Drops the "- --<flag>[=...]" items whose flag is listed in .disabled.
Usage: include "coprocessor.filterArgs" (dict "args" $argsYaml "disabled" $c.disabledArgs)
*/}}
{{- define "coprocessor.filterArgs" -}}
{{- $disabled := .disabled | default list -}}
{{- range $line := splitList "\n" .args }}
{{- $item := trim $line }}
{{- if $item }}
{{- if not (has (regexReplaceAll "^- --([^=]+).*$" $item "${1}") $disabled) }}
{{ $item }}
{{- end }}
{{- end }}
{{- end }}
{{- end -}}

{{/*
Container resources. Requests/limits default to 100m/256Mi and 500m/512Mi;
other keys (nvidia.com/gpu, ephemeral-storage, ...) pass through.
*/}}
{{- define "coprocessor.containerResources" -}}
{{- $res := deepCopy (default dict .) -}}
{{- $requests := merge (default dict $res.requests) (dict "cpu" "100m" "memory" "256Mi") -}}
{{- $limits := merge (default dict $res.limits) (dict "cpu" "500m" "memory" "512Mi") -}}
{{- range $section := list $requests $limits -}}
{{- if hasKey $section "nvidia.com/gpu" -}}
{{- $_ := set $section "nvidia.com/gpu" (toString (index $section "nvidia.com/gpu")) -}}
{{- end -}}
{{- end -}}
resources:
  {{- toYaml (merge (dict "requests" $requests "limits" $limits) $res) | nindent 2 }}
{{- end -}}

{{/*
Deployment shared by every long-running component.
  root                 chart root context
  c                    merged component values (defaults overlaid with the
                       component block): image, imagePullSecrets, replicas,
                       ports, resources, probes, updateStrategy, podLabels,
                       scheduling, serviceAccountName
  name                 resource name (also app.kubernetes.io/name)
  appLabel             value of the app label
  container            container name
  command              binary; also run with --stack-version by the startup probe
  args                 built-in args (YAML items), filtered by c.disabledArgs
  extraArgs            list of arg lists appended unfiltered
  env                  env items (YAML)
  extraSelectorLabels  add commonConfig.extraSelectorLabels to the selector
*/}}
{{- define "coprocessor.deployment" -}}
{{- $root := .root -}}
{{- $c := .c -}}
{{- $probe := $root.Values.commonConfig.stackVersionProbe | default dict -}}
{{- $sslMount := eq (include "coprocessor.databaseSslRootCertMountEnabled" $root) "true" -}}
apiVersion: apps/v1
kind: Deployment
metadata:
  labels:
    app: {{ .appLabel }}
    app.kubernetes.io/name: {{ .name }}
  name: {{ .name }}
spec:
  replicas: {{ $c.replicas }}
  selector:
    matchLabels:
      app: {{ .appLabel }}
      {{- if .extraSelectorLabels }}
      {{- include "coprocessor.extraSelectorLabels" $root | nindent 6 }}
      {{- end }}
  {{- with $c.updateStrategy }}
  strategy:
    {{- toYaml . | nindent 4 }}
  {{- end }}
  template:
    metadata:
      labels:
        app: {{ .appLabel }}
        app.kubernetes.io/name: {{ .name }}
        {{- if .extraSelectorLabels }}
        {{- include "coprocessor.extraSelectorLabels" $root | nindent 8 }}
        {{- end }}
        {{- with $root.Values.commonConfig.stackVersion }}
        app.kubernetes.io/version: {{ . | quote }}
        {{- end }}
        {{- with merge (deepCopy ($c.podLabels | default dict)) ($root.Values.podLabels | default dict) }}
        {{- toYaml . | nindent 8 }}
        {{- end }}
      {{- with $root.Values.podAnnotations }}
      annotations:
        {{- toYaml . | nindent 8 }}
      {{- end }}
    spec:
      {{- with $c.imagePullSecrets }}
      imagePullSecrets:
        {{- toYaml . | nindent 8 }}
      {{- end }}
      restartPolicy: Always
      {{- with $c.nodeSelector }}
      nodeSelector:
        {{- toYaml . | nindent 8 }}
      {{- end }}
      {{- with $c.affinity }}
      affinity:
        {{- toYaml . | nindent 8 }}
      {{- end }}
      {{- with $c.tolerations }}
      tolerations:
        {{- toYaml . | nindent 8 }}
      {{- end }}
      {{- with $c.serviceAccountName }}
      serviceAccountName: {{ . }}
      {{- end }}
      containers:
        - name: {{ .container }}
          image: {{ include "coprocessor.image" (dict "root" $root "image" $c.image) }}
          command: [{{ .command | quote }}]
          {{- if $probe.enabled }}
          startupProbe:
            exec:
              command: [{{ .command | quote }}, "--stack-version"]
            periodSeconds: {{ $probe.periodSeconds | default 10 }}
            failureThreshold: {{ $probe.failureThreshold | default 30 }}
          {{- end }}
          args:
            {{- include "coprocessor.filterArgs" (dict "args" .args "disabled" $c.disabledArgs) | nindent 12 }}
            {{- range $extra := .extraArgs }}
            {{- with $extra }}
            {{- toYaml . | nindent 12 }}
            {{- end }}
            {{- end }}
          env:
            {{- .env | nindent 12 }}
          {{- with $c.ports }}
          ports:
            {{- range $portName, $portValue := . }}
            - name: {{ $portName }}
              containerPort: {{ $portValue }}
              protocol: TCP
            {{- end }}
          {{- end }}
          {{- include "coprocessor.containerResources" $c.resources | nindent 10 }}
          {{- $probes := $c.probes | default dict }}
          {{- with $probes.liveness }}
          {{- if .enabled }}
          livenessProbe:
            {{- include "coprocessor.probe" . | nindent 12 }}
          {{- end }}
          {{- end }}
          {{- with $probes.readiness }}
          {{- if .enabled }}
          readinessProbe:
            {{- include "coprocessor.probe" . | nindent 12 }}
          {{- end }}
          {{- end }}
          {{- if $sslMount }}
          {{- include "coprocessor.databaseSslRootCertVolumeMount" $root | nindent 10 }}
          {{- end }}
      {{- if $sslMount }}
      {{- include "coprocessor.databaseSslRootCertVolume" $root | nindent 6 }}
      {{- end }}
{{- end -}}

{{/* Singleton components: values key, resource name suffix, and whether they expose a Service. */}}
{{- define "coprocessor.singletonComponents" -}}
components:
  - {key: gwListener, suffix: gw-listener, service: true}
  - {key: tfheWorker, suffix: tfhe-worker, service: true}
  - {key: zkProofWorker, suffix: zkproof-worker, service: true}
  - {key: snsWorker, suffix: sns-worker, service: true}
  - {key: consensusDetector, suffix: consensus-detector, service: true}
  - {key: txSender, suffix: tx-sender, service: true}
  - {key: upgradeController, suffix: upgrade-controller, service: false}
{{- end -}}

{{/*
Values of a singleton component: .Values.defaults overlaid with
.Values.<key>, as YAML.
Usage: include "coprocessor.componentValues" (dict "root" $ "key" "dbMigration") | fromYaml
*/}}
{{- define "coprocessor.componentValues" -}}
{{- include "coprocessor.deepMerge" (dict "base" .root.Values.defaults "override" (index .root.Values .key)) -}}
{{- end -}}

{{/*
Every enabled workload, returned as YAML under "workloads". Each entry has
key, name, appLabel, c (merged component values), service and
extraSelectorLabels. Host listener entries also carry mode, chain and cfg
(see coprocessor.hostListenerInstances).
*/}}
{{- define "coprocessor.workloads" -}}
{{- $root := . -}}
{{- $workloads := list -}}
{{- range $s := (include "coprocessor.singletonComponents" . | fromYaml).components -}}
  {{- if (index $root.Values $s.key | default dict).enabled -}}
    {{- $c := include "coprocessor.componentValues" (dict "root" $root "key" $s.key) | fromYaml -}}
    {{- $name := include "coprocessor.componentName" (dict "root" $root "key" $s.key "suffix" $s.suffix) -}}
    {{- $workloads = append $workloads (dict "key" $s.key "name" $name "appLabel" (printf "coprocessor-%s" $s.suffix) "c" $c "service" $s.service "extraSelectorLabels" true) -}}
  {{- end -}}
{{- end -}}
{{- range $i := (include "coprocessor.hostListenerInstances" . | fromYaml).instances -}}
  {{- $workloads = append $workloads (merge (dict "key" (printf "hostListener.%s" $i.mode.key) "service" true "extraSelectorLabels" $i.mode.extraSelectorLabels) $i) -}}
{{- end -}}
{{- toYaml (dict "workloads" $workloads) -}}
{{- end -}}

{{/*
The workloads entry of an enabled singleton component, as YAML; empty when
the component is disabled.
Usage: with (include "coprocessor.workload" (dict "root" $ "key" "tfheWorker") | fromYaml)
*/}}
{{- define "coprocessor.workload" -}}
{{- $key := .key -}}
{{- range $w := (include "coprocessor.workloads" .root | fromYaml).workloads -}}
  {{- if eq $w.key $key -}}
    {{- toYaml $w -}}
  {{- end -}}
{{- end -}}
{{- end -}}

{{/*
Host listener modes. Each chains[] entry with hostListener.<key>.enabled
deploys one instance of the mode, configured by hostListener.common overlaid
with hostListener.modes.<key>.
  rpc                  "ws" (wsUrl), "http" (httpUrl) or "" (no RPC)
  metricsArg           pass --metrics-addr
  tenant               pass TENANT_API_KEY / --coprocessor-api-key
  broker               pass BROKER_URL / --broker-url from config.brokerUrl
  chainIdArg           pass --chain-id
  extraSelectorLabels  add commonConfig.extraSelectorLabels to the selector
                       (selectors are immutable, so this is fixed per mode)
*/}}
{{- define "coprocessor.hostListenerModes" -}}
modes:
  - key: listener
    suffix: host-listener
    command: host_listener
    rpc: ws
    metricsArg: false
    tenant: true
    broker: false
    chainIdArg: false
    extraSelectorLabels: false
  - key: poller
    suffix: host-listener-poller
    command: host_listener_poller
    rpc: http
    metricsArg: true
    tenant: true
    broker: false
    chainIdArg: false
    extraSelectorLabels: false
  - key: catchupOnly
    suffix: host-listener-catchup-only
    command: host_listener
    rpc: ws
    metricsArg: false
    tenant: true
    broker: false
    chainIdArg: false
    extraSelectorLabels: false
  - key: consumer
    suffix: host-listener-consumer
    command: host_listener_consumer
    rpc: ""
    metricsArg: true
    tenant: false
    broker: true
    chainIdArg: true
    extraSelectorLabels: true
{{- end -}}

{{/*
Enabled host listener instances, returned as YAML under "instances". Each has
mode, chain, cfg (chains[].hostListener.<mode>), c, name and appLabel.
c is defaults, then hostListener.common, then hostListener.modes.<mode>; its
env, extraArgs and disabledArgs are concatenated across those levels (and
cfg.disabledArgs) instead of replaced. useLegacyName drops the chain suffix
from name and appLabel.
*/}}
{{- define "coprocessor.hostListenerInstances" -}}
{{- $root := . -}}
{{- $hl := .Values.hostListener | default dict -}}
{{- $common := $hl.common | default dict -}}
{{- $base := include "coprocessor.deepMerge" (dict "base" .Values.defaults "override" $common) | fromYaml -}}
{{- $chains := (include "coprocessor.chains" . | fromYaml).chains -}}
{{- $instances := list -}}
{{- range $mode := (include "coprocessor.hostListenerModes" . | fromYaml).modes -}}
  {{- $modeValues := index ($hl.modes | default dict) $mode.key | default dict -}}
  {{- range $chain := $chains -}}
    {{- $cfg := index ($chain.hostListener | default dict) $mode.key | default dict -}}
    {{- if $cfg.enabled -}}
      {{- $c := include "coprocessor.deepMerge" (dict "base" $base "override" $modeValues) | fromYaml -}}
      {{- range $field := list "env" "extraArgs" "disabledArgs" -}}
        {{- $_ := set $c $field (concat (get $common $field | default list) (get $modeValues $field | default list)) -}}
      {{- end -}}
      {{- $_ := set $c "disabledArgs" (concat $c.disabledArgs ($cfg.disabledArgs | default list)) -}}
      {{- $legacy := $cfg.useLegacyName | default false -}}
      {{- $name := ternary (printf "%s-%s" $root.Release.Name $mode.suffix) (printf "%s-%s-%s" $root.Release.Name $mode.suffix $chain.name) $legacy | trunc 63 | trimSuffix "-" -}}
      {{- $appLabel := ternary (printf "coprocessor-%s" $mode.suffix) (printf "coprocessor-%s-%s" $mode.suffix $chain.name) $legacy -}}
      {{- $instances = append $instances (dict "mode" $mode "chain" $chain "cfg" $cfg "c" $c "name" $name "appLabel" $appLabel) -}}
    {{- end -}}
  {{- end -}}
{{- end -}}
{{- toYaml (dict "instances" $instances) -}}
{{- end -}}
