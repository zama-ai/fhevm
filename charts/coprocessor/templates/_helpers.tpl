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
{{- range $chain := .root.Values.chains -}}
  {{- if and (hasKey $chain $componentKey) ((index $chain $componentKey).useLegacyName | default false) -}}
    {{- $claims = add1 $claims -}}
  {{- end -}}
{{- end -}}
{{- if gt $claims 1 -}}
{{- fail (printf "only one chains entry may set %s.useLegacyName=true" $componentKey) -}}
{{- end -}}
{{- end -}}

{{- define "coprocessor.renderChainEnvValue" -}}
{{- $name := .name -}}
{{- $value := .value -}}
{{- $valueFrom := .valueFrom -}}
{{- $fieldName := .fieldName -}}
{{- $chainName := .chainName -}}
{{- $hasValue := not (empty $value) -}}
{{- $hasValueFrom := not (empty $valueFrom) -}}
{{- if and $hasValue $hasValueFrom -}}
{{- fail (printf "chains entry %q: only one of %s or %sValueFrom may be set" $chainName $fieldName $fieldName) -}}
{{- end -}}
{{- if not (or $hasValue $hasValueFrom) -}}
{{- fail (printf "chains entry %q: %s or %sValueFrom is required" $chainName $fieldName $fieldName) -}}
{{- end -}}
- name: {{ $name }}
{{- if $hasValue }}
  value: {{ $value | quote }}
{{- else }}
  valueFrom:
{{ toYaml $valueFrom | nindent 4 }}
{{- end }}
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
{{- $authMode := default "password" .Values.commonConfig.database.authMode -}}
{{- if not (or (eq $authMode "password") (eq $authMode "iam")) -}}
{{- fail (printf "commonConfig.database.authMode must be either \"password\" or \"iam\", got %q" $authMode) -}}
{{- end -}}
{{- $authMode -}}
{{- end -}}

{{- define "coprocessorDatabaseEnv" -}}
{{- $authMode := include "coprocessorDatabaseAuthMode" . -}}
- name: DATABASE_URL
  value: {{ .Values.commonConfig.databaseUrl | quote }}
{{- if eq $authMode "iam" }}
- name: DATABASE_IAM_AUTH_ENABLED
  value: "true"
{{- if .Values.commonConfig.database.iam.region }}
- name: DATABASE_IAM_REGION
  value: {{ .Values.commonConfig.database.iam.region | quote }}
{{- end }}
- name: DATABASE_SSL_ROOT_CERT_PATH
  value: {{ required "commonConfig.database.iam.sslRootCertPath is required when authMode=iam" .Values.commonConfig.database.iam.sslRootCertPath | quote }}
{{- end }}
{{- end -}}
