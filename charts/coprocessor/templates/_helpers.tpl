{{/*
Optional extra matchLabels so two releases of this chart can coexist in one
namespace (RFC-021 BCS + GCS). Empty by default so existing Deployments keep
their immutable selectors. Preview-env sets e.g. fhevm.zama.ai/fleet=bcs.
*/}}
{{- define "coprocessor.extraSelectorLabels" -}}
{{- range $k, $v := (.Values.commonConfig.extraSelectorLabels | default dict) }}
{{ $k }}: {{ $v | quote }}
{{- end }}
{{- end -}}

{{/*
Appends commonConfig.stackVersion (lowercased, dots as dashes) to a resource
name so blue and green releases each own their copy. The base name is
truncated instead of the suffix to keep names distinct within 63 characters.
Returns the name unchanged when stackVersion is empty.
Usage: include "coprocessor.versionedName" (dict "root" $ "name" "eth-sc-addresses")
*/}}
{{- define "coprocessor.versionedName" -}}
{{- $suffix := .root.Values.commonConfig.stackVersion | default "" | toString | lower | replace "." "-" | trimAll "-" -}}
{{- if $suffix -}}
{{- $base := .name | trunc (int (sub 62 (len $suffix))) | trimSuffix "-" -}}
{{- printf "%s-%s" $base $suffix -}}
{{- else -}}
{{- .name | trunc 63 | trimSuffix "-" -}}
{{- end -}}
{{- end -}}

{{- define "coprocessor.contractConfigMapName" -}}
{{- include "coprocessor.versionedName" (dict "root" . "name" .Values.contractConfigMap.name) -}}
{{- end -}}

{{- define "coprocessor.hostContractConfigMapName" -}}
{{- include "coprocessor.versionedName" (dict "root" . "name" .Values.hostContractConfigMap.name) -}}
{{- end -}}

{{- define "coprocessor.gatewayContractConfigMapName" -}}
{{- include "coprocessor.versionedName" (dict "root" . "name" .Values.gatewayContractConfigMap.name) -}}
{{- end -}}

{{- define "coprocessor.polygonContractConfigMapName" -}}
{{- include "coprocessor.versionedName" (dict "root" . "name" .Values.polygonContractConfigMap.name) -}}
{{- end -}}

{{/*
contractConfigMap data: configs/<commonConfig.network>.yaml overlaid with
contractConfigMap.addresses. Returns YAML; parse with fromYaml.
*/}}
{{- define "coprocessor.contractAddresses" -}}
{{- $cm := .Values.contractConfigMap | default dict -}}
{{- $allowed := list "" "devnet" "testnet" "mainnet" -}}
{{- $network := default "" .Values.commonConfig.network -}}
{{- if not (has $network $allowed) -}}
{{- fail (printf "commonConfig.network must be one of: devnet, testnet, mainnet (or empty); got %q" $network) -}}
{{- end -}}
{{- $preset := dict -}}
{{- if $network -}}
{{- $preset = .Files.Get (printf "configs/%s.yaml" $network) | fromYaml -}}
{{- end -}}
{{- toYaml (mergeOverwrite (deepCopy $preset) ($cm.addresses | default dict)) -}}
{{- end -}}

{{/*
Effective host chains list, returned as YAML under a "chains" key:
  (include "coprocessor.chains" . | fromYaml).chains

When contractConfigMap is enabled and commonConfig.network is set, one entry is generated per
"<prefix>.chain_id" key whose "<prefix>.acl.address" is non-empty (so the
gateway and undeployed chains are skipped). Each entry gets:
  name     <- "<prefix>.name", or the prefix itself
  chainId  <- "<prefix>.chain_id"
  *ContractAddressValueFrom -> configMapKeyRef on the contract ConfigMap for
  every non-empty "<prefix>.{acl,fhevm_executor,kms_generation,protocol_config}.address"
.Values.chains entries with the same name are merged on top (values win);
entries matching no generated chain are appended unchanged.
*/}}
{{- define "coprocessor.chains" -}}
{{- $cm := .Values.contractConfigMap | default dict -}}
{{- $valuesChains := .Values.chains | default list -}}
{{- $generated := list -}}
{{- if and $cm.enabled .Values.commonConfig.network -}}
{{- $addresses := include "coprocessor.contractAddresses" . | fromYaml -}}
{{- $cmName := include "coprocessor.contractConfigMapName" . -}}
{{- $fields := dict "aclContractAddressValueFrom" "acl" "fhevmExecutorContractAddressValueFrom" "fhevm_executor" "kmsGenerationContractAddressValueFrom" "kms_generation" "protocolConfigContractAddressValueFrom" "protocol_config" -}}
{{- range $key, $chainId := $addresses -}}
  {{- if hasSuffix ".chain_id" $key -}}
    {{- $prefix := trimSuffix ".chain_id" $key -}}
    {{- if get $addresses (printf "%s.acl.address" $prefix) -}}
      {{- $chain := dict "name" (get $addresses (printf "%s.name" $prefix) | default $prefix | toString) "chainId" (toString $chainId) -}}
      {{- range $field, $contract := $fields -}}
        {{- $addrKey := printf "%s.%s.address" $prefix $contract -}}
        {{- if get $addresses $addrKey -}}
          {{- $_ := set $chain $field (dict "configMapKeyRef" (dict "name" $cmName "key" $addrKey)) -}}
        {{- end -}}
      {{- end -}}
      {{- $generated = append $generated $chain -}}
    {{- end -}}
  {{- end -}}
{{- end -}}
{{- end -}}
{{- $overrides := dict -}}
{{- range $c := $valuesChains -}}
  {{- $_ := set $overrides (toString $c.name) $c -}}
{{- end -}}
{{- $chains := list -}}
{{- $seen := dict -}}
{{- range $g := $generated -}}
  {{- $chains = append $chains (mergeOverwrite $g (deepCopy (get $overrides $g.name | default dict))) -}}
  {{- $_ := set $seen $g.name true -}}
{{- end -}}
{{- range $c := $valuesChains -}}
  {{- if not (hasKey $seen (toString $c.name)) -}}
    {{- $chains = append $chains $c -}}
  {{- end -}}
{{- end -}}
{{- toYaml (dict "chains" $chains) -}}
{{- end -}}

{{/*
Renders a valueFrom block (or env list) through tpl so values can reference
chart helpers, e.g. name: '{{ include "coprocessor.hostContractConfigMapName" . }}'.
Usage: include "coprocessor.valueFrom" (dict "root" $ "valueFrom" .valueFrom)
*/}}
{{- define "coprocessor.valueFrom" -}}
{{- tpl (toYaml .valueFrom) .root -}}
{{- end -}}

{{- define "upgradeControllerName" -}}
{{- $default := printf "%s-%s" .Release.Name "upgrade-controller" }}
{{- default $default .Values.upgradeController.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "consensusDetectorName" -}}
{{- $default := printf "%s-%s" .Release.Name "consensus-detector" }}
{{- default $default .Values.consensusDetector.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "tfheWorkerName" -}}
{{- $tfheWorkerNameDefault := printf "%s-%s" .Release.Name "tfhe-worker" }}
{{- default $tfheWorkerNameDefault .Values.tfheWorker.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "hostListenerName" -}}
{{- $hostListenerNameDefault := printf "%s-%s" .Release.Name "host-listener" }}
{{- default $hostListenerNameDefault .Values.hostListenerShared.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "hostListenerPollerName" -}}
{{- $hostListenerPollerNameDefault := printf "%s-%s" .Release.Name "host-listener-poller" }}
{{- default $hostListenerPollerNameDefault .Values.hostListenerPollerShared.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "hostListenerCatchupOnlyName" -}}
{{- $hostListenerCatchupOnlyNameDefault := printf "%s-%s" .Release.Name "host-listener-catchup-only" }}
{{- default $hostListenerCatchupOnlyNameDefault .Values.hostListenerCatchupOnlyShared.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "coprocessor.failIfDeprecatedTopLevelListenerKeysPresent" -}}
{{- $deprecatedKeys := list -}}
{{- if hasKey .Values "hostListener" -}}
  {{- $deprecatedKeys = append $deprecatedKeys "hostListener" -}}
{{- end -}}
{{- if hasKey .Values "hostListenerPoller" -}}
  {{- $deprecatedKeys = append $deprecatedKeys "hostListenerPoller" -}}
{{- end -}}
{{- if hasKey .Values "hostListenerCatchupOnly" -}}
  {{- $deprecatedKeys = append $deprecatedKeys "hostListenerCatchupOnly" -}}
{{- end -}}
{{- if gt (len $deprecatedKeys) 0 -}}
{{- fail (printf "deprecated top-level listener keys are no longer supported: %s. Use hostListenerShared / hostListenerPollerShared / hostListenerCatchupOnlyShared plus .Values.chains instead" (join ", " $deprecatedKeys)) -}}
{{- end -}}
{{- end -}}

{{- define "coprocessor.failIfMultipleLegacyNameClaims" -}}
{{- $componentKey := .componentKey -}}
{{- $claims := 0 -}}
{{- range $chain := (include "coprocessor.chains" .root | fromYaml).chains -}}
  {{- if and (hasKey $chain $componentKey) ((index $chain $componentKey).useLegacyName | default false) -}}
    {{- $claims = add1 $claims -}}
  {{- end -}}
{{- end -}}
{{- if gt $claims 1 -}}
{{- fail (printf "only one chains entry may set %s.useLegacyName=true" $componentKey) -}}
{{- end -}}
{{- end -}}

{{- define "txSenderName" -}}
{{- $txSenderNameDefault := printf "%s-%s" .Release.Name "tx-sender" }}
{{- default $txSenderNameDefault .Values.txSender.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "gwListenerName" -}}
{{- $gwListenerNameDefault := printf "%s-%s" .Release.Name "gw-listener" }}
{{- default $gwListenerNameDefault .Values.gwListener.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "zkProofWorkerName" -}}
{{- $zkProofWorkerNameDefault := printf "%s-%s" .Release.Name "zkproof-worker" }}
{{- default $zkProofWorkerNameDefault .Values.zkProofWorker.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "snsWorkerName" -}}
{{- $snsWorkerNameDefault := printf "%s-%s" .Release.Name "sns-worker" }}
{{- default $snsWorkerNameDefault .Values.snsWorker.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}


{{- define "coprocessorDatabaseAuthMode" -}}
{{- $authMode := default "password" .Values.commonConfig.databaseAuthMode -}}
{{- if not (or (eq $authMode "password") (eq $authMode "iam")) -}}
{{- fail (printf "commonConfig.databaseAuthMode must be either \"password\" or \"iam\", got %q" $authMode) -}}
{{- end -}}
{{- $authMode -}}
{{- end -}}

{{/*
RDS CA bundle wiring.

When commonConfig.databaseSslRootCert.enabled is set AND the database auth mode
is "iam", the chart renders configs/global-rds-ca-root.pem into a ConfigMap,
mounts it read-only into every component, and auto-derives
DATABASE_SSL_ROOT_CERT_PATH from the mount location so it can never drift from
where the file actually lands. When mounting is disabled, the env var falls back
to the manual commonConfig.databaseSslRootCertPath field (bring-your-own-mount).
*/}}

{{- define "coprocessor.databaseSslRootCertConfigMapName" -}}
{{- printf "%s-rds-ca-cert" .Release.Name | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{/* Returns "true" only when the chart should mount the bundled CA cert. */}}
{{- define "coprocessor.databaseSslRootCertMountEnabled" -}}
{{- if and (eq (include "coprocessorDatabaseAuthMode" .) "iam") (.Values.commonConfig.databaseSslRootCert).enabled -}}
true
{{- end -}}
{{- end -}}

{{/* Absolute in-pod path to the mounted CA cert (mountPath + fileName). */}}
{{- define "coprocessor.databaseSslRootCertPath" -}}
{{- $cfg := .Values.commonConfig.databaseSslRootCert -}}
{{- printf "%s/%s" (trimSuffix "/" $cfg.mountPath) $cfg.fileName -}}
{{- end -}}

{{/*
Renders the DATABASE_SSL_ROOT_CERT_PATH env entry. Prefers the auto-derived
mount path; otherwise falls back to the manual databaseSslRootCertPath field
(value or valueFrom). Callers must gate on there being something to render and
indent with nindent (12 for deployments, 10 for the dbMigration job).
*/}}
{{- define "coprocessor.databaseSslRootCertEnv" -}}
- name: DATABASE_SSL_ROOT_CERT_PATH
{{- if eq (include "coprocessor.databaseSslRootCertMountEnabled" .) "true" }}
  value: {{ include "coprocessor.databaseSslRootCertPath" . | quote }}
{{- else if (.Values.commonConfig.databaseSslRootCertPath).value }}
  value: {{ .Values.commonConfig.databaseSslRootCertPath.value | quote }}
{{- else if (.Values.commonConfig.databaseSslRootCertPath).valueFrom }}
  valueFrom:
    {{- include "coprocessor.valueFrom" (dict "root" . "valueFrom" .Values.commonConfig.databaseSslRootCertPath.valueFrom) | nindent 4 }}
{{- end }}
{{- end -}}

{{/* Returns "true" when a DATABASE_SSL_ROOT_CERT_PATH env entry should render. */}}
{{- define "coprocessor.databaseSslRootCertEnvEnabled" -}}
{{- if or (eq (include "coprocessor.databaseSslRootCertMountEnabled" .) "true") (.Values.commonConfig.databaseSslRootCertPath) -}}
true
{{- end -}}
{{- end -}}

{{/* Container volumeMounts block for the CA cert. Caller gates on mount-enabled. */}}
{{- define "coprocessor.databaseSslRootCertVolumeMount" -}}
volumeMounts:
  - name: rds-ca-cert
    mountPath: {{ .Values.commonConfig.databaseSslRootCert.mountPath }}
    readOnly: true
{{- end -}}

{{/* Pod-spec volumes block for the CA cert. Caller gates on mount-enabled. */}}
{{- define "coprocessor.databaseSslRootCertVolume" -}}
volumes:
  - name: rds-ca-cert
    configMap:
      name: {{ include "coprocessor.databaseSslRootCertConfigMapName" . }}
{{- end -}}

{{/*
Container resources for FHE workers. CPU/memory keep the previous defaults;
nvidia.com/gpu is passed through when set so pods can land on GPU nodepools.
*/}}
{{- define "coprocessor.containerResources" -}}
{{- $res := default dict . -}}
{{- $req := default dict $res.requests -}}
{{- $lim := default dict $res.limits -}}
resources:
  requests:
    cpu: {{ default "100m" $req.cpu }}
    memory: {{ default "256Mi" $req.memory }}
    {{- with index $req "nvidia.com/gpu" }}
    nvidia.com/gpu: {{ . | quote }}
    {{- end }}
  limits:
    cpu: {{ default "500m" $lim.cpu }}
    memory: {{ default "512Mi" $lim.memory }}
    {{- with index $lim "nvidia.com/gpu" }}
    nvidia.com/gpu: {{ . | quote }}
    {{- end }}
{{- end -}}

