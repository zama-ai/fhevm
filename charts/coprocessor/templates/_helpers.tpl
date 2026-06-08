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

{{- define "coprocessor.valueOrValueFromEnv" -}}
- name: {{ .name }}
{{- if .source.value }}
  value: {{ .source.value | quote }}
{{- else if .source.valueFrom }}
  valueFrom:
    {{- toYaml .source.valueFrom | nindent 4 }}
{{- end }}
{{- end -}}

{{- define "coprocessor.gatewayTransport" -}}
{{- $component := .component -}}
{{- $mode := default "auto" .mode | toString -}}
{{- if or (eq $mode "http") (eq $mode "ws") -}}
{{- $mode -}}
{{- else if ne $mode "auto" -}}
{{- fail (printf "%s.config.gatewayTransport must be one of: auto, http, ws" $component) -}}
{{- else -}}
{{- $tag := default "" .tag | toString | trimPrefix "v" -}}
{{- if and (regexMatch "^\\d+\\.\\d+\\.\\d+(?:[-+].*)?$" $tag) (semverCompare "<0.13.0-0" $tag) -}}
ws
{{- else -}}
http
{{- end -}}
{{- end -}}
{{- end -}}

{{- define "coprocessor.gatewayWsUrlFromHttp" -}}
{{- $httpUrl := default dict .Values.commonConfig.gatewayHttpUrl -}}
{{- with $httpUrl.value -}}
{{- $value := . | toString -}}
{{- if hasPrefix "https://" $value -}}
{{- printf "wss://%s" (trimPrefix "https://" $value) -}}
{{- else if hasPrefix "http://" $value -}}
{{- printf "ws://%s" (trimPrefix "http://" $value) -}}
{{- end -}}
{{- end -}}
{{- end -}}

{{- define "coprocessor.gatewayUrlEnv" -}}
{{- $root := .root -}}
{{- $transport := include "coprocessor.gatewayTransport" . -}}
{{- if eq $transport "ws" -}}
{{- $wsUrl := default dict $root.Values.commonConfig.gatewayWsUrl -}}
{{- $legacyUrl := default dict $root.Values.commonConfig.gatewayUrl -}}
{{- if or $wsUrl.value $wsUrl.valueFrom -}}
{{- include "coprocessor.valueOrValueFromEnv" (dict "name" "GATEWAY_URL" "source" $wsUrl) -}}
{{- else if or $legacyUrl.value $legacyUrl.valueFrom -}}
{{- include "coprocessor.valueOrValueFromEnv" (dict "name" "GATEWAY_URL" "source" $legacyUrl) -}}
{{- else -}}
{{- $httpUrl := default dict $root.Values.commonConfig.gatewayHttpUrl -}}
{{- $derivedWsUrl := include "coprocessor.gatewayWsUrlFromHttp" $root -}}
{{- if $derivedWsUrl -}}
- name: GATEWAY_URL
  value: {{ $derivedWsUrl | quote }}
{{- else if or $httpUrl.value $httpUrl.valueFrom -}}
{{- fail (printf "%s selected websocket gateway transport, but commonConfig.gatewayHttpUrl cannot be converted to a websocket URL. Set commonConfig.gatewayWsUrl or commonConfig.gatewayUrl, or set %s.config.gatewayTransport=http with an HTTP-capable image." .component .component) -}}
{{- end -}}
{{- end -}}
{{- else -}}
{{- $httpUrl := default dict $root.Values.commonConfig.gatewayHttpUrl -}}
{{- if or $httpUrl.value $httpUrl.valueFrom -}}
{{- include "coprocessor.valueOrValueFromEnv" (dict "name" "GATEWAY_URL" "source" $httpUrl) -}}
{{- else -}}
{{- $wsUrl := default dict $root.Values.commonConfig.gatewayWsUrl -}}
{{- $legacyUrl := default dict $root.Values.commonConfig.gatewayUrl -}}
{{- if or (or $wsUrl.value $wsUrl.valueFrom) (or $legacyUrl.value $legacyUrl.valueFrom) -}}
{{- fail (printf "%s selected HTTP gateway transport, but commonConfig.gatewayHttpUrl is not set. Set commonConfig.gatewayHttpUrl, or set %s.config.gatewayTransport=ws with a websocket-compatible image." .component .component) -}}
{{- end -}}
{{- end -}}
{{- end -}}
{{- end -}}

{{- define "coprocessor.failIfMultipleLegacyNameClaims" -}}
{{- $componentKey := .componentKey -}}
{{- $claims := 0 -}}
{{- range $chain := .root.Values.chains -}}
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
    {{- toYaml .Values.commonConfig.databaseSslRootCertPath.valueFrom | nindent 4 }}
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
