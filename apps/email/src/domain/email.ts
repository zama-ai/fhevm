export interface Email {
  from: string
  to: string
  subject: string
  data: {
    context: Record<string, any>
    template: string
  }
}
