{{- define "kmsWalletsName" -}}
{{- $kmsWalletsNameDefault := printf "%s-%s" .Release.Name "wallets" }}
{{- default $kmsWalletsNameDefault .Values.kmsWallets.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsSCDeployJobName" -}}
{{- $kmsSCDeployNameDefault := printf "%s-%s-%s" .Release.Name "kms-sc-deploy" .Values.kmsSCDeploy.image.tag}}
{{- default $kmsSCDeployNameDefault .Values.kmsSCDeploy.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}


{{- define "kmsSCDeployConfigmapName" -}}
{{- if .Values.kmsSCDeploy.nameOverride -}}
{{ printf "%s-addresses" .Values.kmsSCDeploy.nameOverride }}
{{- else -}}
{{ printf "%s-%s" .Release.Name "kms-sc-addresses" }}
{{- end -}}
{{- end -}}

{{- define "kmsSCAdminName" -}}
{{- $kmsSCAdminNameDefault := printf "%s-%s" .Release.Name "kms-sc-admin" }}
{{- default $kmsSCAdminNameDefault .Values.kmsSCAdmin.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}
