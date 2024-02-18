<script lang="ts">
  import { onMount } from "svelte";
  import { createLogEmitter, transformPipeline } from "./lib/logEmitter";
    import { filterEmpty } from "./transformers/filterEmpty";

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

  let logLines: string[] = [];

  // Because the logEmitter is an infinite loop,
  // we boot a secondary mount effect whose sole job is to kill
  // the initial loop when this component unmounts
  onMount(() => kill);
  onMount(async function () {
    const pipeline: any[] = [filterEmpty];

    for await (const line of transformPipeline(logEmitter, pipeline)) {
      logLines.unshift(line);
      logLines = logLines;
    }
  });
</script>

<nav id="navigation" />
<aside id="tab-select" />
<section id="logs">
  <ul>
    {#each logLines as line}
      <li>{line}</li>
    {/each}
  </ul>
</section>

<style>
  :global(body) {
    background: var(--eggshell);
    margin: var(--grid-2x);
    display: grid;
    grid-template-areas:
      "a a"
      "b c";
  }

  ul {
    list-style: none;
  }

  #navigation {
    grid-area: a;
  }

  #tab-select {

    grid-area: b;
  }

  #logs {
    grid-area: c;
  }

  li {
    margin-bottom: var(--grid-4s);
    background: var(--white);
    padding: var(--grid-2s);
  }
</style>
