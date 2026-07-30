import { act, create, type ReactTestRenderer } from 'react-test-renderer';
import { describe, expect, test } from 'vitest';

import { SettlementProgress } from './JourneyPrimitives';

describe('SettlementProgress', () => {
  test('shows event-driven progress without operator controls', async () => {
    let renderer!: ReactTestRenderer;
    await act(async () => {
      renderer = create(
        <SettlementProgress
          lifecycle={{ kind: 'awaiting-dispatch', remainingSlots: 3n }}
          action={null}
        />,
      );
    });

    expect(renderer.root.findAllByType('button')).toHaveLength(0);
    expect(renderer.root.findByType('progress').props.value).toBe(1);
    expect(renderer.root.findByProps({ role: 'status' }).children).toEqual(['Waiting for batch close']);
  });
});
