# users teams & dapps

## users

Users cans login. They are members of a team (with only one member)

### how to create a user

First, you need to create an invitation. This can only be done on graphql. You can use a graphql client such as Altair or Bruno or use teh Playground on http://localhost:3000/graphql (disabled in prod).

The secret is available in `/apps/back/.env`

```gql
mutation CreateInvitation {
  createInvitation(
    input: {
      email: "anyaddress@example.com"
      secret: "InvitationSecretPassPhrase"
    }
  ) {
    token
  }
}
```

the response contains an invitation token

```json
{
  "data": {
    "createInvitation": {
      "token": "8ca08ddd-725b-4003-b8b8-66a379b1a3e2"
    }
  }
}
```

Use this invitation token to get to the subscription URL
`http://localhost:4174/subscribe/8ca08ddd-725b-4003-b8b8-66a379b1a3e2`

## teams

Teams are aggregate of users. They own dapps.
Each user belongs to a team with only one member.

Tams are automatically created upon user creation.

## dapps

A dapp is connected to a chain and a smart contract address.
