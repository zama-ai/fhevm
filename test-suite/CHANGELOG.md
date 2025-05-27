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

## [v0.7.0] - 2025-05-27

This version is a specific release for the fhevm architecture with tfhe-rs v0.11

## Notes

- Testing in the test-suite for fhevm is based on `v0.7.0` release.
- For anyone wanting to run the local setup to "see the magic happens" please follow instructions in [README.md](README.md).
- Note that some tests may take significantly longer to execute (between 20 and 150 seconds), especially due to input proof verification.

## Some References

|           Name            |  Type      |   Version   |
|:-------------------------:|:----------:|:-----------:|
|         fhevm             | repository |    v0.7.0   |
|         gateway-contracts | docker     |    v0.7.0   |
|         host-contracts    | docker     |    v0.7.0   |
|         coprocessor       | docker     |    v0.7.0   |
|         kms-connector     | docker     |    v0.7.0   |
|         test-suite        | docker     |    v0.7.0   |
|         kms-core          | docker     |    v0.11.0  |
|         relayer           | docker     |    v0.1.0   |
