export type Timing = {
  description: string;
  time: number;
};

export const displayTimings = (timings: Timing[]) => {
  timings.forEach((t: Timing) => {
    let time = `${t.time}ms`;
    if (t.time >= 100) {
      time = `${Math.round(t.time / 100) / 10}s`;
    }
    console.log(`${t.description} in ${time}`);
  });
};
