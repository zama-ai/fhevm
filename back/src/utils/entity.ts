export abstract class Entity<Props> {
  #props: Props

  protected constructor(props: Props) {
    this.#props = structuredClone(props)
    Object.freeze(this.#props)
  }

  protected get<Key extends keyof Props>(key: Key): Props[Key] {
    return this.#props[key]
  }

  toJSON(): Props {
    return structuredClone(this.#props)
  }

  toString() {
    return JSON.stringify(this.#props)
  }
}
