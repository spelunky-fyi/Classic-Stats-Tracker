<script lang="ts">
  import store from "./stores";

  export let name,
    collection = null;

  let collected = false;

  store.stats.subscribe((value) => {
    if (collection === null) {
      collected = value[name] > 0;
    } else {
      collected = value[collection][name] > 0;
    }
  });
</script>

<main>
  {#if collection === null}
    <img
      src="images/stats-icons/{name}.png"
      class={collected ? "collected" : ""}
      alt=""
    />
  {/if}
  {#if collection !== null}
    <img
      src="images/stats-icons/{collection}_{name}.png"
      class={collected ? "collected" : ""}
      alt=""
    />
  {/if}
</main>

<style>
  main {
    display: inline;
  }
  img.collected {
    opacity: 0.2;
  }
</style>
