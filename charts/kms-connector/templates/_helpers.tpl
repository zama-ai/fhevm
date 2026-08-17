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

{{- define "kmsConnectorDbMigrationName" -}}
{{- $kmsConnectorDbMigrationNameDefault := printf "%s-db-migration-%s" .Release.Name .Values.kmsConnectorDbMigration.image.tag }}
{{- default $kmsConnectorDbMigrationNameDefault .Values.kmsConnectorDbMigration.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{/*
Resolve smart contract addresses by loading the configs/contracts-<network>.yaml
preset for commonConfig.network and overriding individual entries with any
commonConfig.*ContractAddresses values that are set. Returns a flat YAML dict;
consume with `fromYaml`.
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
{{- $pol := .Values.commonConfig.polygonContractAddresses | default dict -}}
{{- $bnb := .Values.commonConfig.bnbTestnetContractAddresses | default dict -}}
gatewayChainId: {{ default (index $preset "gateway.chain_id") .Values.commonConfig.gatewayChainId | quote }}
ethereumChainId: {{ default (index $preset "ethereum.chain_id") .Values.commonConfig.ethereumChainId | quote }}
polygonChainId: {{ default (index $preset "polygon.chain_id") .Values.commonConfig.polygonChainId | quote }}
bnbTestnetChainId: {{ default (index $preset "bnb_testnet.chain_id") .Values.commonConfig.bnbTestnetChainId | quote }}
decryption: {{ default (index $preset "gateway.decryption.address") $gw.decryption | quote }}
gatewayConfig: {{ default (index $preset "gateway.gateway_config.address") $gw.gatewayConfig | quote }}
ethereumKmsGeneration: {{ default (index $preset "gateway.kms_generation.address") $eth.kmsGeneration | quote }}
ethereumAcl: {{ default (index $preset "ethereum.acl.address") $eth.acl | quote }}
ethereumProtocolConfig: {{ default (index $preset "ethereum.protocol_config.address") $eth.protocolConfig | quote }}
polygonAcl: {{ default (index $preset "polygon.acl.address") $pol.acl | quote }}
bnbTestnetAcl: {{ default (index $preset "bnb_testnet.acl.address") $bnb.acl | quote }}
{{- end -}}

{{/*
Render the kms-worker host_chains list as JSON with chainId as an integer.
chainId values are resolved by Kubernetes from $(...) env substitution at
runtime, so they are strings at template time; we strip the surrounding quotes
in the JSON so the substituted value is emitted unquoted and deserializes as a
u64. url and aclAddress stay quoted strings.
*/}}
{{- define "kmsConnector.hostChainsJson" -}}
{{- $json := toJson .Values.kmsConnectorKmsWorker.config.hostChains -}}
{{- regexReplaceAll "\"chainId\":\"([^\"]*)\"" $json "\"chainId\":${1}" -}}
{{- end -}}
