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
  fromYamlArray. Entries come from .Values.scJobs, skipping any with
  enabled: false (default true if omitted).

  Job entry schema:
    enabled:         bool, default true; set to false to skip the entry
    name:            short identifier (slug)
    kind:            deploy | upgrade | task
    image:           { name, tag } container image
    oldImage:        { name, tag } source for old contracts (kind=upgrade)
    env:             list of env entries
    commands:        shell commands run in order
    envFile:         path or glob of address env files to record after the
                     commands, default "addresses/.env.*"; set it to the file
                     the job's image family writes (e.g. addresses/.env.host)
                     so jobs sharing a PVC never re-stamp each other's keys
    configmap:       { name } overrides the global configmap.name for this job
    persistence:     merged over the global persistence block for this job
                     (enabled, mountPath, volumeClaim.name)
*/ -}}
{{- define "scAllJobs" -}}
{{- $jobs := list -}}
{{- range $j := (.Values.scJobs | default list) -}}
{{-   $enabled := true -}}
{{-   if hasKey $j "enabled" -}}{{- $enabled = $j.enabled -}}{{- end -}}
{{-   if $enabled -}}{{- $jobs = append $jobs $j -}}{{- end -}}
{{- end -}}
{{- $jobs | toYaml -}}
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