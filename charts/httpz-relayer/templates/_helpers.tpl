{{- define "relayerName" -}}
{{- $relayerNameDefault := printf "%s-%s" .Release.Name "server" }}
{{- default $relayerNameDefault .Values.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "relayerConfigName" -}}
{{- $relayerConfigNameDefault := printf "%s-%s" .Release.Name "config" }}
{{- default $relayerConfigNameDefault .Values.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}
