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
gatewayChainId: {{ default (index $preset "gateway.chain_id") .Values.commonConfig.gatewayChainId | quote }}
ethereumChainId: {{ default (index $preset "ethereum.chain_id") .Values.commonConfig.ethereumChainId | quote }}
polygonChainId: {{ default (index $preset "polygon.chain_id") .Values.commonConfig.polygonChainId | quote }}
decryption: {{ default (index $preset "gateway.decryption.address") $gw.decryption | quote }}
gatewayConfig: {{ default (index $preset "gateway.gateway_config.address") $gw.gatewayConfig | quote }}
kmsGeneration: {{ default (index $preset "gateway.kms_generation.address") $gw.kmsGeneration | quote }}
ethereumAcl: {{ default (index $preset "ethereum.acl.address") $eth.acl | quote }}
ethereumKmsVerifier: {{ default (index $preset "ethereum.kms_verifier.address") $eth.kmsVerifier | quote }}
polygonAcl: {{ default (index $preset "polygon.acl.address") $pol.acl | quote }}
{{- end -}}
