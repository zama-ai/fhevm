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
overridden by the matching commonConfig values when set. Consume with `fromYaml`.

hostChains is rendered as a map keyed by chain name (ethereum, polygon, ...):
- with a preset, the deployed chains are the preset's `hostChains` entries. Each
  needs a non-empty commonConfig.hostChains.<name>.url; chainId / aclAddress
  default from the preset. A commonConfig.hostChains key missing from the
  preset fails the render.
- without a preset (network: ""), every commonConfig.hostChains entry must set
  url, chainId and aclAddress. A Solana entry (chainKind: solana) sets
  solanaHostProgramId, solanaProofEndpoints and solanaProofApiKey instead of
  aclAddress.
URLs are passed through untouched so they may reference an environment variable
declared in commonConfig.env (e.g. "$(ETHEREUM_RPC_URL)").
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
{{- $presetGw := $preset.gateway | default dict -}}
{{- $presetEth := $preset.ethereum | default dict -}}
{{- $presetChains := $preset.hostChains | default dict -}}
{{- $gw := .Values.commonConfig.gatewayContractAddresses | default dict -}}
{{- $eth := .Values.commonConfig.ethereumContractAddresses | default dict -}}
{{- $chains := .Values.commonConfig.hostChains | default dict -}}
{{- if not (kindIs "map" $chains) -}}
{{- fail "commonConfig.hostChains must be a map keyed by chain name (ethereum, polygon, ...), not a list" -}}
{{- end -}}
gatewayChainId: {{ default $presetGw.chainId .Values.commonConfig.gatewayChainId | toString | quote }}
decryption: {{ default $presetGw.decryption $gw.decryption | quote }}
gatewayConfig: {{ default $presetGw.gatewayConfig $gw.gatewayConfig | quote }}
ethereumKmsGeneration: {{ default $presetGw.kmsGeneration $eth.kmsGeneration | quote }}
ethereumProtocolConfig: {{ default $presetEth.protocolConfig $eth.protocolConfig | quote }}
hostChains:
{{- if $network }}
{{- range $name, $_ := $chains }}
{{- if not (hasKey $presetChains $name) }}
{{- fail (printf "commonConfig.hostChains.%s is not deployed on network %q (known: %s); remove it or set commonConfig.network to \"\" and provide url, chainId and aclAddress" $name $network (keys $presetChains | sortAlpha | join ", ")) }}
{{- end }}
{{- end }}
{{- range $name, $presetChain := $presetChains }}
{{- $chain := index $chains $name | default dict }}
{{- $url := $chain.url | default "" }}
{{- if not $url }}
{{- fail (printf "commonConfig.hostChains.%s.url must be set: %s is deployed on network %q" $name $name $network) }}
{{- end }}
  {{ $name }}:
    url: {{ $url | quote }}
    chainId: {{ $chain.chainId | default $presetChain.chainId | toString | quote }}
    aclAddress: {{ $chain.aclAddress | default $presetChain.aclAddress | quote }}
{{- end }}
{{- else }}
{{- range $name, $chain := $chains }}
{{- $chain = $chain | default dict }}
{{- $solana := eq ($chain.chainKind | default "") "solana" }}
{{- $required := ternary (list "url" "chainId" "solanaHostProgramId" "solanaProofEndpoints" "solanaProofApiKey") (list "url" "chainId" "aclAddress") $solana }}
{{- range $field := $required }}
{{- if not (index $chain $field) }}
{{- fail (printf "commonConfig.hostChains.%s.%s must be set when commonConfig.network is empty (no preset to default from)" $name $field) }}
{{- end }}
{{- end }}
  {{ $name }}:
    url: {{ $chain.url | quote }}
    chainId: {{ $chain.chainId | toString | quote }}
{{- if $solana }}
    chainKind: solana
    solanaHostProgramId: {{ $chain.solanaHostProgramId | quote }}
    solanaProofEndpoints: {{ $chain.solanaProofEndpoints | toJson }}
    solanaProofApiKey: {{ $chain.solanaProofApiKey | quote }}
{{- else }}
    aclAddress: {{ $chain.aclAddress | quote }}
{{- end }}
{{- end }}
{{- end }}
{{- end -}}

{{/*
The host chain named "ethereum". Consume with `fromYaml`.
*/}}
{{- define "kmsConnector.ethereumHostChain" -}}
{{- $eth := index (include "kmsConnector.contracts" . | fromYaml).hostChains "ethereum" -}}
{{- if not $eth }}{{ fail "commonConfig.hostChains must contain an \"ethereum\" entry" }}{{ end -}}
{{- toYaml $eth -}}
{{- end -}}

{{/*
kms-worker KMS_CONNECTOR_HOST_CHAINS: JSON list with chainId as an integer.
*/}}
{{- define "kmsConnector.hostChainsJson" -}}
{{- $chains := list -}}
{{- range $name, $chain := (include "kmsConnector.contracts" . | fromYaml).hostChains -}}
{{- $chains = append $chains (set (deepCopy $chain) "chainId" (atoi $chain.chainId)) -}}
{{- end -}}
{{- toJson $chains -}}
{{- end -}}

{{/*
endpoint KMS_CONNECTOR_SUPPORTED_CHAIN_IDS: comma-separated resolved host chain ids.
*/}}
{{- define "kmsConnector.endpointSupportedChainIds" -}}
{{- $ids := list -}}
{{- range $name, $chain := (include "kmsConnector.contracts" . | fromYaml).hostChains -}}
{{- $ids = append $ids $chain.chainId -}}
{{- end -}}
{{- join "," $ids -}}
{{- end -}}
