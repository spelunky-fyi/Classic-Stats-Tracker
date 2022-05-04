<script lang="ts">
  import store from "./stores";

  export let name,
    showKills = true,
    showDeaths = true;

  let collectedDeath = false;
  let collectedKill = false;
  let collectedBoth = false;

  store.stats.subscribe((value) => {
    // Treat undefined as collected
    collectedDeath = (value.enemy_deaths[name] ?? 1) > 0;
    collectedKill = (value.enemy_kills[name] ?? 1) > 0;

    if (!showKills) {
      collectedKill = true;
    }

    if (!showDeaths) {
      collectedDeath = true;
    }

    if (collectedDeath && collectedKill) {
      collectedBoth = true;
    } else {
      collectedBoth = false;
    }
  });
</script>

<main>
  <img
    src="images/stats-icons/enemy_{name}.png"
    alt=""
    class={collectedBoth ? "collected" : ""}
  />
  {#if showDeaths}
    <img
      src="images/stats-icons/skull.png"
      alt=""
      class={collectedDeath ? "collected" : ""}
    />
  {:else}
    <img src="images/stats-icons/empty.png" alt="" />
  {/if}

  {#if showKills}
    <img
      src="images/stats-icons/whip.png"
      alt=""
      class={collectedKill ? "collected" : ""}
    />
  {:else}
    <img src="images/stats-icons/empty.png" alt="" />
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
