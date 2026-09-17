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

{{- define "kmsConnectorEndpointName" -}}
{{- $kmsConnectorEndpointNameDefault := printf "%s-%s" .Release.Name "kms-connector-endpoint" }}
{{- default $kmsConnectorEndpointNameDefault .Values.kmsConnectorEndpoint.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsConnectorProxyName" -}}
{{- $kmsConnectorProxyNameDefault := printf "%s-%s" .Release.Name "kms-connector-proxy" }}
{{- default $kmsConnectorProxyNameDefault .Values.kmsConnectorProxy.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsConnectorDbMigrationName" -}}
{{- $kmsConnectorDbMigrationNameDefault := printf "%s-db-migration-%s" .Release.Name .Values.kmsConnectorDbMigration.image.tag }}
{{- default $kmsConnectorDbMigrationNameDefault .Values.kmsConnectorDbMigration.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{/*
Chain ids and contract addresses: values from configs/contracts-<network>.yaml,
overridden by the matching commonConfig values when set. Host chains are the
commonConfig.hostChains entries with chainId / aclAddress defaulted from the
preset entry named after the chain. Consume with `fromYaml`.
*/}}
{{- define "kmsConnector.contracts" -}}
{{- $allowed := list "" "devnet" "testnet" "mainnet" -}}
{{- $network := default "" .Values.commonConfig.network -}}
{{- if not (has $network $allowed) -}}
{{- fail (printf "commonConfig.network must be one of: devnet, testnet, mainnet (or empty); got %q" $network) -}}
{{- end -}}
{{- $preset := dict -}}
{{- if $network -}}
{{- $preset = .Files.Get (printf "configs/contracts-%s.yaml" $network) | fromYaml -}}
{{- end -}}
{{- $gw := .Values.commonConfig.gatewayContractAddresses | default dict -}}
{{- $eth := .Values.commonConfig.ethereumContractAddresses | default dict -}}
gatewayChainId: {{ default (index $preset "gateway.chain_id") .Values.commonConfig.gatewayChainId | quote }}
decryption: {{ default (index $preset "gateway.decryption.address") $gw.decryption | quote }}
gatewayConfig: {{ default (index $preset "gateway.gateway_config.address") $gw.gatewayConfig | quote }}
ethereumKmsGeneration: {{ default (index $preset "gateway.kms_generation.address") $eth.kmsGeneration | quote }}
ethereumProtocolConfig: {{ default (index $preset "ethereum.protocol_config.address") $eth.protocolConfig | quote }}
hostChains:
{{- range .Values.commonConfig.hostChains }}
{{- $name := required "every commonConfig.hostChains entry needs a `name`" .name }}
  - name: {{ $name | quote }}
    url: {{ required (printf "commonConfig.hostChains[%s].url is required" $name) .url | quote }}
    chainId: {{ .chainId | default (index $preset (printf "%s.chain_id" $name)) | default "" | toString | quote }}
    aclAddress: {{ .aclAddress | default (index $preset (printf "%s.acl.address" $name)) | default "" | quote }}
{{- end }}
{{- end -}}

{{/*
The host chain named "ethereum", used by gw-listener and tx-sender. Consume with `fromYaml`.
*/}}
{{- define "kmsConnector.ethereumHostChain" -}}
{{- $eth := dict -}}
{{- range (include "kmsConnector.contracts" . | fromYaml).hostChains -}}
{{- if eq .name "ethereum" }}{{ $eth = . }}{{ end -}}
{{- end -}}
{{- if not $eth }}{{ fail "commonConfig.hostChains must contain an entry named \"ethereum\"" }}{{ end -}}
{{- toYaml $eth -}}
{{- end -}}

{{/*
kms-worker KMS_CONNECTOR_HOST_CHAINS: JSON list with chainId as an integer.
*/}}
{{- define "kmsConnector.hostChainsJson" -}}
{{- $chains := list -}}
{{- range (include "kmsConnector.contracts" . | fromYaml).hostChains -}}
{{- $chain := dict "url" .url "aclAddress" .aclAddress -}}
{{- if .chainId }}{{ $_ := set $chain "chainId" (atoi .chainId) }}{{ end -}}
{{- $chains = append $chains $chain -}}
{{- end -}}
{{- toJson $chains -}}
{{- end -}}

{{/*
endpoint KMS_CONNECTOR_SUPPORTED_CHAIN_IDS: comma-separated resolved host chain ids.
*/}}
{{- define "kmsConnector.endpointSupportedChainIds" -}}
{{- $ids := list -}}
{{- range (include "kmsConnector.contracts" . | fromYaml).hostChains -}}
{{- if .chainId }}{{ $ids = append $ids .chainId }}{{ end -}}
{{- end -}}
{{- join "," $ids -}}
{{- end -}}
