<!--
Guiding Principles:

Changelogs are for humans, not machines.
There should be an entry for every single version.
The same types of changes should be grouped.
Versions and sections should be linkable.
The latest version comes first.
The release date of each version is displayed.
Mention whether you follow Semantic Versioning.

Usage:

Change log entries are to be added to the Unreleased section under the
appropriate stanza (see below). Each entry should ideally include a tag and
the Github issue reference in the following format:

* (<tag>) \#<issue-number> message

The issue numbers will later be link-ified during the release process so you do
not have to worry about including a link manually, but you can if you wish.

Types of changes (Stanzas):

"Features" for new features.
"Improvements" for changes in existing functionality.
"Deprecated" for soon-to-be removed features.
"Bug Fixes" for any bug fixes.
"Client Breaking" for breaking CLI commands and REST routes used by end-users.
"API Breaking" for breaking exported APIs used by developers building on SDK.
"State Machine Breaking" for any changes that result in a different AppState given same genesisState and txList.

Ref: https://keepachangelog.com/en/1.0.0/
-->

# Changelog

## [v0.6.0] - 2024-12-12

This version is a specific release for the coprocessor architecture with tfhe-rs v0.9

## Notes

- Testing in the coprocessor/work_dir for fhevm is based on the fhevm-specific tag v0.6.0-2-test.
- Testing in the e2e folder was initially designed to work with Sepolia (fewer tests and optimized for gas preservation). In this release, the same tests can also be run against a local setup.
- For anyone wanting to run the local setup to "see the magic happen," please use the centralized version of KMS (default). Otherwise, some tests may take significantly longer to execute (between 20 and 150 seconds), especially due to input proof verification.

## Some references

|           Name            |  Type  |   version   |
|:-------------------------:|:------:|:-----------:|
|        KMS images         | docker | v0.9.0-rc37 |
|    fhevm-db-migration     | docker |   v0.1.2    |
|     fhevm-coprocessor     | docker |  v0.6.0-6   |
|     geth-coprocessor      | docker |   v0.1.1    |
| fhevm-smart-contracts-dev | docker |  v0.1.1-1   |

## For e2e test

| Name  |    Type    |    version    |
|:-----:|:----------:|:-------------:|
| fhevm | repository | v0.6.0-2-test |
