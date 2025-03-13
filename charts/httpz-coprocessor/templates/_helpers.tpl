{{- define "serverName" -}}
{{- $serverNameDefault := printf "%s-%s" .Release.Name "server" }}
{{- default $serverNameDefault .Values.server.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "listenerName" -}}
{{- $listenerNameDefault := printf "%s-%s" .Release.Name "listener" }}
{{- default $listenerNameDefault .Values.listener.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "txSenderName" -}}
{{- $txSenderNameDefault := printf "%s-%s" .Release.Name "tx-sender" }}
{{- default $txSenderNameDefault .Values.txSender.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "gwListenerName" -}}
{{- $gwListenerNameDefault := printf "%s-%s" .Release.Name "gw-listener" }}
{{- default $gwListenerNameDefault .Values.gwListener.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "zkProofWorkerName" -}}
{{- $zkProofWorkerNameDefault := printf "%s-%s" .Release.Name "zkproof-worker" }}
{{- default $zkProofWorkerNameDefault .Values.zkProofWorker.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "snsWorkerName" -}}
{{- $snsWorkerNameDefault := printf "%s-%s" .Release.Name "sns-worker" }}
{{- default $snsWorkerNameDefault .Values.zkProofWorker.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}