/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 * Learn more about it here: https://the-guild.dev/graphql/codegen/plugins/presets/preset-client#reducing-bundle-size
 */
type Documents = {
    "\n  query ValidateAddress($chainId: String!, $address: String!) {\n    validateAddress(input: { chainId: $chainId, address: $address }) {\n      check\n      message\n    }\n  }\n": typeof types.ValidateAddressDocument,
    "\n  mutation CreateDapp($teamId: String!, $name: String!) {\n    createDapp(input: { teamId: $teamId, name: $name }) {\n      id\n      name\n      address\n      status\n    }\n  }\n": typeof types.CreateDappDocument,
    "\n  query GetDapp($dappId: ID!) {\n    dapp(input: { id: $dappId }) {\n      id\n      name\n      status\n    }\n  }\n": typeof types.GetDappDocument,
    "\n  mutation SetDappAddress($id: ID!, $address: String!) {\n    updateDapp(input: { id: $id, address: $address }) {\n      id\n      name\n      address\n      status\n    }\n  }\n": typeof types.SetDappAddressDocument,
    "\n  mutation DeployDapp($applicationId: String!) {\n    deployDapp(input: { dappId: $applicationId }) {\n      id\n      name\n      address\n      status\n    }\n  }\n": typeof types.DeployDappDocument,
    "\n  query GetDappDetails($dappId: ID!) {\n    dapp(input: { id: $dappId }) {\n      id\n      name\n      status\n      stats {\n        id\n        name\n        timestamp\n        externalRef\n      }\n    }\n  }\n": typeof types.GetDappDetailsDocument,
    "\n  subscription DappUpdated($dappId: ID!) {\n    dappUpdated(input: { id: $dappId }) {\n      id\n      name\n      status\n      stats {\n        id\n        name\n        timestamp\n        externalRef\n      }\n    }\n  }\n": typeof types.DappUpdatedDocument,
    "\n  query Preferences {\n    me {\n      id\n      email\n      name\n      teams {\n        id\n        name\n      }\n    }\n  }\n": typeof types.PreferencesDocument,
    "\n  mutation ChangeUserName($id: ID!, $name: String!) {\n    updateUser(input: { id: $id, name: $name }) {\n      id\n      name\n    }\n  }\n": typeof types.ChangeUserNameDocument,
    "\n  mutation SignIn($email: String!, $password: String!) {\n    login(input: { email: $email, password: $password }) {\n      token\n      user {\n        id\n        email\n        name\n      }\n    }\n  }\n": typeof types.SignInDocument,
    "\n  query InvitationToken($token: String!) {\n    invitation(token: $token) {\n      id\n      expiresAt\n      token\n      email\n    }\n  }\n": typeof types.InvitationTokenDocument,
    "\n  mutation SignUp(\n    $name: String!\n    $password: String!\n    $invitationToken: String!\n  ) {\n    signup(\n      input: {\n        name: $name\n        password: $password\n        invitationToken: $invitationToken\n      }\n    ) {\n      token\n      user {\n        id\n        email\n        name\n      }\n    }\n  }\n": typeof types.SignUpDocument,
    "\n  query Me {\n    me {\n      id\n      email\n      name\n      teams {\n        id\n        name\n        dapps {\n          id\n          name\n          status\n          createdAt\n        }\n      }\n    }\n  }\n": typeof types.MeDocument,
};
const documents: Documents = {
    "\n  query ValidateAddress($chainId: String!, $address: String!) {\n    validateAddress(input: { chainId: $chainId, address: $address }) {\n      check\n      message\n    }\n  }\n": types.ValidateAddressDocument,
    "\n  mutation CreateDapp($teamId: String!, $name: String!) {\n    createDapp(input: { teamId: $teamId, name: $name }) {\n      id\n      name\n      address\n      status\n    }\n  }\n": types.CreateDappDocument,
    "\n  query GetDapp($dappId: ID!) {\n    dapp(input: { id: $dappId }) {\n      id\n      name\n      status\n    }\n  }\n": types.GetDappDocument,
    "\n  mutation SetDappAddress($id: ID!, $address: String!) {\n    updateDapp(input: { id: $id, address: $address }) {\n      id\n      name\n      address\n      status\n    }\n  }\n": types.SetDappAddressDocument,
    "\n  mutation DeployDapp($applicationId: String!) {\n    deployDapp(input: { dappId: $applicationId }) {\n      id\n      name\n      address\n      status\n    }\n  }\n": types.DeployDappDocument,
    "\n  query GetDappDetails($dappId: ID!) {\n    dapp(input: { id: $dappId }) {\n      id\n      name\n      status\n      stats {\n        id\n        name\n        timestamp\n        externalRef\n      }\n    }\n  }\n": types.GetDappDetailsDocument,
    "\n  subscription DappUpdated($dappId: ID!) {\n    dappUpdated(input: { id: $dappId }) {\n      id\n      name\n      status\n      stats {\n        id\n        name\n        timestamp\n        externalRef\n      }\n    }\n  }\n": types.DappUpdatedDocument,
    "\n  query Preferences {\n    me {\n      id\n      email\n      name\n      teams {\n        id\n        name\n      }\n    }\n  }\n": types.PreferencesDocument,
    "\n  mutation ChangeUserName($id: ID!, $name: String!) {\n    updateUser(input: { id: $id, name: $name }) {\n      id\n      name\n    }\n  }\n": types.ChangeUserNameDocument,
    "\n  mutation SignIn($email: String!, $password: String!) {\n    login(input: { email: $email, password: $password }) {\n      token\n      user {\n        id\n        email\n        name\n      }\n    }\n  }\n": types.SignInDocument,
    "\n  query InvitationToken($token: String!) {\n    invitation(token: $token) {\n      id\n      expiresAt\n      token\n      email\n    }\n  }\n": types.InvitationTokenDocument,
    "\n  mutation SignUp(\n    $name: String!\n    $password: String!\n    $invitationToken: String!\n  ) {\n    signup(\n      input: {\n        name: $name\n        password: $password\n        invitationToken: $invitationToken\n      }\n    ) {\n      token\n      user {\n        id\n        email\n        name\n      }\n    }\n  }\n": types.SignUpDocument,
    "\n  query Me {\n    me {\n      id\n      email\n      name\n      teams {\n        id\n        name\n        dapps {\n          id\n          name\n          status\n          createdAt\n        }\n      }\n    }\n  }\n": types.MeDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query ValidateAddress($chainId: String!, $address: String!) {\n    validateAddress(input: { chainId: $chainId, address: $address }) {\n      check\n      message\n    }\n  }\n"): (typeof documents)["\n  query ValidateAddress($chainId: String!, $address: String!) {\n    validateAddress(input: { chainId: $chainId, address: $address }) {\n      check\n      message\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation CreateDapp($teamId: String!, $name: String!) {\n    createDapp(input: { teamId: $teamId, name: $name }) {\n      id\n      name\n      address\n      status\n    }\n  }\n"): (typeof documents)["\n  mutation CreateDapp($teamId: String!, $name: String!) {\n    createDapp(input: { teamId: $teamId, name: $name }) {\n      id\n      name\n      address\n      status\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query GetDapp($dappId: ID!) {\n    dapp(input: { id: $dappId }) {\n      id\n      name\n      status\n    }\n  }\n"): (typeof documents)["\n  query GetDapp($dappId: ID!) {\n    dapp(input: { id: $dappId }) {\n      id\n      name\n      status\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation SetDappAddress($id: ID!, $address: String!) {\n    updateDapp(input: { id: $id, address: $address }) {\n      id\n      name\n      address\n      status\n    }\n  }\n"): (typeof documents)["\n  mutation SetDappAddress($id: ID!, $address: String!) {\n    updateDapp(input: { id: $id, address: $address }) {\n      id\n      name\n      address\n      status\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation DeployDapp($applicationId: String!) {\n    deployDapp(input: { dappId: $applicationId }) {\n      id\n      name\n      address\n      status\n    }\n  }\n"): (typeof documents)["\n  mutation DeployDapp($applicationId: String!) {\n    deployDapp(input: { dappId: $applicationId }) {\n      id\n      name\n      address\n      status\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query GetDappDetails($dappId: ID!) {\n    dapp(input: { id: $dappId }) {\n      id\n      name\n      status\n      stats {\n        id\n        name\n        timestamp\n        externalRef\n      }\n    }\n  }\n"): (typeof documents)["\n  query GetDappDetails($dappId: ID!) {\n    dapp(input: { id: $dappId }) {\n      id\n      name\n      status\n      stats {\n        id\n        name\n        timestamp\n        externalRef\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  subscription DappUpdated($dappId: ID!) {\n    dappUpdated(input: { id: $dappId }) {\n      id\n      name\n      status\n      stats {\n        id\n        name\n        timestamp\n        externalRef\n      }\n    }\n  }\n"): (typeof documents)["\n  subscription DappUpdated($dappId: ID!) {\n    dappUpdated(input: { id: $dappId }) {\n      id\n      name\n      status\n      stats {\n        id\n        name\n        timestamp\n        externalRef\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query Preferences {\n    me {\n      id\n      email\n      name\n      teams {\n        id\n        name\n      }\n    }\n  }\n"): (typeof documents)["\n  query Preferences {\n    me {\n      id\n      email\n      name\n      teams {\n        id\n        name\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation ChangeUserName($id: ID!, $name: String!) {\n    updateUser(input: { id: $id, name: $name }) {\n      id\n      name\n    }\n  }\n"): (typeof documents)["\n  mutation ChangeUserName($id: ID!, $name: String!) {\n    updateUser(input: { id: $id, name: $name }) {\n      id\n      name\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation SignIn($email: String!, $password: String!) {\n    login(input: { email: $email, password: $password }) {\n      token\n      user {\n        id\n        email\n        name\n      }\n    }\n  }\n"): (typeof documents)["\n  mutation SignIn($email: String!, $password: String!) {\n    login(input: { email: $email, password: $password }) {\n      token\n      user {\n        id\n        email\n        name\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query InvitationToken($token: String!) {\n    invitation(token: $token) {\n      id\n      expiresAt\n      token\n      email\n    }\n  }\n"): (typeof documents)["\n  query InvitationToken($token: String!) {\n    invitation(token: $token) {\n      id\n      expiresAt\n      token\n      email\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation SignUp(\n    $name: String!\n    $password: String!\n    $invitationToken: String!\n  ) {\n    signup(\n      input: {\n        name: $name\n        password: $password\n        invitationToken: $invitationToken\n      }\n    ) {\n      token\n      user {\n        id\n        email\n        name\n      }\n    }\n  }\n"): (typeof documents)["\n  mutation SignUp(\n    $name: String!\n    $password: String!\n    $invitationToken: String!\n  ) {\n    signup(\n      input: {\n        name: $name\n        password: $password\n        invitationToken: $invitationToken\n      }\n    ) {\n      token\n      user {\n        id\n        email\n        name\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query Me {\n    me {\n      id\n      email\n      name\n      teams {\n        id\n        name\n        dapps {\n          id\n          name\n          status\n          createdAt\n        }\n      }\n    }\n  }\n"): (typeof documents)["\n  query Me {\n    me {\n      id\n      email\n      name\n      teams {\n        id\n        name\n        dapps {\n          id\n          name\n          status\n          createdAt\n        }\n      }\n    }\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;