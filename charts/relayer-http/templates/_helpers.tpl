{{- define "relayer-http.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{- define "relayer-http.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{- define "relayer-http.selectorLabels" -}}
app.kubernetes.io/name: {{ include "relayer-http.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "relayer-http.labels" -}}
helm.sh/chart: {{ printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{ include "relayer-http.selectorLabels" . }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/* The default configuration with .Values.config merged over it. */}}
{{- define "relayer-http.config" -}}
{{- $default := .Files.Get "configs/relayer-http-default.yaml" | fromYaml }}
{{- mergeOverwrite $default (deepCopy .Values.config) | toYaml }}
{{- end }}

{{/* The port of http.endpoint ("host:port"), which serves the API and the probes. */}}
{{- define "relayer-http.port" -}}
{{- $config := include "relayer-http.config" . | fromYaml }}
{{- last (splitList ":" $config.http.endpoint) }}
{{- end }}
