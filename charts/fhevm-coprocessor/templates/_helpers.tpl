{{- define "serverName" -}}
{{- $server := printf "%s-%s" .Release.Name "server" }}
{{- default $server .Values.server.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "listenerName" -}}
{{- $server := printf "%s-%s" .Release.Name "listener" }}
{{- default $server .Values.server.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}
