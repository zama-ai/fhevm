{{- define "gatewayName" -}}
{{- $gatewayNameDefault := printf "%s-%s" .Release.Name "gateway" }}
{{- default $gatewayNameDefault .Values.gateway.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "kvStoreName" -}}
{{- $kvStoreNameDefault := printf "%s-%s" .Release.Name "kv-store" }}
{{- default $kvStoreNameDefault .Values.kvStore.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "kvStoreAddress" -}}
{{ printf "%s:%s" (include "kvStoreName" .) .Values.kvStore.ports.rpc }}
{{- end -}}
