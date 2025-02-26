{{- define "kmsConnectorName" -}}
{{- $kmsConnectorNameDefault := printf "%s-%s" .Release.Name "connector" }}
{{- default $kmsConnectorNameDefault .Values.kmsConnector.nameOverride | trunc 52 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsCoreName" -}}
{{- $kmsCoreNameDefault := printf "%s-%s" .Release.Name "core" }}
{{- default $kmsCoreNameDefault .Values.kmsCore.nameOverride | trunc 52 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsCoreClientName" -}}
{{- $kmsCoreClientNameDefault := printf "%s-%s" .Release.Name "kms-core-client" }}
{{- default $kmsCoreClientNameDefault .Values.kmsCoreClient.nameOverride | trunc 52 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsCoreClientTestingName" -}}
{{- $kmsCoreClientNameDefault := printf "%s-%s" .Release.Name "kms-core-client-testing" }}
{{- default $kmsCoreClientNameDefault .Values.kmsCoreClientTesting.nameOverride | trunc 52 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsCoreAddress" -}}
{{- $kmsCoreAddressDefault := print "" -}}
{{- if .Values.mtls.enabled -}}
{{ printf "http://%s_%s_svc_%d.mesh:80" (include "kmsCoreName" .) .Release.Namespace (int .Values.kmsCore.ports.client) }}
{{- else -}}
{{ printf "http://%s:%d" (include "kmsCoreName" .) (int .Values.kmsCore.ports.client) }}
{{- end -}}
{{ default $kmsCoreAddressDefault .Values.kmsCore.addressOverride }}
{{- end -}}

{{- define "kmsCoreMode" -}}
{{- if .Values.kmsCore.thresholdMode.enabled -}}
threshold
{{- else -}}
centralized
{{- end -}}
{{- end -}}

{{- define "kmsPublicVaultStorage" -}}
{{- if .Values.kmsCore.publicVault.s3.enabled -}}
s3://{{ .Values.kmsCore.publicVault.s3.bucket }}/{{ .Values.kmsCore.publicVault.s3.path }}
{{- else -}}
file:///keys
{{- end -}}
{{- end -}}

{{- define "kmsPrivateVaultStorage" -}}
{{- if .Values.kmsCore.privateVault.s3.enabled -}}
s3://{{ .Values.kmsCore.privateVault.s3.bucket }}/{{ .Values.kmsCore.privateVault.s3.path }}
{{- else -}}
file:///keys
{{- end -}}
{{- end -}}

{{- define "kmsPeersStartID" -}}
{{ default 1 .Values.kmsPeers.id }}
{{- end -}}

{{/* takes a (dict "name" string
     	     	   "image" (dict "name" string "tag" string)
     	     	   "from" string
		           "to" string) */}}
{{- define "socatContainer" -}}
name: {{ .name }}
image: {{ .image.name }}:{{ .image.tag }}
imagePullPolicy: {{ .image.pullPolicy }}
restartPolicy: Always
command:
  - socat
args:
  - {{ .from }}
  - {{ .to }}
{{- end -}}

{{/* takes a (dict "name" string
     	     	   "image" (dict "name" string "tag" string)
                   "vsockPort" int
		           "to" string) */}}
{{- define "proxyFromEnclave" -}}
{{- include "socatContainer"
      (dict "name" .name
            "image" .image
            "from" (printf "VSOCK-LISTEN:%d,fork,reuseaddr" (int .vsockPort))
	        "to" .to) }}
{{- end -}}

{{/* takes a (dict "name" string
                   "image" (dict "name" string "tag" string)
                   "vsockPort" int
		           "address" string
		           "port" int) */}}
{{- define "proxyFromEnclaveTcp" -}}
{{- include "proxyFromEnclave"
      (dict "name" .name
            "image" .image
            "vsockPort" .vsockPort
	        "to" (printf "TCP:%s:%d" .address (int .port))) }}
{{- end -}}

{{/* takes a (dict "name" string
     	     	   "image" (dict "name" string "tag" string)
		           "from" string
		           "cid" int
                   "port" int) */}}
{{- define "proxyToEnclave" -}}
{{- include "socatContainer"
      (dict "name" .name
            "image" .image
            "from" .from
	        "to" (printf "VSOCK-CONNECT:%d:%d" (int .cid) (int .port))) }}
{{- end -}}

{{/* takes a (dict "name" string
     	     	   "image" (dict "name" string "tag" string)
		           "cid" int
                   "port" int) */}}
{{- define "proxyToEnclaveTcp" -}}
{{- include "proxyToEnclave"
      (dict "name" .name
            "image" .image
            "from" (printf "TCP-LISTEN:%d,fork,reuseaddr" (int .port))
            "cid" .cid
	        "port" .port) }}
{{- end -}}


{{- define "kmsThresholdInitJobName" -}}
{{- $kmsCoreNameDefault := printf "%s-%s" .Release.Name "threshold-init" }}
{{- default $kmsCoreNameDefault .Values.kmsCore.nameOverride | trunc 52 | trimSuffix "-" -}}
{{- end -}}
