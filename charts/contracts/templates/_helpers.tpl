{{- define "scVolumeName" -}}
{{- default .Release.Name .Values.persistence.volumeClaim.name }}
{{- end -}}

{{- /*
  scSlug: lowercase, DNS-1123-safe fragment. Image tags like "v0.13.0"
  become "v0-13-0", which is appended to Job and ConfigMap key names so
  each (kind, name, tag) combination yields a unique, immutable resource.
*/ -}}
{{- define "scSlug" -}}
{{- regexReplaceAll "[^a-z0-9]+" (lower .) "-" | trimAll "-" -}}
{{- end -}}

{{- /*
  scAllJobs: consolidated Job list emitted as YAML; consumers parse via
  fromYamlArray. Legacy scDeploy / scUpgrade get synthesized into entries
  for back-compat, then any explicit .Values.scJobs entries are appended.

  Job entry schema:
    enabled:         bool, default true; set to false to skip the entry
    name:            short identifier (slug)
    kind:            deploy | upgrade | task
    image:           { name, tag } container image
    oldImage:        { name, tag } source for old contracts (kind=upgrade)
    env:             list of env entries
    commands:        shell commands run in order
    verifyContracts: bool (deploy only)
*/ -}}
{{- define "scAllJobs" -}}
{{- $jobs := list -}}
{{- if .Values.scUpgrade.enabled -}}
{{- $jobs = append $jobs (dict
  "name" "default"
  "kind" "upgrade"
  "image" .Values.scDeploy.image
  "oldImage" .Values.scUpgrade.oldContracts.image
  "env" (.Values.scDeploy.env | default list)
  "commands" (.Values.scUpgrade.upgradeCommands | default list)
) -}}
{{- else if .Values.scDeploy.enabled -}}
{{- $jobs = append $jobs (dict
  "name" "default"
  "kind" "deploy"
  "image" .Values.scDeploy.image
  "env" (.Values.scDeploy.env | default list)
  "commands" (.Values.scDeploy.deployCommands | default list)
  "verifyContracts" (.Values.scDeploy.verifyContracts | default false)
) -}}
{{- end -}}
{{- range (.Values.scJobs | default list) -}}
{{- $jobs = append $jobs . -}}
{{- end -}}
{{- /* Filter out entries with enabled: false (default true if omitted) */ -}}
{{- $filtered := list -}}
{{- range $j := $jobs -}}
{{-   $enabled := true -}}
{{-   if hasKey $j "enabled" -}}{{- $enabled = $j.enabled -}}{{- end -}}
{{-   if $enabled -}}{{- $filtered = append $filtered $j -}}{{- end -}}
{{- end -}}
{{- $filtered | toYaml -}}
{{- end -}}

{{- /* Job resource name: <release>-<kind>-<name>-<tagSlug>, capped at 63 chars */ -}}
{{- define "scJobResourceName" -}}
{{- $tagSlug := include "scSlug" .job.image.tag -}}
{{- printf "%s-%s-%s-%s" .ctx.Release.Name .job.kind .job.name $tagSlug | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- /* ConfigMap key for the per-job runner script */ -}}
{{- define "scJobScriptKey" -}}
{{- printf "run-%s-%s-%s.sh" .job.kind .job.name (include "scSlug" .job.image.tag) -}}
{{- end -}}

{{- define "scDebugStatefulSetName" -}}
{{- $scDebugStatefulSetNameDefault := printf "%s-%s" .Release.Name "debug" }}
{{- default $scDebugStatefulSetNameDefault .Values.scDebug.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}