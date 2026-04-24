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
