{{- define "kmsConnectorName" -}}
{{- $kmsConnectorNameDefault := printf "%s-%s" .Release.Name "connector" }}
{{- default $kmsConnectorNameDefault .Values.kmsConnector.nameOverride | trunc 52 | trimSuffix "-" -}}
{{- end -}}

