<script lang="ts">
  import { onMount } from "svelte";

  let [kill, logEmitter] = createLogEmitter();

  async function* toUpperCase(prev: AsyncGenerator<string>) {
    for await (const line of prev) {
      yield line.toUpperCase();
    }
  }

  async function* filterOdd(prev: AsyncGenerator<string>) {
    for await (const line of prev) {
      const chunks = line.split(" ");
      const n = parseInt(chunks.at(-1) ?? "0");

      if (n % 2 == 0) {
        yield line;
      } else {
        continue;
      }
    }
  }

  async function* groupByTwo(prev: AsyncGenerator<string>) {
    let buffer = [];

    for await (const line of prev) {
      buffer.push(line);

      if (buffer.length >= 2) {
        yield buffer.join(" | ");

        buffer = [];
      }
    }
  }

  function createLogEmitter() {
    let killSignal = false;

    function kill() {
      killSignal = true;
    }

    async function* logEmitter(): AsyncGenerator<string> {
      let deferred: (event: MessageEvent) => void;

      const socket = new WebSocket(`ws://${window.location.hostname}:3000/ws`);
      const listener = function (event: MessageEvent) {
        deferred(event);
      };

      socket.addEventListener("message", listener);

      while (!killSignal) {
        const event = await new Promise<MessageEvent>(function (resolve) {
          deferred = (event: MessageEvent) => resolve(event);
        });

        yield* event.data.split("\n");
      }

      socket.removeEventListener("message", listener);
      socket.close();
    }

    return [kill, logEmitter] as const;
  }

  function transformPipeline(
    transformers: ((prev: AsyncGenerator<string>) => AsyncGenerator<string>)[]
  ): AsyncGenerator<string> {
    if (!transformers.length) {
      throw new Error("Transform list must contain at least 1 element");
    }

    let intermediate = transformers[0](
      undefined as unknown as AsyncGenerator<string>
    );

    for (const t of transformers.slice(1)) {
      intermediate = t(intermediate);
    }

    return intermediate;
  }

  let logLines: string[] = [];

  // Because the logEmitter is an infinite loop,
  // we boot a secondary mount effect whose sole job is to kill
  // the initial loop when this component unmounts
  onMount(() => kill);
  onMount(async function () {
    const pipeline = [
      logEmitter,
      filterOdd,
      toUpperCase,
      groupByTwo,
    ];

    for await (const line of transformPipeline(pipeline)) {
      logLines.unshift(line);
      logLines = logLines;
    }
  });
</script>

{#each logLines as line}
  <pre>{line}</pre>
{/each}
