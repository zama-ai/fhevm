import { Injectable } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'

import * as tyk from './tyk.types.js'

@Injectable()
export class DeveloperPortalService {
  private readonly endpoint: string
  private readonly apiKey: string
  constructor(config: ConfigService) {
    this.endpoint = config.getOrThrow('tyk.endpoint')
    this.apiKey = config.getOrThrow('tyk.apiKey')
  }

  get headers() {
    return {
      'Content-Type': 'application/json',
      Authorization: this.apiKey,
    }
  }

  async sendJsonRequest<T>(
    method: 'POST' | 'PUT',
    path: string,
    body: unknown,
  ) {
    const response = await fetch(`${this.endpoint}/${path}`, {
      method,
      headers: this.headers,
      body: JSON.stringify(body),
    })

    if (!response.ok) {
      const text = await response.text() // Get the error response.
      throw new Error(`Failed to ${method} ${this.endpoint}/${path}: ${text}`)
    }

    return (await response.json()) as T
  }

  async postJson<T>(path: string, body: unknown) {
    return this.sendJsonRequest<T>('POST', path, body)
  }

  async putJson<T>(path: string, body: unknown) {
    return this.sendJsonRequest<T>('PUT', path, body)
  }

  async createOrganisation(organisationName: string) {
    return this.postJson<tyk.NewOrganizationResponse>('organisations', {
      Name: organisationName,
    })
  }

  async updateUser(userId: number, payload: Partial<Omit<tyk.User, 'ID'>>) {
    return this.putJson<tyk.User>(`users/${userId}`, payload)
  }
}
