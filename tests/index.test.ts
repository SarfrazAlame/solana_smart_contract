import { expect, test } from 'bun:test'

test('counter does increase', async () => {
    expect(sum(1, 3)).toBe(4)
})

function sum(a: number, b: number) {
    return a + b;
}
