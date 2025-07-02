{{- define "scVolumeName" -}}
{{- default .Release.Name .Values.persistence.volumeClaim.name }}
{{- end -}}

{{- define "scDeployJobName" -}}
{{- $scDeployJobNameDefault := printf "%s-%s" .Release.Name "deploy" }}
{{- printf "%s-%s" (default $scDeployJobNameDefault .Values.scDeploy.nameOverride) (.Chart.AppVersion | replace "." "-") | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "scDebugStatefulSetName" -}}
{{- $scDebugStatefulSetNameDefault := printf "%s-%s" .Release.Name "debug" }}
{{- default $scDebugStatefulSetNameDefault .Values.scDebug.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

