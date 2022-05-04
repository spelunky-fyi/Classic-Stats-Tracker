<script lang="ts">
  import store from "./stores";

  export let name,
    showKills = true,
    showDeaths = true;

  let collectedDeath = false;
  let collectedKill = false;
  let collected = false;

  // Default value for for an enemy
  let default_count = 1;
  let default_collect_state = true;

  if (store.ruleset === 2) {
    default_count = 0;
    default_collect_state = false;
  }

  store.stats.subscribe((value) => {
    // Treat undefined as collected
    collectedDeath = (value.enemy_deaths[name] ?? default_count) > 0;
    collectedKill = (value.enemy_kills[name] ?? default_count) > 0;

    if (!showKills) {
      collectedKill = default_collect_state;
    }

    if (!showDeaths) {
      collectedDeath = default_collect_state;
    }

    if (store.ruleset === 1 && collectedDeath && collectedKill) {
      collected = true;
    } else if (store.ruleset === 2 && (collectedDeath || collectedKill)) {
      collected = true;
    } else {
      collected = false;
    }
  });
</script>

<main>
  <img
    src="images/stats-icons/enemy_{name}.png"
    alt=""
    class={collected ? "collected" : ""}
  />
  {#if store.ruleset == 1}
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
