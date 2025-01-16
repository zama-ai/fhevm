{{- define "genesisConfigmapName" -}}
{{- $kmsBlockchainGenesisNameDefault := printf "%s-%s" .Release.Name "genesis" }}
{{- default $kmsBlockchainGenesisNameDefault .Values.kmsBlockchainNetworkSetup.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsBlockchainBootnodeName" -}}
{{- $kmsBlockchainBootnodeNameDefault := printf "%s-%s" .Release.Name "bootnode" }}
{{- default $kmsBlockchainBootnodeNameDefault .Values.kmsBlockchainBootnode.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsBlockchainValidatorName" -}}
{{- $kmsBlockchainValidatorNameDefault := printf "%s-%s" .Release.Name "validator" }}
{{- default $kmsBlockchainValidatorNameDefault .Values.kmsBlockchainValidator.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}


{{- define "kmsBlockchainRpcName" -}}
{{- $kmsBlockchainRpcNameDefault := printf "%s-%s" .Release.Name "rpc" }}
{{- default $kmsBlockchainRpcNameDefault .Values.kmsBlockchainRpc.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsFaucetName" -}}
{{- $kmsFaucetNameDefault := printf "%s-%s" .Release.Name "faucet" }}
{{- default $kmsFaucetNameDefault .Values.kmsFaucet.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsGatewayName" -}}
{{- $kmsGatewayNameDefault := printf "%s-%s" .Release.Name "gateway" }}
{{- default $kmsGatewayNameDefault .Values.kmsGateway.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsKvStoreName" -}}
{{- $kmsKvStoreNameDefault := printf "%s-%s" .Release.Name "kv-store" }}
{{- default $kmsKvStoreNameDefault .Values.kmsKvStore.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "kmsFaucetRpcAddress" -}}
{{- $kmsKvStoreNameDefault :=  printf "http://%s:%d" (include "kmsBlockchainRpcName" .) (.Values.kmsBlockchainRpc.exposedPorts.rpc | int) }}
{{- default $kmsKvStoreNameDefault .Values.kmsFaucet.config.rpcAddress -}}
{{- end -}}

{{- define "kmsBlockchainRPCGenesisConfigmapName" -}}
{{- default (include "genesisConfigmapName" .) .Values.kmsBlockchainRpc.genesisConfigmapName | trunc 63 | trimSuffix "-" -}}
{{- end -}}
