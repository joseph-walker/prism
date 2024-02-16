<script lang="ts">
  let selectedCategory = "misc";
  let buffer: Record<string, string[]> = {};

  const socket = new WebSocket(`ws://${window.location.hostname}:3000/ws`);
  const logExtractor = /^([\w@\/\-:]+): (.*)$/g;

  socket.addEventListener("message", function (event) {
    const lines = event.data.split("\n");

    for (const line of lines) {
      const matches = [...line.matchAll(logExtractor)];

      let bucket;
      let content;

      if (matches.length) {
        bucket = matches[0][1];
        content = matches[0][2];
      } else {
        bucket = "misc";
        content = line;
      }

      if (bucket in buffer) {
        buffer[bucket].push(content);
      } else {
        buffer[bucket] = [content];
      }
    }

    buffer = buffer;
  });

  $: categories = Object.keys(buffer);
  $: logsToShow = buffer[selectedCategory] ?? [];
</script>

<ul>
  {#each categories as category}
    <li>
      <button on:click={() => (selectedCategory = category)}>
        {category} [{buffer[category].length}]
      </button>
    </li>
  {/each}
</ul>

<ul>
  {#each logsToShow as log}
    <li>{log}</li>
  {/each}
</ul>
