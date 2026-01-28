/**
 * Simple EventEmitter for browser environment
 */

type EventHandler = (...args: any[]) => void

export class EventEmitter {
  private events: Map<string, EventHandler[]> = new Map()

  on(event: string, handler: EventHandler): this {
    if (!this.events.has(event)) {
      this.events.set(event, [])
    }
    this.events.get(event)!.push(handler)
    return this
  }

  off(event: string, handler: EventHandler): this {
    const handlers = this.events.get(event)
    if (handlers) {
      const index = handlers.indexOf(handler)
      if (index > -1) {
        handlers.splice(index, 1)
      }
    }
    return this
  }

  emit(event: string, ...args: any[]): boolean {
    const handlers = this.events.get(event)
    if (handlers) {
      handlers.forEach(handler => handler(...args))
      return true
    }
    return false
  }

  once(event: string, handler: EventHandler): this {
    const onceHandler = (...args: any[]) => {
      handler(...args)
      this.off(event, onceHandler)
    }
    return this.on(event, onceHandler)
  }

  removeAllListeners(event?: string): this {
    if (event) {
      this.events.delete(event)
    } else {
      this.events.clear()
    }
    return this
  }
}
