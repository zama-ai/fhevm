/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export type ApiKey = {
  __typename?: 'ApiKey';
  /** unix timestamp ms */
  createdAt: Scalars['Float']['output'];
  /** DApp ID */
  dappId: Scalars['ID']['output'];
  /** API key description */
  description?: Maybe<Scalars['String']['output']>;
  /** API key ID */
  id: Scalars['ID']['output'];
  /** API key name */
  name: Scalars['String']['output'];
};

export type Chain = {
  __typename?: 'Chain';
  /** Chain description */
  description?: Maybe<Scalars['String']['output']>;
  /** Chain ID */
  id: Scalars['ID']['output'];
  /** Chain name */
  name: Scalars['String']['output'];
};

export type ChangePasswordInput = {
  /** New password */
  newPassword: Scalars['String']['input'];
  /** Current password */
  oldPassword: Scalars['String']['input'];
};

export type CreateApiKey = {
  __typename?: 'CreateApiKey';
  /** API Key details */
  apiKey: ApiKey;
  /** API Key token to use for authentication */
  token: Scalars['String']['output'];
};

export type CreateApiKeyInput = {
  /** DApp ID */
  dappId: Scalars['String']['input'];
  /** API key description */
  description?: InputMaybe<Scalars['String']['input']>;
  /** API key name */
  name: Scalars['String']['input'];
};

export type CreateDappInput = {
  /** Your smart contract address, it should start with 0x and have 42 characters */
  address?: InputMaybe<Scalars['String']['input']>;
  /** Your smart contract chain ID */
  chainId?: InputMaybe<Scalars['Int']['input']>;
  name: Scalars['String']['input'];
  teamId: Scalars['String']['input'];
};

export type CreateInvitationInput = {
  email: Scalars['String']['input'];
  /** You need the secret key to create an invitation ask the #zws team to get one */
  secret: Scalars['String']['input'];
};

export type CumulativeDappStats = {
  __typename?: 'CumulativeDappStats';
  Cast: Scalars['Int']['output'];
  FheAdd: Scalars['Int']['output'];
  FheBitAnd: Scalars['Int']['output'];
  FheBitOr: Scalars['Int']['output'];
  FheBitXor: Scalars['Int']['output'];
  FheDiv: Scalars['Int']['output'];
  FheEq: Scalars['Int']['output'];
  FheEqBytes: Scalars['Int']['output'];
  FheGe: Scalars['Int']['output'];
  FheGt: Scalars['Int']['output'];
  FheIfThenElse: Scalars['Int']['output'];
  FheLe: Scalars['Int']['output'];
  FheLt: Scalars['Int']['output'];
  FheMax: Scalars['Int']['output'];
  FheMin: Scalars['Int']['output'];
  FheMul: Scalars['Int']['output'];
  FheNe: Scalars['Int']['output'];
  FheNeBytes: Scalars['Int']['output'];
  FheNeg: Scalars['Int']['output'];
  FheNot: Scalars['Int']['output'];
  FheRand: Scalars['Int']['output'];
  FheRandBounded: Scalars['Int']['output'];
  FheRem: Scalars['Int']['output'];
  FheRotl: Scalars['Int']['output'];
  FheRotr: Scalars['Int']['output'];
  FheShl: Scalars['Int']['output'];
  FheShr: Scalars['Int']['output'];
  FheSub: Scalars['Int']['output'];
  TrivialEncrypt: Scalars['Int']['output'];
  TrivialEncryptBytes: Scalars['Int']['output'];
  VerifyCiphertext: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type DailyDappStats = {
  __typename?: 'DailyDappStats';
  computation: Scalars['Int']['output'];
  day: Scalars['String']['output'];
  encryption: Scalars['Int']['output'];
  /** The id of the day */
  id: Scalars['ID']['output'];
  total: Scalars['Int']['output'];
};

export type Dapp = {
  __typename?: 'Dapp';
  address?: Maybe<Scalars['String']['output']>;
  apiKeys: Array<ApiKey>;
  chain?: Maybe<Chain>;
  /** Chain ID */
  chainId?: Maybe<Scalars['Int']['output']>;
  /** unix timestamp ms */
  createdAt: Scalars['Float']['output'];
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
  /** DApp usage statistics */
  rawStats: Array<RawStats>;
  /** DApp usage aggregated statistics */
  stats: DappStats;
  status: DappStatus;
  team: Team;
  /** @deprecated Do not use this, it shall go away when I find a way to make it disappear */
  teamId: Scalars['String']['output'];
};

export type DappStats = {
  __typename?: 'DappStats';
  byDay: Array<DailyDappStats>;
  cumulative: CumulativeDappStats;
  /** The id of the dapp */
  id: Scalars['ID']['output'];
};

export enum DappStatus {
  Archived = 'ARCHIVED',
  /** @deprecated Not implmented yet */
  Deleted = 'DELETED',
  /** We are deploying it */
  Deploying = 'DEPLOYING',
  /** Still being worked on */
  Draft = 'DRAFT',
  Failed = 'FAILED',
  /** You can use it now */
  Live = 'LIVE'
}

export type DeleteApiKeyInput = {
  /** API key ID */
  id: Scalars['ID']['input'];
};

export type DeployDAppInput = {
  dappId: Scalars['String']['input'];
};

export type DeployedDAppInput = {
  id: Scalars['ID']['input'];
};

export type Invitation = {
  __typename?: 'Invitation';
  email: Scalars['String']['output'];
  expiresAt: Scalars['Float']['output'];
  id: Scalars['ID']['output'];
  token: Scalars['String']['output'];
};

export type LoginInput = {
  email: Scalars['String']['input'];
  password: Scalars['String']['input'];
};

export type Mutation = {
  __typename?: 'Mutation';
  changePassword: Scalars['Boolean']['output'];
  createApiKey: CreateApiKey;
  createDapp: Dapp;
  createInvitation: Invitation;
  deleteApiKey: Scalars['ID']['output'];
  deployDapp: Dapp;
  login: Auth;
  requestResetPassword: Scalars['Boolean']['output'];
  resetPassword: Auth;
  signup: Auth;
  updateApiKey: ApiKey;
  updateDapp: Dapp;
  updateUser: User;
};


export type MutationChangePasswordArgs = {
  input: ChangePasswordInput;
};


export type MutationCreateApiKeyArgs = {
  input: CreateApiKeyInput;
};


export type MutationCreateDappArgs = {
  input: CreateDappInput;
};


export type MutationCreateInvitationArgs = {
  input: CreateInvitationInput;
};


export type MutationDeleteApiKeyArgs = {
  input: DeleteApiKeyInput;
};


export type MutationDeployDappArgs = {
  input: DeployDAppInput;
};


export type MutationLoginArgs = {
  input: LoginInput;
};


export type MutationRequestResetPasswordArgs = {
  input: RequestResetPasswordInput;
};


export type MutationResetPasswordArgs = {
  input: ResetPasswordInput;
};


export type MutationSignupArgs = {
  input: SignupInput;
};


export type MutationUpdateApiKeyArgs = {
  input: UpdateApiKeyInput;
};


export type MutationUpdateDappArgs = {
  input: UpdateDappInput;
};


export type MutationUpdateUserArgs = {
  input: UpdateUserInput;
};

export type Query = {
  __typename?: 'Query';
  apiKey: ApiKey;
  /** Get chain by ID */
  chain: Chain;
  /** Get all chains */
  chains: Array<Chain>;
  dapp: Dapp;
  invitation: Invitation;
  me: User;
  validateAddress: ValidateAddress;
};


export type QueryApiKeyArgs = {
  input: QueryApiKeyInput;
};


export type QueryChainArgs = {
  input: QueryChainInput;
};


export type QueryDappArgs = {
  input: QueryDappInput;
};


export type QueryInvitationArgs = {
  token: Scalars['String']['input'];
};


export type QueryValidateAddressArgs = {
  input: ValidateAddressInput;
};

export type QueryApiKeyInput = {
  /** API key ID */
  id: Scalars['ID']['input'];
};

export type QueryChainInput = {
  /** Chain ID */
  id: Scalars['ID']['input'];
};

export type QueryDappInput = {
  id: Scalars['ID']['input'];
};

export type RawStats = {
  __typename?: 'RawStats';
  externalRef: Scalars['String']['output'];
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
  timestamp: Scalars['Float']['output'];
};

export type SignupInput = {
  invitationToken: Scalars['String']['input'];
  name: Scalars['String']['input'];
  password: Scalars['String']['input'];
};

export type Subscription = {
  __typename?: 'Subscription';
  dappUpdated: Dapp;
};


export type SubscriptionDappUpdatedArgs = {
  input: DeployedDAppInput;
};

export type Team = {
  __typename?: 'Team';
  dapps: Array<Dapp>;
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
};

export type UpdateApiKeyInput = {
  /** API key description */
  description?: InputMaybe<Scalars['String']['input']>;
  /** API key ID */
  id: Scalars['ID']['input'];
  /** API key name */
  name?: InputMaybe<Scalars['String']['input']>;
};

export type UpdateDappInput = {
  /** Your smart contract address, it should start with 0x and have 42 characters */
  address?: InputMaybe<Scalars['String']['input']>;
  /** Your smart contract chain ID */
  chainId?: InputMaybe<Scalars['Int']['input']>;
  id: Scalars['ID']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
};

export type UpdateUserInput = {
  id: Scalars['ID']['input'];
  name: Scalars['String']['input'];
};

export type User = {
  __typename?: 'User';
  email: Scalars['String']['output'];
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
  /** User teams */
  teams: Array<Team>;
};

export type ValidateAddress = {
  __typename?: 'ValidateAddress';
  check: Scalars['Boolean']['output'];
  message?: Maybe<Scalars['String']['output']>;
};

export type ValidateAddressInput = {
  address: Scalars['String']['input'];
  /** 1 for eth mainnet, 11155111 for sepolia, etc */
  chainId: Scalars['Int']['input'];
};

export type Auth = {
  __typename?: 'auth';
  token: Scalars['String']['output'];
  user: User;
};

/** Request reset password input */
export type RequestResetPasswordInput = {
  /** The user's email address to reset the password */
  email: Scalars['String']['input'];
};

export type ResetPasswordInput = {
  /** The new password */
  password: Scalars['String']['input'];
  /** The token received by email */
  token: Scalars['String']['input'];
};

export type CreateApiKeyMutationVariables = Exact<{
  dappId: Scalars['String']['input'];
  name: Scalars['String']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
}>;


export type CreateApiKeyMutation = { __typename?: 'Mutation', createApiKey: { __typename?: 'CreateApiKey', token: string, apiKey: { __typename?: 'ApiKey', id: string, dappId: string, name: string, description?: string | null } } };

export type ListChainsQueryVariables = Exact<{ [key: string]: never; }>;


export type ListChainsQuery = { __typename?: 'Query', chains: Array<{ __typename?: 'Chain', id: string, name: string, description?: string | null }> };

export type UpdateDappMutationVariables = Exact<{
  dappId: Scalars['ID']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  address?: InputMaybe<Scalars['String']['input']>;
}>;


export type UpdateDappMutation = { __typename?: 'Mutation', updateDapp: { __typename?: 'Dapp', id: string, name: string, status: DappStatus, address?: string | null, team: { __typename?: 'Team', id: string, name: string } } };

export type ListApiKeysQueryVariables = Exact<{
  dappId: Scalars['ID']['input'];
}>;


export type ListApiKeysQuery = { __typename?: 'Query', dapp: { __typename?: 'Dapp', apiKeys: Array<{ __typename?: 'ApiKey', id: string, name: string, description?: string | null, createdAt: number }> } };

export type DeleteApiKeyMutationVariables = Exact<{
  apiKeyId: Scalars['ID']['input'];
}>;


export type DeleteApiKeyMutation = { __typename?: 'Mutation', deleteApiKey: string };

export type RequestResetPasswordMutationVariables = Exact<{
  email: Scalars['String']['input'];
}>;


export type RequestResetPasswordMutation = { __typename?: 'Mutation', requestResetPassword: boolean };

export type ResetPasswordMutationVariables = Exact<{
  token: Scalars['String']['input'];
  password: Scalars['String']['input'];
}>;


export type ResetPasswordMutation = { __typename?: 'Mutation', resetPassword: { __typename?: 'auth', token: string } };

export type ChangePasswordMutationVariables = Exact<{
  oldPassword: Scalars['String']['input'];
  newPassword: Scalars['String']['input'];
}>;


export type ChangePasswordMutation = { __typename?: 'Mutation', changePassword: boolean };

export type PreferencesQueryVariables = Exact<{ [key: string]: never; }>;


export type PreferencesQuery = { __typename?: 'Query', me: { __typename?: 'User', id: string, email: string, name: string, teams: Array<{ __typename?: 'Team', id: string, name: string }> } };

export type ChangeUserNameMutationVariables = Exact<{
  id: Scalars['ID']['input'];
  name: Scalars['String']['input'];
}>;


export type ChangeUserNameMutation = { __typename?: 'Mutation', updateUser: { __typename?: 'User', id: string, name: string } };

export type ValidateAddressQueryVariables = Exact<{
  chainId: Scalars['Int']['input'];
  address: Scalars['String']['input'];
}>;


export type ValidateAddressQuery = { __typename?: 'Query', validateAddress: { __typename?: 'ValidateAddress', check: boolean, message?: string | null } };

export type CreateDappMutationVariables = Exact<{
  teamId: Scalars['String']['input'];
  name: Scalars['String']['input'];
  chainId: Scalars['Int']['input'];
  address: Scalars['String']['input'];
}>;


export type CreateDappMutation = { __typename?: 'Mutation', createDapp: { __typename?: 'Dapp', id: string, name: string, chainId?: number | null, address?: string | null, status: DappStatus } };

export type GetDappDetailsQueryVariables = Exact<{
  dappId: Scalars['ID']['input'];
}>;


export type GetDappDetailsQuery = { __typename?: 'Query', dapp: { __typename?: 'Dapp', id: string, name: string, status: DappStatus, rawStats: Array<{ __typename?: 'RawStats', id: string, name: string, timestamp: number, externalRef: string }>, stats: { __typename?: 'DappStats', id: string, cumulative: { __typename?: 'CumulativeDappStats', total: number, FheAdd: number, FheSub: number, FheMul: number, FheDiv: number, FheRem: number, FheBitAnd: number, FheBitOr: number, FheBitXor: number, FheShl: number, FheShr: number, FheRotl: number, FheRotr: number, FheEq: number, FheEqBytes: number, FheNe: number, FheNeBytes: number, FheGe: number, FheGt: number, FheLe: number, FheLt: number, FheMin: number, FheMax: number, FheNeg: number, FheNot: number, VerifyCiphertext: number, Cast: number, TrivialEncrypt: number, TrivialEncryptBytes: number, FheIfThenElse: number, FheRand: number, FheRandBounded: number }, byDay: Array<{ __typename?: 'DailyDappStats', id: string, day: string, total: number, computation: number, encryption: number }> } } };

export type DappUpdatedSubscriptionVariables = Exact<{
  dappId: Scalars['ID']['input'];
}>;


export type DappUpdatedSubscription = { __typename?: 'Subscription', dappUpdated: { __typename?: 'Dapp', id: string, name: string, status: DappStatus, rawStats: Array<{ __typename?: 'RawStats', id: string, name: string, timestamp: number, externalRef: string }>, stats: { __typename?: 'DappStats', id: string, cumulative: { __typename?: 'CumulativeDappStats', total: number, FheAdd: number, FheSub: number, FheMul: number, FheDiv: number, FheRem: number, FheBitAnd: number, FheBitOr: number, FheBitXor: number, FheShl: number, FheShr: number, FheRotl: number, FheRotr: number, FheEq: number, FheEqBytes: number, FheNe: number, FheNeBytes: number, FheGe: number, FheGt: number, FheLe: number, FheLt: number, FheMin: number, FheMax: number, FheNeg: number, FheNot: number, VerifyCiphertext: number, Cast: number, TrivialEncrypt: number, TrivialEncryptBytes: number, FheIfThenElse: number, FheRand: number, FheRandBounded: number }, byDay: Array<{ __typename?: 'DailyDappStats', id: string, day: string, total: number, computation: number, encryption: number }> } } };

export type SignInMutationVariables = Exact<{
  email: Scalars['String']['input'];
  password: Scalars['String']['input'];
}>;


export type SignInMutation = { __typename?: 'Mutation', login: { __typename?: 'auth', token: string, user: { __typename?: 'User', id: string, email: string, name: string } } };

export type InvitationTokenQueryVariables = Exact<{
  token: Scalars['String']['input'];
}>;


export type InvitationTokenQuery = { __typename?: 'Query', invitation: { __typename?: 'Invitation', id: string, expiresAt: number, token: string, email: string } };

export type SignUpMutationVariables = Exact<{
  name: Scalars['String']['input'];
  password: Scalars['String']['input'];
  invitationToken: Scalars['String']['input'];
}>;


export type SignUpMutation = { __typename?: 'Mutation', signup: { __typename?: 'auth', token: string, user: { __typename?: 'User', id: string, email: string, name: string } } };

export type MeQueryVariables = Exact<{ [key: string]: never; }>;


export type MeQuery = { __typename?: 'Query', me: { __typename?: 'User', id: string, email: string, name: string, teams: Array<{ __typename?: 'Team', id: string, name: string, dapps: Array<{ __typename?: 'Dapp', id: string, name: string, status: DappStatus, createdAt: number }> }> } };


export const CreateApiKeyDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"CreateApiKey"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"dappId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"name"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"description"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createApiKey"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"dappId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"dappId"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"name"},"value":{"kind":"Variable","name":{"kind":"Name","value":"name"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"description"},"value":{"kind":"Variable","name":{"kind":"Name","value":"description"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"token"}},{"kind":"Field","name":{"kind":"Name","value":"apiKey"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"dappId"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"description"}}]}}]}}]}}]} as unknown as DocumentNode<CreateApiKeyMutation, CreateApiKeyMutationVariables>;
export const ListChainsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"ListChains"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"chains"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"description"}}]}}]}}]} as unknown as DocumentNode<ListChainsQuery, ListChainsQueryVariables>;
export const UpdateDappDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"UpdateDapp"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"dappId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"name"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"address"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateDapp"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"dappId"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"name"},"value":{"kind":"Variable","name":{"kind":"Name","value":"name"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"address"},"value":{"kind":"Variable","name":{"kind":"Name","value":"address"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"status"}},{"kind":"Field","name":{"kind":"Name","value":"address"}},{"kind":"Field","name":{"kind":"Name","value":"team"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]}}]} as unknown as DocumentNode<UpdateDappMutation, UpdateDappMutationVariables>;
export const ListApiKeysDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"ListApiKeys"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"dappId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"dapp"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"dappId"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"apiKeys"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}}]}}]}}]}}]} as unknown as DocumentNode<ListApiKeysQuery, ListApiKeysQueryVariables>;
export const DeleteApiKeyDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"DeleteApiKey"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"apiKeyId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"deleteApiKey"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"apiKeyId"}}}]}}]}]}}]} as unknown as DocumentNode<DeleteApiKeyMutation, DeleteApiKeyMutationVariables>;
export const RequestResetPasswordDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"RequestResetPassword"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"email"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"requestResetPassword"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"email"},"value":{"kind":"Variable","name":{"kind":"Name","value":"email"}}}]}}]}]}}]} as unknown as DocumentNode<RequestResetPasswordMutation, RequestResetPasswordMutationVariables>;
export const ResetPasswordDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"ResetPassword"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"token"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"password"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"resetPassword"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"token"},"value":{"kind":"Variable","name":{"kind":"Name","value":"token"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"password"},"value":{"kind":"Variable","name":{"kind":"Name","value":"password"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"token"}}]}}]}}]} as unknown as DocumentNode<ResetPasswordMutation, ResetPasswordMutationVariables>;
export const ChangePasswordDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"ChangePassword"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"oldPassword"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"newPassword"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"changePassword"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"oldPassword"},"value":{"kind":"Variable","name":{"kind":"Name","value":"oldPassword"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"newPassword"},"value":{"kind":"Variable","name":{"kind":"Name","value":"newPassword"}}}]}}]}]}}]} as unknown as DocumentNode<ChangePasswordMutation, ChangePasswordMutationVariables>;
export const PreferencesDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"Preferences"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"me"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"email"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"teams"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]}}]} as unknown as DocumentNode<PreferencesQuery, PreferencesQueryVariables>;
export const ChangeUserNameDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"ChangeUserName"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"id"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"name"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateUser"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"id"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"name"},"value":{"kind":"Variable","name":{"kind":"Name","value":"name"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]} as unknown as DocumentNode<ChangeUserNameMutation, ChangeUserNameMutationVariables>;
export const ValidateAddressDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"ValidateAddress"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"chainId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"address"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"validateAddress"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"chainId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"chainId"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"address"},"value":{"kind":"Variable","name":{"kind":"Name","value":"address"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"check"}},{"kind":"Field","name":{"kind":"Name","value":"message"}}]}}]}}]} as unknown as DocumentNode<ValidateAddressQuery, ValidateAddressQueryVariables>;
export const CreateDappDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"CreateDapp"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"teamId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"name"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"chainId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"address"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createDapp"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"teamId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"teamId"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"name"},"value":{"kind":"Variable","name":{"kind":"Name","value":"name"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"chainId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"chainId"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"address"},"value":{"kind":"Variable","name":{"kind":"Name","value":"address"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"chainId"}},{"kind":"Field","name":{"kind":"Name","value":"address"}},{"kind":"Field","name":{"kind":"Name","value":"status"}}]}}]}}]} as unknown as DocumentNode<CreateDappMutation, CreateDappMutationVariables>;
export const GetDappDetailsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetDappDetails"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"dappId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"dapp"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"dappId"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"status"}},{"kind":"Field","name":{"kind":"Name","value":"rawStats"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"timestamp"}},{"kind":"Field","name":{"kind":"Name","value":"externalRef"}}]}},{"kind":"Field","name":{"kind":"Name","value":"stats"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"cumulative"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"total"}},{"kind":"Field","name":{"kind":"Name","value":"FheAdd"}},{"kind":"Field","name":{"kind":"Name","value":"FheSub"}},{"kind":"Field","name":{"kind":"Name","value":"FheMul"}},{"kind":"Field","name":{"kind":"Name","value":"FheDiv"}},{"kind":"Field","name":{"kind":"Name","value":"FheRem"}},{"kind":"Field","name":{"kind":"Name","value":"FheBitAnd"}},{"kind":"Field","name":{"kind":"Name","value":"FheBitOr"}},{"kind":"Field","name":{"kind":"Name","value":"FheBitXor"}},{"kind":"Field","name":{"kind":"Name","value":"FheShl"}},{"kind":"Field","name":{"kind":"Name","value":"FheShr"}},{"kind":"Field","name":{"kind":"Name","value":"FheRotl"}},{"kind":"Field","name":{"kind":"Name","value":"FheRotr"}},{"kind":"Field","name":{"kind":"Name","value":"FheEq"}},{"kind":"Field","name":{"kind":"Name","value":"FheEqBytes"}},{"kind":"Field","name":{"kind":"Name","value":"FheNe"}},{"kind":"Field","name":{"kind":"Name","value":"FheNeBytes"}},{"kind":"Field","name":{"kind":"Name","value":"FheGe"}},{"kind":"Field","name":{"kind":"Name","value":"FheGt"}},{"kind":"Field","name":{"kind":"Name","value":"FheLe"}},{"kind":"Field","name":{"kind":"Name","value":"FheLt"}},{"kind":"Field","name":{"kind":"Name","value":"FheMin"}},{"kind":"Field","name":{"kind":"Name","value":"FheMax"}},{"kind":"Field","name":{"kind":"Name","value":"FheNeg"}},{"kind":"Field","name":{"kind":"Name","value":"FheNot"}},{"kind":"Field","name":{"kind":"Name","value":"VerifyCiphertext"}},{"kind":"Field","name":{"kind":"Name","value":"Cast"}},{"kind":"Field","name":{"kind":"Name","value":"TrivialEncrypt"}},{"kind":"Field","name":{"kind":"Name","value":"TrivialEncryptBytes"}},{"kind":"Field","name":{"kind":"Name","value":"FheIfThenElse"}},{"kind":"Field","name":{"kind":"Name","value":"FheRand"}},{"kind":"Field","name":{"kind":"Name","value":"FheRandBounded"}}]}},{"kind":"Field","name":{"kind":"Name","value":"byDay"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"day"}},{"kind":"Field","name":{"kind":"Name","value":"total"}},{"kind":"Field","name":{"kind":"Name","value":"computation"}},{"kind":"Field","name":{"kind":"Name","value":"encryption"}}]}}]}}]}}]}}]} as unknown as DocumentNode<GetDappDetailsQuery, GetDappDetailsQueryVariables>;
export const DappUpdatedDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"subscription","name":{"kind":"Name","value":"DappUpdated"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"dappId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"dappUpdated"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"dappId"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"status"}},{"kind":"Field","name":{"kind":"Name","value":"rawStats"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"timestamp"}},{"kind":"Field","name":{"kind":"Name","value":"externalRef"}}]}},{"kind":"Field","name":{"kind":"Name","value":"stats"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"cumulative"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"total"}},{"kind":"Field","name":{"kind":"Name","value":"FheAdd"}},{"kind":"Field","name":{"kind":"Name","value":"FheSub"}},{"kind":"Field","name":{"kind":"Name","value":"FheMul"}},{"kind":"Field","name":{"kind":"Name","value":"FheDiv"}},{"kind":"Field","name":{"kind":"Name","value":"FheRem"}},{"kind":"Field","name":{"kind":"Name","value":"FheBitAnd"}},{"kind":"Field","name":{"kind":"Name","value":"FheBitOr"}},{"kind":"Field","name":{"kind":"Name","value":"FheBitXor"}},{"kind":"Field","name":{"kind":"Name","value":"FheShl"}},{"kind":"Field","name":{"kind":"Name","value":"FheShr"}},{"kind":"Field","name":{"kind":"Name","value":"FheRotl"}},{"kind":"Field","name":{"kind":"Name","value":"FheRotr"}},{"kind":"Field","name":{"kind":"Name","value":"FheEq"}},{"kind":"Field","name":{"kind":"Name","value":"FheEqBytes"}},{"kind":"Field","name":{"kind":"Name","value":"FheNe"}},{"kind":"Field","name":{"kind":"Name","value":"FheNeBytes"}},{"kind":"Field","name":{"kind":"Name","value":"FheGe"}},{"kind":"Field","name":{"kind":"Name","value":"FheGt"}},{"kind":"Field","name":{"kind":"Name","value":"FheLe"}},{"kind":"Field","name":{"kind":"Name","value":"FheLt"}},{"kind":"Field","name":{"kind":"Name","value":"FheMin"}},{"kind":"Field","name":{"kind":"Name","value":"FheMax"}},{"kind":"Field","name":{"kind":"Name","value":"FheNeg"}},{"kind":"Field","name":{"kind":"Name","value":"FheNot"}},{"kind":"Field","name":{"kind":"Name","value":"VerifyCiphertext"}},{"kind":"Field","name":{"kind":"Name","value":"Cast"}},{"kind":"Field","name":{"kind":"Name","value":"TrivialEncrypt"}},{"kind":"Field","name":{"kind":"Name","value":"TrivialEncryptBytes"}},{"kind":"Field","name":{"kind":"Name","value":"FheIfThenElse"}},{"kind":"Field","name":{"kind":"Name","value":"FheRand"}},{"kind":"Field","name":{"kind":"Name","value":"FheRandBounded"}}]}},{"kind":"Field","name":{"kind":"Name","value":"byDay"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"day"}},{"kind":"Field","name":{"kind":"Name","value":"total"}},{"kind":"Field","name":{"kind":"Name","value":"computation"}},{"kind":"Field","name":{"kind":"Name","value":"encryption"}}]}}]}}]}}]}}]} as unknown as DocumentNode<DappUpdatedSubscription, DappUpdatedSubscriptionVariables>;
export const SignInDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"SignIn"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"email"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"password"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"login"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"email"},"value":{"kind":"Variable","name":{"kind":"Name","value":"email"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"password"},"value":{"kind":"Variable","name":{"kind":"Name","value":"password"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"token"}},{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"email"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]}}]} as unknown as DocumentNode<SignInMutation, SignInMutationVariables>;
export const InvitationTokenDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"InvitationToken"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"token"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"invitation"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"token"},"value":{"kind":"Variable","name":{"kind":"Name","value":"token"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"expiresAt"}},{"kind":"Field","name":{"kind":"Name","value":"token"}},{"kind":"Field","name":{"kind":"Name","value":"email"}}]}}]}}]} as unknown as DocumentNode<InvitationTokenQuery, InvitationTokenQueryVariables>;
export const SignUpDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"SignUp"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"name"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"password"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"invitationToken"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"signup"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"name"},"value":{"kind":"Variable","name":{"kind":"Name","value":"name"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"password"},"value":{"kind":"Variable","name":{"kind":"Name","value":"password"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"invitationToken"},"value":{"kind":"Variable","name":{"kind":"Name","value":"invitationToken"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"token"}},{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"email"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]}}]} as unknown as DocumentNode<SignUpMutation, SignUpMutationVariables>;
export const MeDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"Me"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"me"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"email"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"teams"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"dapps"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"status"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}}]}}]}}]}}]}}]} as unknown as DocumentNode<MeQuery, MeQueryVariables>;