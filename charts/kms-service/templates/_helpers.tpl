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


{{- define "kmsService.clientPodSpec" -}}
{{- $kmsCoreName := include "kmsCoreName" . }}
{{- $peersIDList := untilStep (default 1 .Values.kmsPeers.id | int) (.Values.kmsPeers.count | add1 | int) 1  }}
spec:
  securityContext:
    {{- toYaml .Values.podSecurityContext | nindent 8 }}
  containers:
  - name: kms-core-client
    image: {{ .Values.kmsCoreClient.image.name }}:{{ .Values.kmsCoreClient.image.tag }}
    env:
      {{ if .Values.minio.enabled }}
      - name: S3_ENDPOINT
        value: "http://minio:9000/{{ .Values.kmsCore.publicVault.s3.bucket }}/{{ .Values.kmsCore.publicVault.s3.path }}"
      {{ else }}
      - name: S3_ENDPOINT
        value: "https://{{ .Values.kmsCore.publicVault.s3.bucket }}.s3.{{ .Values.kmsCore.aws.region }}.amazonaws.com{{ if .Values.kmsCore.publicVault.s3.path }}/{{ .Values.kmsCore.publicVault.s3.path }}{{ end }}"
      {{ end }}
      - name: OBJECT_FOLDER
        value: '[{{ if .Values.kmsCore.thresholdMode.enabled }}{{ range $i, $peer := .Values.kmsCore.thresholdMode.peersList }}{{- if $i -}},{{ end }}"PUB-p{{-  $peer.id  -}}"{{- end }}{{ else }}"PUB"{{ end }}]'
      - name: CORE_ADDRESSES
      {{- if .Values.kmsCore.thresholdMode.peersList }}
        value: '[{{ range $i, $peer := .Values.kmsCore.thresholdMode.peersList }}{{- if $i -}},{{ end }}"http://{{- $peer.host }}:{{- $.Values.kmsCore.ports.client -}}"{{- end }}]'
      {{ else }}
        value: '[{{ range $i := $peersIDList }}{{- if (sub $i 1) -}},{{ end }}"http://{{- printf "%s-%d" $kmsCoreName $i }}:{{- $.Values.kmsCore.ports.client -}}"{{- end }}]'
      {{- end }}
      - name: NUM_MAJORITY
        value: '{{ .Values.kmsCoreClient.num_majority | int }}'
      - name: NUM_RECONSTRUCT
        value: '{{ .Values.kmsCoreClient.num_reconstruct | int }}'
      - name: DECRYPTION_MODE
        value: '{{ .Values.kmsCoreClient.decryption_mode | quote }}'
{{- end -}}
