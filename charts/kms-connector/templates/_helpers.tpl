{{- define "kmsConnectorGwListenerName" -}}
{{- $kmsConnectorGwListenerNameDefault := printf "%s-%s" .Release.Name "kms-connector-gw-listener" }}
{{- default $kmsConnectorGwListenerNameDefault .Values.kmsConnectorGwListener.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsConnectorKmsWorkerName" -}}
{{- $kmsConnectorKmsWorkerNameDefault := printf "%s-%s" .Release.Name "kms-connector-kms-worker" }}
{{- default $kmsConnectorKmsWorkerNameDefault .Values.kmsConnectorKmsWorker.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsConnectorTxSenderName" -}}
{{- $kmsConnectorTxSenderNameDefault := printf "%s-%s" .Release.Name "kms-connector-tx-sender" }}
{{- default $kmsConnectorTxSenderNameDefault .Values.kmsConnectorTxSender.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}
