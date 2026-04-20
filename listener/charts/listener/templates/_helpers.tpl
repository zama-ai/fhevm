{{/*
Expand the name of the chart.
*/}}
{{- define "listener.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "listener.fullname" -}}
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

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "listener.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels (chart-level).
*/}}
{{- define "listener.labels" -}}
helm.sh/chart: {{ include "listener.chart" . }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
app.kubernetes.io/part-of: {{ include "listener.name" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
{{- end }}

{{/*
Per-listener fully qualified name: <release>-<chart>-<listener.name>
Truncated at 63 chars for DNS compliance.
Usage: {{ include "listener.instanceName" (dict "root" . "listener" $listener) }}
*/}}
{{- define "listener.instanceName" -}}
{{- $base := include "listener.fullname" .root }}
{{- printf "%s-%s" $base .listener.name | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Per-listener labels.
Usage: {{ include "listener.instanceLabels" (dict "root" . "listener" $listener) }}
*/}}
{{- define "listener.instanceLabels" -}}
{{ include "listener.labels" .root }}
app.kubernetes.io/name: {{ include "listener.name" .root }}
app.kubernetes.io/instance: {{ .root.Release.Name }}
app.kubernetes.io/component: listener
listener.zama.ai/chain: {{ .listener.name }}
{{- $chainId := dig "blockchain" "chain_id" "" (.listener.config | default dict) }}
{{- if $chainId }}
listener.zama.ai/chain-id: {{ $chainId | quote }}
{{- end }}
{{- end }}

{{/*
Per-listener selector labels.
Usage: {{ include "listener.instanceSelectorLabels" (dict "root" . "listener" $listener) }}
*/}}
{{- define "listener.instanceSelectorLabels" -}}
app.kubernetes.io/name: {{ include "listener.name" .root }}
app.kubernetes.io/instance: {{ .root.Release.Name }}
app.kubernetes.io/component: listener
listener.zama.ai/chain: {{ .listener.name }}
{{- end }}

{{/*
Create the name of the service account to use.
*/}}
{{- define "listener.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "listener.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Merge root env with per-listener env. Per-listener entries win on name conflict.
Env values support tpl expressions (e.g., {{ .Values.secretName }}).
Usage: {{ include "listener.mergedEnv" (dict "root" . "listener" $listener) }}

Strategy: emit per-listener env first, then root env entries whose name does
NOT appear in the per-listener list. All values are processed through tpl.
*/}}
{{- define "listener.mergedEnv" -}}
{{- $listenerEnv := default (list) .listener.env -}}
{{- $listenerNames := list -}}
{{- range $listenerEnv -}}
  {{- $listenerNames = append $listenerNames .name -}}
{{- end -}}
{{- /* Emit per-listener env (processed through tpl) */ -}}
{{- range $listenerEnv }}
- {{ tpl (toYaml .) $.root | nindent 2 | trim }}
{{- end }}
{{- /* Emit root env entries not overridden by per-listener (processed through tpl) */ -}}
{{- range .root.Values.env }}
{{- if not (has .name $listenerNames) }}
- {{ tpl (toYaml .) $.root | nindent 2 | trim }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Effective metrics port for a listener instance.
Merges the same 3 layers as configmap.yaml and extracts telemetry.metrics_port.
Usage: {{ include "listener.metricsPort" (dict "root" $ "listener" $listener) }}
*/}}
{{- define "listener.metricsPort" -}}
{{- $defaultConfig := .root.Files.Get "configs/listener-default.yaml" | fromYaml }}
{{- $common := .root.Values.commonConfig | default dict }}
{{- $perListener := .listener.config | default dict }}
{{- $merged := mergeOverwrite (deepCopy $defaultConfig) $common $perListener }}
{{- dig "telemetry" "metrics_port" 9090 $merged }}
{{- end }}

{{/*
eRPC fully qualified name.
*/}}
{{- define "listener.erpcName" -}}
{{- printf "%s-erpc" (include "listener.fullname" .) | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
eRPC labels.
*/}}
{{- define "listener.erpcLabels" -}}
{{ include "listener.labels" . }}
app.kubernetes.io/name: {{ include "listener.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/component: erpc
{{- end }}

{{/*
eRPC selector labels.
*/}}
{{- define "listener.erpcSelectorLabels" -}}
app.kubernetes.io/name: {{ include "listener.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/component: erpc
{{- end }}
