{{- define "scVolumeName" -}}
{{- default .Release.Name .Values.persistence.volumeClaim.name }}
{{- end -}}

{{- define "scDeployJobName" -}}
{{- $scDeployJobNameDefault := printf "%s-%s" .Release.Name "deploy" }}
{{- if .Values.scDeploy.runOnUpgrade -}}
{{- printf "%s-%d" ((default $scDeployJobNameDefault .Values.scDeploy.nameOverride) | trunc 50 | trimSuffix "-") .Release.Revision -}}
{{- else -}}
{{- printf "%s-%s" (default $scDeployJobNameDefault .Values.scDeploy.nameOverride) (.Chart.AppVersion | replace "." "-") | trunc 63 | trimSuffix "-" -}}
{{- end -}}
{{- end -}}

{{- define "scDebugStatefulSetName" -}}
{{- $scDebugStatefulSetNameDefault := printf "%s-%s" .Release.Name "debug" }}
{{- default $scDebugStatefulSetNameDefault .Values.scDebug.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

