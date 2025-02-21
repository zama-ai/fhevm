import { faker } from '@faker-js/faker'
import { describe, expect, test, vi } from 'vitest'
import { Task } from './task.js'

describe('Task', () => {
  describe('of', () => {
    test('should return the passed value', async () => {
      expect(Task.of('test').toPromise()).resolves.toBe('test')
    })
  })

  describe('reject', () => {
    test('should throw the passed error', async () => {
      expect(Task.reject('expected error').toPromise()).rejects.toThrowError(
        'expected error',
      )
    })
  })

  describe('map', () => {
    describe('when task succeded', () => {
      test('should call the function', async () => {
        const lorem = faker.lorem.words(5)
        await expect(
          Task.of(lorem)
            .map(s => s.length)
            .toPromise(),
        ).resolves.toBe(lorem.length)
      })
    })

    describe('when task fail', () => {
      test('should return the error', async () => {
        const error = faker.string.alpha(10)
        await expect(
          Task.reject<string, string>(error)
            .map(s => s.length)
            .toPromise(),
        ).rejects.toThrowError(error)
      })

      test('should not call the function', async () => {
        const fn = vi.fn()
        await expect(
          Task.reject<string, string>(faker.string.alpha(10))
            .map(fn)
            .toPromise(),
        ).rejects.toThrow()
        expect(fn).not.toBeCalled()
      })
    })
  })

  describe('mapError', () => {
    describe('when task succeded', () => {
      test('should return the value', async () => {
        const value = faker.lorem.word()
        await expect(
          Task.of(value)
            .mapError(() => faker.lorem.paragraph())
            .toPromise(),
        ).resolves.toBe(value)
      })

      test('should not call the function', async () => {
        const fn = vi.fn()
        await expect(Task.of(10).mapError(fn).toPromise()).resolves.all
        expect(fn).not.toBeCalled()
      })
    })

    describe('when task fail', () => {
      test('should call the function', async () => {
        const error = faker.lorem.word()
        await expect(
          Task.reject(error)
            .mapError(s => `computed: ${s.length}`)
            .toPromise(),
        ).rejects.toThrowError(`computed: ${error.length}`)
      })
    })
  })

  describe('chain', () => {
    describe('when task succeded', () => {
      test('should call the function', async () => {
        const value = faker.animal.bear()
        await expect(
          Task.of(value)
            .chain(s => Task.of(s.length))
            .toPromise(),
        ).resolves.toBe(value.length)
      })
    })

    describe('when task fail', () => {
      test('should return the error', async () => {
        const error = faker.lorem.words(3)
        await expect(
          Task.reject<string, string>(error)
            .chain(s => Task.of(s.length))
            .toPromise(),
        ).rejects.toThrowError(error)
      })

      test('should not call the function', async () => {
        const fn = vi.fn()
        await expect(
          Task.reject<string, string>(faker.lorem.words(3))
            .chain(fn)
            .toPromise(),
        ).rejects.toThrow()
        expect(fn).not.toBeCalled()
      })
    })
  })

  describe('or', () => {
    describe('when task succeded', () => {
      test('should return the value', async () => {
        const value = faker.animal.cat()
        await expect(Task.of(value).or('world').toPromise()).resolves.toBe(
          value,
        )
      })
    })

    describe('when task fail', () => {
      test('should return the value', async () => {
        const value = faker.animal.bird()
        await expect(
          Task.reject(faker.string.alpha()).or(value).toPromise(),
        ).resolves.toBe(value)
      })
    })
  })

  describe('orElse', () => {
    describe('when task succeded', () => {
      test('should return the value', async () => {
        const value = faker.animal.cat()
        await expect(
          Task.of(value)
            .orElse(() => faker.animal.dog())
            .toPromise(),
        ).resolves.toBe(value)
      })

      test('should not call the function', async () => {
        const fn = vi.fn()
        await expect(Task.of(10).orElse(fn).toPromise()).resolves.all
        expect(fn).not.toBeCalled()
      })
    })

    describe('when task fail', () => {
      test('should call the function', async () => {
        const value = faker.animal.cat()
        await expect(
          Task.reject(faker.string.alpha())
            .orElse(() => value)
            .toPromise(),
        ).resolves.toBe(value)
      })
    })
  })

  describe('orChain', () => {
    describe('when task succeded', () => {
      test('should return the value', async () => {
        const value = faker.animal.cat()
        await expect(
          Task.of(value)
            .orChain(() => Task.of(faker.animal.dog()))
            .toPromise(),
        ).resolves.toBe(value)
      })

      test('should not call the function', async () => {
        const fn = vi.fn()
        await expect(Task.of(10).orChain(fn).toPromise()).resolves.all
        expect(fn).not.toBeCalled()
      })
    })

    describe('when task fail', () => {
      test('should call the function', async () => {
        const value = faker.animal.cat()
        await expect(
          Task.reject(faker.string.alpha())
            .orChain(() => Task.of(value))
            .toPromise(),
        ).resolves.toBe(value)
      })
    })
  })

  describe('fork', () => {
    describe('when task succeded', () => {
      test('should call the resolve function', async () => {
        const resolve = vi.fn()
        const reject = vi.fn()
        const cat = faker.animal.cat()

        Task.of(cat).fork(resolve, reject)
        expect(resolve).toHaveBeenCalledExactlyOnceWith(cat)
      })

      test('should not call the reject function', async () => {
        const resolve = vi.fn()
        const reject = vi.fn()
        const cat = faker.animal.cat()

        Task.of(cat).fork(resolve, reject)
        expect(reject).not.toHaveBeenCalled()
      })
    })

    describe('when task fail', () => {
      test('should not call the resolve function', async () => {
        const resolve = vi.fn()
        const reject = vi.fn()
        const cat = faker.animal.cat()

        Task.reject(cat).fork(resolve, reject)
        expect(resolve).not.toHaveBeenCalled()
      })

      test('should call the reject function', async () => {
        const resolve = vi.fn()
        const reject = vi.fn()
        const cat = faker.animal.cat()

        Task.reject(cat).fork(resolve, reject)
        expect(reject).toHaveBeenCalledExactlyOnceWith(cat)
      })
    })
  })

  describe('match', () => {
    describe('when task succeded', async () => {
      test('should call the ok function', async () => {
        const ok = vi.fn()
        const fail = vi.fn()
        const dog = faker.animal.dog()

        await Task.of(dog).match({ ok, fail }).toPromise()
        expect(ok).toHaveBeenCalledExactlyOnceWith(dog)
      })

      test('should not call the fail function', async () => {
        const ok = vi.fn()
        const fail = vi.fn()
        const dog = faker.animal.dog()

        await Task.of(dog).match({ ok, fail }).toPromise()
        expect(fail).not.toHaveBeenCalled()
      })
    })

    describe('when task fail', () => {
      test('should not call the ok function', async () => {
        const ok = vi.fn()
        const fail = vi.fn()
        const dog = faker.animal.dog()

        await Task.reject(dog).match({ ok, fail }).toPromise()
        expect(ok).not.toHaveBeenCalled()
      })

      test('should call the fail function', async () => {
        const ok = vi.fn()
        const fail = vi.fn()
        const dog = faker.animal.dog()

        await Task.reject(dog).match({ ok, fail }).toPromise()
        expect(fail).toHaveBeenCalledExactlyOnceWith(dog)
      })
    })
  })

  describe('all', () => {
    describe('when all task succeded', () => {
      test('should return the tuple of results', async () => {
        await expect(
          Task.all([Task.of('a'), Task.of(1)]).toPromise(),
        ).resolves.toEqual(['a', 1])
      })

      test('should return the array of results', async () => {
        await expect(
          Task.all(
            new Array(5).fill('').map((_, idx) => Task.of(idx + 1)),
          ).toPromise(),
        ).resolves.toEqual([1, 2, 3, 4, 5])
      })
    })

    describe('when at least a task fails', () => {
      test('should return the first failing task', async () => {
        await expect(
          Task.all([
            Task.of('test'),
            new Task((_, reject) => {
              setTimeout(() => reject('delayed error'), 10)
            }),
            Task.reject('fastest error'),
          ]).toPromise(),
        ).rejects.toThrowError('delayed error')
      })
    })
  })

  describe('timeout', () => {
    test('should fail if seconds is negative', async () => {
      await expect(Task.timeout(-1).toPromise()).rejects.toThrowError(
        expect.objectContaining({
          _tag: 'ValidationError',
        }),
      )
    })

    test('should timeout', async () => {
      await expect(Task.timeout(0).toPromise()).rejects.toThrowError(
        expect.objectContaining({
          _tag: 'TimeoutError',
        }),
      )
    })
  })

  describe('race', () => {
    test('should return the fastest to solve', async () => {
      await expect(
        Task.race([
          new Task(resolve => {
            setTimeout(() => resolve('slowest'), 10)
          }),
          new Task(resolve => {
            setTimeout(() => resolve('fastest'), 5)
          }),
        ]).toPromise(),
      ).resolves.toBe('fastest')
    })
  })
})
