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
const documents = {
    "\n  query Me {\n    me {\n      id\n      email\n    }\n  }\n": types.MeDocument,
    "\n  mutation SignIn($email: String!, $password: String!) {\n    login(input: { email: $email, password: $password }) {\n      token\n      user {\n        id\n        email\n        teams {\n          id\n        }\n      }\n    }\n  }\n": types.SignInDocument,
    "\n  query InvitationToken($token: String!) {\n    invitation(token: $token) {\n      id\n      expiresAt\n      token\n      email\n    }\n  }\n": types.InvitationTokenDocument,
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
export function graphql(source: "\n  query Me {\n    me {\n      id\n      email\n    }\n  }\n"): (typeof documents)["\n  query Me {\n    me {\n      id\n      email\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation SignIn($email: String!, $password: String!) {\n    login(input: { email: $email, password: $password }) {\n      token\n      user {\n        id\n        email\n        teams {\n          id\n        }\n      }\n    }\n  }\n"): (typeof documents)["\n  mutation SignIn($email: String!, $password: String!) {\n    login(input: { email: $email, password: $password }) {\n      token\n      user {\n        id\n        email\n        teams {\n          id\n        }\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query InvitationToken($token: String!) {\n    invitation(token: $token) {\n      id\n      expiresAt\n      token\n      email\n    }\n  }\n"): (typeof documents)["\n  query InvitationToken($token: String!) {\n    invitation(token: $token) {\n      id\n      expiresAt\n      token\n      email\n    }\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;