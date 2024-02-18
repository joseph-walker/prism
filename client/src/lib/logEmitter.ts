/**
 * A log entry is the primitive structure that the transformation pipelines
 * deal in. It represents some item from the terminal stream as it passes through
 * the steps in a transformation pipeline.
 *
 * It initially is nothing but an empty struct with the `line` value unmodified
 * directly from stdout.
 *
 * Transformers can alter, add, remove, combine, split, and filter these log
 * entries however they want to, so long as they conform with the LogEntry contract.
 */
export type LogEntry = {
  /**
   * Group is a label that defines what tab the log entry will be displayed in.
   * If group is null, it shows up in the "Everything" tab.
   * */
  group: string | null;

  /**
   * Line is the text content of the log line.
   */
  line: string;
};

export function createLogEmitter() {
  let sigKill = false;

  function kill() {
    sigKill = true;
  }

  async function* logEmitter(): AsyncGenerator<string> {
    let deferred: (event: MessageEvent) => void;

    const socket = new WebSocket(`ws://${window.location.hostname}:3000/ws`);
    const listener = function(event: MessageEvent) {
      deferred(event);
    };

    socket.addEventListener("message", listener);

    while (!sigKill) {
      const event = await new Promise<MessageEvent>(function(resolve) {
        deferred = (event: MessageEvent) => resolve(event);
      });

      yield* event.data.split("\n");
    }

    socket.removeEventListener("message", listener);
    socket.close();
  }

  return [kill, logEmitter] as const;
}

export function transformPipeline(
  logEmitterInstance: () => AsyncGenerator<string>,
  transformers: ((prev: AsyncGenerator<string>) => AsyncGenerator<string>)[]
): AsyncGenerator<string> {
  let intermediate = logEmitterInstance();

  for (const t of transformers) {
    intermediate = t(intermediate);
  }

  return intermediate;
}
