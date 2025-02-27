import request from 'supertest'

export type GraphQlResponse<T> =
  | {
      success: true
      data: T
    }
  | {
      success: false
      errors: ReadonlyArray<{ message: string }>
    }

export class GraphQl<
  Data extends object = object,
  Variables extends object | void = void,
> {
  #request: request.Request

  private constructor(server: any) {
    this.#request = request(server)
      .post('/graphql')
      .set('Content-Type', 'application/json')
      .set('Accept', 'application/json')
  }

  auth(token: string) {
    this.#request.set('Authorization', `Bearer ${token}`)
    return this
  }

  mutate(mutation: string, variables?: Variables) {
    this.#request.send({ query: mutation, variables })
    return this
  }

  query(mutation: string, variables?: Variables) {
    this.#request.send({ query: mutation, variables })
    return this
  }

  async exec(key: keyof Data): Promise<GraphQlResponse<Data[typeof key]>> {
    const data = await this.#request

    return data.body.data
      ? { success: true, data: (data.body.data as Data)[key] }
      : { success: false, errors: data.body.errors! }
  }

  static request<Data extends object, Variables extends object | void = void>(
    server: any,
  ) {
    return new GraphQl<Data, Variables>(server)
  }
}
